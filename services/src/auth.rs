use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use stellar_base::{KeyPair, Network, transaction::TransactionEnvelope, Operation};
use stellar_sdk::Keypair;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use once_cell::sync::OnceCell;
use helpers::stellartoml::TomlFetcher;


#[derive(Error, Debug)]
pub enum TomlError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("TOML parsing failed: {0}")]
    TomlParseError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}
// Error Types
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("XDR parsing failed: {0}")]
    XdrError(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Invalid challenge: {0}")]
    ChallengeError(String),
    #[error("Signature verification failed")]
    SignatureError,
    #[error("Configuration missing: {0}")]
    ConfigError(String),
}


// Anchor Configuration
#[derive(Debug, Clone)]
pub struct AnchorConfig {
    pub base_url: String,
    pub web_auth_endpoint: String,
    pub signing_key: String,
    pub transfer_server: Option<String>,
    pub kyc_server: Option<String>,
}

// Main StellarAuth Service
#[derive(Debug, Clone)]
pub struct StellarAuth {
    client: Client,
    toml_fetcher: TomlFetcher,
    network: Network,
    anchors: Arc<RwLock<HashMap<String, AnchorConfig>>>,
    web_auth_endpoint: OnceCell<String>,
    signing_key: OnceCell<String>,
}

impl StellarAuth {
    // Initialization
    pub fn new(toml_fetcher: TomlFetcher, initial_anchors: HashMap<String, AnchorConfig>) -> Self {
        Self {
            client: Client::new(),
            toml_fetcher,
            network: Network::new_public(),
            anchors: Arc::new(RwLock::new(initial_anchors)),
            web_auth_endpoint: OnceCell::new(),
            signing_key: OnceCell::new(),
        }
    }

    // Anchor Management
    pub async fn upsert_anchor(&self, slug: String, config: AnchorConfig) {
        let mut anchors = self.anchors.write().await;
        anchors.insert(slug, config);
    }

    pub async fn get_anchor(&self, slug: &str) -> Result<AnchorConfig, AuthError> {
        let anchors = self.anchors.read().await;
        anchors.get(slug)
            .cloned()
            .ok_or_else(|| AuthError::ConfigError(format!("Unknown anchor: {}", slug)))
    }

    pub async fn refresh_anchor(&self, slug: &str, domain: &str) -> Result<(), AuthError> {
        let toml = self.toml_fetcher.fetch_toml(domain).await?;
        let config = AnchorConfig {
            base_url: domain.to_string(),
            web_auth_endpoint: toml.web_auth_endpoint.clone(),
            signing_key: toml.signing_key.clone(),
            transfer_server: toml.transfer_server,
            kyc_server: toml.kyc_server,
        };
        self.upsert_anchor(slug.to_string(), config).await;
        Ok(())
    }

    // Authentication Flow
    pub async fn init(&self, anchor_domain: &str) -> Result<(), AuthError> {
        let toml = self.toml_fetcher.fetch_toml(anchor_domain).await?;
        self.web_auth_endpoint.set(toml.web_auth_endpoint.clone())
            .map_err(|_| AuthError::ConfigError("WEB_AUTH_ENDPOINT already set".into()))?;
        self.signing_key.set(toml.signing_key.clone())
            .map_err(|_| AuthError::ConfigError("SIGNING_KEY already set".into()))?;
        Ok(())
    }

    pub async fn create_challenge(&self, account_id: &str) -> Result<String, AuthError> {
        let endpoint = self.web_auth_endpoint.get()
            .ok_or(AuthError::ConfigError("WEB_AUTH_ENDPOINT not initialized".into()))?;
        let response = self.client.get(endpoint).query(&[("account", account_id)]).send().await?;
        
        if !response.status().is_success() {
            return Err(AuthError::AuthFailed(format!("Challenge request failed: {}", response.status())));
        }

        #[derive(Deserialize)]
        struct Challenge { transaction: String }
        let challenge: Challenge = response.json().await?;
        Ok(challenge.transaction)
    }

    pub fn sign_challenge(&self, challenge_xdr: &str, user_keypair: &KeyPair) -> Result<String, AuthError> {
        self.verify_challenge_structure(challenge_xdr, user_keypair.account_id())?;
        let mut envelope = TransactionEnvelope::from_xdr_base64(challenge_xdr)
            .map_err(|e| AuthError::XdrError(e.to_string()))?;
        envelope.sign(user_keypair, &self.network)
            .map_err(|e| AuthError::XdrError(e.to_string()))?;
        envelope.xdr_base64()
            .map_err(|e| AuthError::XdrError(e.to_string()))
    }

    pub async fn exchange_for_token(&self, signed_xdr: &str) -> Result<String, AuthError> {
        let endpoint = self.web_auth_endpoint.get()
            .ok_or(AuthError::ConfigError("WEB_AUTH_ENDPOINT not initialized".into()))?;
        #[derive(Serialize)]
        struct TokenRequest { transaction: String }
        let response = self.client.post(endpoint)
            .json(&TokenRequest { transaction: signed_xdr.to_string() })
            .send().await?;
        #[derive(Deserialize)]
        struct TokenResponse { token: String }
        let token: TokenResponse = response.json().await?;
        Ok(token.token)
    }

    pub fn verify_jwt(&self, token: &str) -> Result<String, AuthError> {
        #[derive(Debug, Deserialize)]
        struct Claims {
            iss: String,
            sub: String,
            exp: u64,
        }
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret("placeholder".as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims.sub)
    }

    pub async fn get_jwt(&self, account_id: &str) -> Result<String, AuthError> {
        let challenge_xdr = self.create_challenge(account_id).await?;
        let user_keypair = KeyPair::from_account_id(account_id)
            .map_err(|e| AuthError::AuthFailed(format!("Invalid account ID: {}", e)))?;
        let signed_xdr = self.sign_challenge(&challenge_xdr, &user_keypair)?;
        let token = self.exchange_for_token(&signed_xdr).await?;
        self.verify_jwt(&token)?;
        Ok(token)
    }

    pub async fn get_jwt_for_anchor(&self, slug: &str, account_id: &str) -> Result<String, AuthError> {
        let anchor = self.get_anchor(slug).await?;
        let temp_auth = Self {
            client: self.client.clone(),
            toml_fetcher: self.toml_fetcher.clone(),
            network: self.network.clone(),
            anchors: Arc::clone(&self.anchors),
            web_auth_endpoint: OnceCell::new(),
            signing_key: OnceCell::new(),
        };
        temp_auth.web_auth_endpoint.set(anchor.web_auth_endpoint.clone())
            .map_err(|_| AuthError::ConfigError("WEB_AUTH_ENDPOINT already set".into()))?;
        temp_auth.signing_key.set(anchor.signing_key.clone())
            .map_err(|_| AuthError::ConfigError("SIGNING_KEY already set".into()))?;
        temp_auth.get_jwt(account_id).await
    }

    // Helper Methods
    /// Helper: Validate challenge structure
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
}