
use stellar_base::network::Network;
use jsonwebtoken::{encode, Header, EncodingKey};
use thiserror::Error;
use stellar_sdk::Keypair;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{engine::general_purpose, Engine as _};
use stellar_base::transaction::TransactionEnvelope;
use stellar_base::xdr::XDRDeserialize;



#[derive(Error, Debug)]
pub enum AuthError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JWT generation failed: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("XDR parsing failed: {0}")]
    XdrError(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone)]
pub struct StellarAuth {
    client: Client,
    domain: String,
    network: Network,
    jwt_secret: String,
    web_auth_endpoint: OnceCell<String>,
    signing_key: OnceCell<String>,
}

#[derive(Deserialize)]
struct StellarToml {
    #[serde(rename = "WEB_AUTH_ENDPOINT")]
    web_auth_endpoint: String,
    #[serde(rename = "SIGNING_KEY")]
    signing_key: String,
}

impl StellarAuth {
    pub fn new(domain: String, network: Network, jwt_secret: String) -> Self {
        Self {
            client: Client::new(),
            domain,
            network,
            jwt_secret,//remove jwt secret
            web_auth_endpoint: OnceCell::new(),
            signing_key: OnceCell::new(),
        }
    }

    pub async fn init(&self) -> Result<(), AuthError> {
        let toml = self.fetch_stellar_toml().await?;
        self.web_auth_endpoint.set(toml.web_auth_endpoint)
            .map_err(|_| AuthError::ConfigError("WEB_AUTH_ENDPOINT already set".into()))?;
        self.signing_key.set(toml.signing_key)
            .map_err(|_| AuthError::ConfigError("SIGNING_KEY already set".into()))?;
        Ok(())
    }

    pub async fn authenticate(&self, account_id: &str, keypair: &Keypair) -> Result<String, AuthError> {
        let challenge = self.get_challenge(account_id).await?;
        let signature = self.sign_challenge(&challenge, keypair)?;
        self.get_jwt_token(account_id, &challenge, &signature).await
    }

    async fn fetch_stellar_toml(&self) -> Result<StellarToml, AuthError> {
        let url = format!("https://{}/.well-known/stellar.toml", self.domain);
        let toml_str = self.client.get(&url).send().await?.text().await?;
        let toml: StellarToml = toml::from_str(&toml_str)
            .map_err(|e| AuthError::ConfigError(format!("TOML parsing failed: {}", e)))?;
        Ok(toml)
    }

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


    
    fn sign_challenge(&self, challenge: &str, keypair: &Keypair) -> Result<String, AuthError> {
        // 1. Parse the XDR
        let tx_envelope = TransactionEnvelope::from_xdr_base64(challenge)
            .map_err(|e| AuthError::XdrError(format!("XDR parsing failed: {}", e)))?;
        
        // 2. Get raw hash bytes
        let tx_hash = tx_envelope
            .hash(&self.network)
            .map_err(|e| AuthError::XdrError(format!("Hash computation failed: {}", e)))?;
        
        // 3. Convert hash to bytes slice explicitly
        let hash_bytes: &[u8] = tx_hash.as_slice();
        
        // 4. Sign and encode
        let signature = keypair.sign(hash_bytes).unwrap();
        Ok(general_purpose::STANDARD.encode(signature))
    }




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
                "account_id": account_id,
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





// use reqwest;
// use toml::Value;
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum Sep10Error {
//     #[error("Failed to fetch stellar.toml: {0}")]
//     FetchError(String),
//     #[error("Invalid stellar.toml format: {0}")]
//     ParseError(String),
//     #[error("Missing required field: {0}")]
//     MissingField(String),
// }

// pub async fn fetch_stellar_toml(domain: &str) -> Result<Value, Sep10Error> {
//     let url = format!("https://{}/.well-known/stellar.toml", domain);
//     let response = reqwest::get(&url)
//         .await
//         .map_err(|e| Sep10Error::FetchError(e.to_string()))?;
    
//     let text = response.text()
//         .await
//         .map_err(|e| Sep10Error::FetchError(e.to_string()))?;
    
//     text.parse::<Value>()
//         .map_err(|e| Sep10Error::ParseError(e.to_string()))
// }

// pub fn get_web_auth_endpoint(toml: &Value) -> Result<String, Sep10Error> {
//     toml.get("WEB_AUTH_ENDPOINT")
//         .and_then(|v| v.as_str())
//         .map(|s| s.to_string())
//         .ok_or_else(|| Sep10Error::MissingField("WEB_AUTH_ENDPOINT".to_string()))
// }

// pub fn get_signing_key(toml: &Value) -> Result<String, Sep10Error> {
//     toml.get("SIGNING_KEY")
//         .and_then(|v| v.as_str())
//         .map(|s| s.to_string())
//         .ok_or_else(|| Sep10Error::MissingField("SIGNING_KEY".to_string()))
// }