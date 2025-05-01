use stellar_base::network::Network;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use thiserror::Error;
use stellar_base::KeyPair;
use stellar_sdk::Keypair;
use stellar_base::transaction::TransactionEnvelope;
use once_cell::sync::Lazy;
use std::sync::Arc;
use chrono::Utc;
use once_cell::sync::OnceCell;
use stellar_base::Operation;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use stellar_base::xdr::{XDRDeserialize, XDRSerialize};


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
    #[error("Invalid challenge structure: {0}")]
    ChallengeError(String),
    #[error("Signature verification failed")]
    SignatureError,
}

#[derive(Debug, Clone)]
pub struct StellarAuth {
    client: Client,
    home_domain: String,
    network: Network,
    jwt_secret: String,
    web_auth_endpoint: OnceCell<String>,
    signing_key: OnceCell<String>,
    challenge_timeout: u64,
}


#[derive(Debug, Deserialize)]
struct Claims {
    iss: String,   
    sub: String,        
    exp: u64,       
}

#[derive(Deserialize)]
struct StellarToml {
    #[serde(rename = "WEB_AUTH_ENDPOINT")]
    web_auth_endpoint: String,
    #[serde(rename = "SIGNING_KEY")]
    signing_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeResponse {
    transaction: String,
    network_passphrase: String,
}

#[derive(Debug, Serialize)]
struct TokenRequest {
    transaction: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    token: String,
}

static STELLAR_AUTH: Lazy<Arc<StellarAuth>> = Lazy::new(|| {
    Arc::new(StellarAuth::from_env().expect("Failed to initialize StellarAuth from environment"))
});

impl StellarAuth {


    pub fn from_env() -> Result<Self, AuthError> {
        let home_domain = env::var("HOME_DOMAIN")
            .map_err(|_| AuthError::ConfigError("Missing HOME_DOMAIN".into()))?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| AuthError::ConfigError("Missing JWT_SECRET".into()))?;

        let network = Network::new_public();

        let challenge_timeout = env::var("CHALLENGE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300); 

        Ok(Self {
            client: Client::new(),
            home_domain,
            network,
            jwt_secret,
            web_auth_endpoint: OnceCell::new(),
            signing_key: OnceCell::new(),
            challenge_timeout,
        })
    }
    pub fn global() -> Arc<Self> {
        STELLAR_AUTH.clone()
    }
    pub async fn init(&self) -> Result<(), AuthError> {
        let toml = self.fetch_stellar_toml().await?;
        self.web_auth_endpoint.set(toml.web_auth_endpoint)
            .map_err(|_| AuthError::ConfigError("WEB_AUTH_ENDPOINT already set".into()))?;
        self.signing_key.set(toml.signing_key)
            .map_err(|_| AuthError::ConfigError("SIGNING_KEY already set".into()))?;
        Ok(())
    }

    pub async fn authenticate(&self, account_id: &str, keypair: &KeyPair) -> Result<String, AuthError> {
        let challenge = self.get_challenge(account_id, None).await?;
        self.verify_challenge_structure(&challenge, account_id)?;
        let signed_envelope = self.sign_challenge(&challenge, keypair)?;
        let xdr_base64 = signed_envelope.xdr_base64()
            .map_err(|e| AuthError::XdrError(format!("XDR serialization failed: {}", e)))?;
        self.get_jwt_token(&xdr_base64).await
    }

    async fn fetch_stellar_toml(&self) -> Result<StellarToml, AuthError> {
        let url = format!("https://{}/.well-known/stellar.toml", self.home_domain);
        let toml_str = self.client.get(&url).send().await?.text().await?;
        let toml: StellarToml = toml::from_str(&toml_str)
            .map_err(|e| AuthError::ConfigError(format!("TOML parsing failed: {}", e)))?;
        Ok(toml)
    }

   pub  async fn get_challenge(&self, account_id: &str, client_domain: Option<&str>) -> Result<String, AuthError> {
        let endpoint = self.web_auth_endpoint.get()
            .ok_or(AuthError::ConfigError("WEB_AUTH_ENDPOINT not initialized".into()))?;

        let mut request = self.client.get(endpoint).query(&[("account", account_id)]);

        if let Some(domain) = client_domain {
            request = request.query(&[("client_domain", domain)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(AuthError::AuthFailed(format!(
                "Challenge request failed with status: {}",
                response.status()
            )));
        }

        let challenge: ChallengeResponse = response.json().await?;
        Ok(challenge.transaction)
    }   

    pub fn verify_jwt(&self, token: &str) -> Result<String, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|e| AuthError::JwtError(e))?;

        // Verify issuer matches our home domain
        if token_data.claims.iss != self.home_domain {
            return Err(AuthError::AuthFailed("Invalid token issuer".into()));
        }

        // Verify token hasn't expired
        let now = Utc::now().timestamp() as u64;


        if token_data.claims.exp < now {
            return Err(AuthError::AuthFailed("Token expired".into()));
        }

        Ok(token_data.claims.sub)
    }

