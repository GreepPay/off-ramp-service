use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use stellar_base::{transaction::TransactionEnvelope, KeyPair, Network, Operation};
use stellar_sdk::Keypair;
use crate::stellartoml::AnchorService;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
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

#[derive(Debug, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: u64,
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

pub async fn authenticate(
    anchor_service: &AnchorService,
    slug: &str,
    account_id: &str,
    keypair: &KeyPair
) -> Result<String, AuthError> {
    let anchor_config = anchor_service.get_anchor_config_details(slug).await
           .map_err(|e| AuthError::ConfigError(e.to_string()))?;
    let client = Client::new();
    let web_auth_endpoint = &anchor_config.general_info.web_auth_endpoint;
    let signing_key = &anchor_config.general_info.signing_key;
    let network_passphrase = &anchor_config.general_info.network_passphrase;
    let network = Network::new(network_passphrase.clone());
    let home_domain: Option<&str> = anchor_config.documentation
        .as_ref()  // Gets Option<&DocumentationInfo>
        .and_then(|doc| doc.org_url.as_ref())  // Gets Option<&String>
        .map(|s| s.as_str());  // Converts &String to &str

    let challenge = get_challenge(&client, web_auth_endpoint, account_id, home_domain).await?;
    verify_challenge_structure(&challenge, account_id, signing_key, &network, home_domain)?;
    let signed_envelope = sign_challenge(&challenge, keypair, &network)?;
    let xdr_base64 = signed_envelope.xdr_base64()
        .map_err(|e| AuthError::XdrError(format!("XDR serialization failed: {}", e)))?;
    get_jwt_token(&client, web_auth_endpoint, &xdr_base64).await
}

pub async fn get_challenge(
    client: &Client,
    web_auth_endpoint: &str,
    account_id: &str,
    client_domain: Option<&str>
) -> Result<String, AuthError> {
    let mut request = client.get(web_auth_endpoint).query(&[("account", account_id)]);

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

pub fn verify_jwt(token: &str, jwt_secret: &str, slug: &str) -> Result<String, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ).map_err(AuthError::JwtError)?;

    if token_data.claims.iss != slug {
        return Err(AuthError::AuthFailed("Invalid token issuer".into()));
    }

    let now = Utc::now().timestamp() as u64;
    if token_data.claims.exp < now {
        return Err(AuthError::AuthFailed("Token expired".into()));
    }

    Ok(token_data.claims.sub)
}

fn verify_challenge_structure(
    challenge_xdr: &str,
    client_account: &str,
    signing_key: &str,
    network: &Network,
    home_domain: Option<&str>
) -> Result<(), AuthError> {
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

    if *transaction.sequence() != 0 {
        return Err(AuthError::ChallengeError("Sequence number must be 0".into()));
    }

    let server_keypair = Keypair::from_public_key(signing_key)
        .map_err(|e| AuthError::ConfigError(format!("Invalid server key: {}", e)))?;

    let signature_data = transaction.signature_data(network)
        .map_err(|e| AuthError::XdrError(format!("Failed to get signature data: {}", e)))?;

    let signature_valid = transaction.signatures().iter().any(|sig| {
        server_keypair.verify(&signature_data, sig.signature().as_bytes())
    });

    if !signature_valid {
        return Err(AuthError::SignatureError);
    }

    let operations = transaction.operations();
    if operations.is_empty() {
        return Err(AuthError::ChallengeError("No operations in challenge".into()));
    }

    let first_op = &operations[0];
    if let Operation::ManageData(op) = first_op {
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

        if op.data_name() != format!("{} auth", home_domain.unwrap_or("unknown")) {
            return Err(AuthError::ChallengeError(
                format!("First operation key must be '{} auth'", home_domain.unwrap_or("unknown"))
            ));
        }

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

    if let Some(time_bounds) = transaction.time_bounds() {
        let now = Utc::now().timestamp();
        let upper_bound = time_bounds.upper();

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

fn sign_challenge(
    challenge_xdr: &str,
    keypair: &KeyPair,
    network: &Network
) -> Result<TransactionEnvelope, AuthError> {
    let mut envelope = TransactionEnvelope::from_xdr_base64(challenge_xdr)
        .map_err(|e| AuthError::XdrError(format!("XDR parsing failed: {}", e)))?;

    envelope.sign(keypair, network)
        .map_err(|e| AuthError::XdrError(format!("Signing failed: {}", e)))?;

    Ok(envelope)
}

pub async fn get_jwt_token(
    client: &Client,
    web_auth_endpoint: &str,
    signed_xdr: &str
) -> Result<String, AuthError> {
    let response = client
        .post(web_auth_endpoint)
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

pub fn generate_service_jwt(
    jwt_secret: &str,
    slug: &str,
    account_id: &str,
    challenge_timeout: u64
) -> Result<String, AuthError> {
    #[derive(Serialize)]
    struct ServiceClaims {
        iss: String,
        sub: String,
        iat: u64,
        exp: u64,
    }

    let now = Utc::now().timestamp() as u64;
    let exp = now + challenge_timeout;

    let claims = ServiceClaims {
        iss: slug.to_string(),
        sub: account_id.to_string(),
        iat: now,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ).map_err(AuthError::JwtError)
}
