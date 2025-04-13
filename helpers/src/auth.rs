use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use ed25519_dalek::{Keypair, Signer};
use once_cell::sync::OnceCell;
use stellar_xdr::{TransactionEnvelope, ReadXdr, NetworkId};
use jsonwebtoken::{encode, Header, EncodingKey};
use thiserror::Error;
use tokio::sync::OnceCell;
use controllers::{
    api::api::{failure, success, ApiResponse},
};

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JWT generation failed: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("XDR parsing failed: {0}")]
    XdrError(#[from] stellar_xdr::Error),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone)]
pub struct StellarAuth {
    client: Client,
    domain: String,
    network: NetworkId,
    jwt_secret: String,
    web_auth_endpoint: OnceCell<String>,
    signing_key: OnceCell<String>,
}

impl StellarAuth {
    /// Create a new StellarAuth instance
    pub fn new(domain: String, network: NetworkId, jwt_secret: String) -> Self {
        StellarAuth {
            client: Client::new(),
            domain,
            network,
            jwt_secret,
            web_auth_endpoint: OnceCell::new(),
            signing_key: OnceCell::new(),
        }
    }

    /// Initialize the auth service by fetching the stellar.toml
    pub async fn init(&self) -> Result<(), AuthError> {
        let toml = self.fetch_stellar_toml().await?;
        self.web_auth_endpoint.set(toml.web_auth_endpoint)
            .map_err(|_| AuthError::ConfigError("WEB_AUTH_ENDPOINT already set".into()))?;
        self.signing_key.set(toml.signing_key)
            .map_err(|_| AuthError::ConfigError("SIGNING_KEY already set".into()))?;
        Ok(())
    }

    /// Authenticate a user with their Stellar account
    pub async fn authenticate(&self, account_id: &str, keypair: &Keypair) -> Result<String, AuthError> {
        let challenge = self.get_challenge(account_id).await?;
        let signature = self.sign_challenge(&challenge, keypair)?;
        self.get_jwt_token(account_id, &challenge, &signature).await
    }

    /// Fetch the Stellar.toml file
    async fn fetch_stellar_toml(&self) -> Result<StellarToml, AuthError> {
        #[derive(Deserialize)]
        struct StellarToml {
            #[serde(rename = "WEB_AUTH_ENDPOINT")]
            web_auth_endpoint: String,
            #[serde(rename = "SIGNING_KEY")]
            signing_key: String,
        }

        let url = format!("https://{}/.well-known/stellar.toml", self.domain);
        let toml_str = self.client.get(&url).send().await?.text().await?;
        let toml: StellarToml = toml::from_str(&toml_str)
            .map_err(|e| AuthError::ConfigError(format!("TOML parsing failed: {}", e)))?;
        Ok(toml)
    }

    /// Get a challenge transaction from the auth endpoint
    async fn get_challenge(&self, account_id: &str) -> Result<String, AuthError> {
        let endpoint = self.web_auth_endpoint.get()
            .ok_or(AuthError::ConfigError("WEB_AUTH_ENDPOINT not initialized".into()))?;

        let response = self.client
            .get(endpoint)
            .query(&[("account", account_id)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AuthError::AuthFailed(format!(
                "Challenge request failed with status: {}",
                response.status()
            )));
        }

        response.text().await.map_err(Into::into)
    }

    /// Sign the challenge transaction
    fn sign_challenge(&self, challenge: &str, keypair: &Keypair) -> Result<String, AuthError> {
        let tx_envelope = TransactionEnvelope::from_xdr_base64(challenge)?;
        
        match tx_envelope {
            TransactionEnvelope::Tx(env) => {
                let tx_hash = env.tx_hash(self.network)?;
                let signature = keypair.sign(&tx_hash.value);
                Ok(signature.to_base64())
            }
            _ => Err(AuthError::AuthFailed("Invalid challenge transaction".into())),
        }
    }

    /// Exchange the signed challenge for a JWT token
    async fn get_jwt_token(
        &self,
        account_id: &str,
        challenge: &str,
        signature: &str,
    ) -> Result<String, AuthError> {
        let endpoint = self.web_auth_endpoint.get()
            .ok_or(AuthError::ConfigError("WEB_AUTH_ENDPOINT not initialized".into()))?;

        let response = self.client
            .post(endpoint)
            .json(&serde_json::json!({
                "transaction": challenge,
                "signature": signature,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AuthError::AuthFailed(format!(
                "Token request failed with status: {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct TokenResponse {
            token: String,
        }
        
        let token: TokenResponse = response.json().await?;
        Ok(token.token)
    }

    /// Generate a service JWT for authenticated requests
    pub fn generate_service_jwt(&self, account_id: &str, expiration_secs: u64) -> Result<String, AuthError> {
        #[derive(Serialize)]
        struct Claims {
            iss: String,
            sub: String,
            iat: u64,
            exp: u64,
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = now + expiration_secs;

        let claims = Claims {
            iss: self.domain.clone(),
            sub: account_id.to_string(),
            iat: now,
            exp,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SecretKey;
    use mockito::{mock, Server as MockServer};

    #[tokio::test]
    async fn test_auth_flow() {
        let mut mock_server = MockServer::new();
        
        // Mock stellar.toml endpoint
        let toml_mock = mock("GET", "/.well-known/stellar.toml")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(r#"
                WEB_AUTH_ENDPOINT = "https://auth.example.com"
                SIGNING_KEY = "GDK..."
            "#)
            .create();

        // Mock challenge endpoint
        let challenge_mock = mock("GET", "/?account=GBRX...")
            .with_status(200)
            .with_body("test_challenge_transaction")
            .create();

        // Mock token endpoint
        let token_mock = mock("POST", "/")
            .with_status(200)
            .with_body(r#"{"token": "test.jwt.token"}"#)
            .create();

        let auth = StellarAuth::new(
            mock_server.host(),
            NetworkId::Testnet,
            "test_secret".into()
        );

        auth.init().await.unwrap();

        let secret_key = SecretKey::from_bytes(&[0u8; 32]).unwrap();
        let keypair = Keypair::from(&secret_key);
        let account_id = "GBRX...";

        let token = auth.authenticate(account_id, &keypair).await.unwrap();
        assert_eq!(token, "test.jwt.token");

        toml_mock.assert();
        challenge_mock.assert();
        token_mock.assert();
    }
}