    fn verify_challenge_structure(&self, challenge_xdr: &str, client_account: &str) -> Result<(), AuthError> {
        let envelope = TransactionEnvelope::from_xdr_base64(challenge_xdr)
            .map_err(|e| AuthError::XdrError(format!("XDR parsing failed: {}", e)))?;

        let transaction = match &envelope {
            TransactionEnvelope::Transaction(tx) => tx,
            TransactionEnvelope::FeeBumpTransaction(_) => {
                return Err(AuthError::ChallengeError(
                    "Fee bump transactions not supported in challenges".into()
                ));
            }
        };

        // Verify sequence number is 0
        if *transaction.sequence() != 0 {
            return Err(AuthError::ChallengeError("Sequence number must be 0".into()));
        }

        // Verify server signature
        let server_key = self.signing_key.get()
            .ok_or(AuthError::ConfigError("SIGNING_KEY not initialized".into()))?;

        let server_keypair = Keypair::from_public_key(server_key)
            .map_err(|e| AuthError::ConfigError(format!("Invalid server key: {}", e)))?;

        let signature_data = transaction.signature_data(&self.network)
            .map_err(|e| AuthError::XdrError(format!("Failed to get signature data: {}", e)))?;

        let signature_valid = transaction.signatures().iter().any(|sig| {
            server_keypair.verify(&signature_data, sig.signature().as_bytes())
        });

        if !signature_valid {
            return Err(AuthError::SignatureError);
        }

        // Verify first operation is ManageData with correct structure
        let operations = transaction.operations();
        if operations.is_empty() {
            return Err(AuthError::ChallengeError("No operations in challenge".into()));
        }

        let first_op = &operations[0];
        if let Operation::ManageData(op) = first_op {
            // Check source account (using getter method if field is private)
            if let Some(source) = op.source_account() {
                if source.account_id().to_string() != client_account {
                    return Err(AuthError::ChallengeError(
                        format!("First operation source must be {}", client_account)
                    ));
                }
            } else {
                return Err(AuthError::ChallengeError(
                    "Operation must have a source account".into()
                ));
            }

            // Check data_name
            if op.data_name() != format!("{} auth", self.home_domain) {
                return Err(AuthError::ChallengeError(
                    format!("First operation key must be '{} auth'", self.home_domain)
                ));
            }

            // Check data_value
            if op.data_value().as_ref().map(|d| d.as_bytes().is_empty()).unwrap_or(true) {
                return Err(AuthError::ChallengeError(
                    "First operation must have nonce value".into()
                ));
            }
              } else {
                  return Err(AuthError::ChallengeError(
                      "First operation must be ManageData".into()
                  ));
              }

        // Verify time bounds if present
        if let Some(time_bounds) = transaction.time_bounds() {
            let now = Utc::now().timestamp();          let upper_bound = time_bounds.upper();

          if let Some(upper_bound) = upper_bound {
            if now > upper_bound.timestamp() {
                return Err(AuthError::ChallengeError(
                    "Challenge transaction expired".into()
                ));
            }
          } else {
              return Err(AuthError::ChallengeError(
                  "Challenge transaction has no time bounds".into()
              ));
          }
        }

        Ok(())
    }

    fn sign_challenge(&self, challenge_xdr: &str, keypair: &KeyPair) -> Result<TransactionEnvelope, AuthError> {
        let mut envelope = TransactionEnvelope::from_xdr_base64(challenge_xdr)
            .map_err(|e| AuthError::XdrError(format!("XDR parsing failed: {}", e)))?;

        envelope.sign(keypair, &self.network)
            .map_err(|e| AuthError::XdrError(format!("Signing failed: {}", e)))?;

        Ok(envelope)
    }

   pub async fn get_jwt_token(&self, signed_xdr: &str) -> Result<String, AuthError> {
        let endpoint = self.web_auth_endpoint.get()
            .ok_or(AuthError::ConfigError("WEB_AUTH_ENDPOINT not initialized".into()))?;

        let response = self.client
            .post(endpoint)
            .json(&TokenRequest {
                transaction: signed_xdr.to_string(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AuthError::AuthFailed(format!(
                "Token request failed with status: {}",
                response.status()
            )));
        }

        let token: TokenResponse = response.json().await?;
        Ok(token.token)
    }

    pub fn generate_service_jwt(&self, account_id: &str) -> Result<String, AuthError> {
        #[derive(Serialize)]
        struct Claims {
            iss: String,
            sub: String,
            iat: u64,
            exp: u64,
        }

        let now = Utc::now().timestamp() as u64;

        let exp = now + self.challenge_timeout;

        let claims = Claims {
            iss: self.home_domain.clone(),
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
