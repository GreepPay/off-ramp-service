use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;



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

#[derive(Error, Debug)]
pub enum AnchorError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("TOML parsing failed: {0}")]
    TomlParseError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

// Anchor Configuration

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorConfig {
    // General Information Fields
    #[serde(rename = "VERSION")]
    pub version: String,

    #[serde(rename = "NETWORK_PASSPHRASE")]
    pub network_passphrase: String,

    #[serde(rename = "FEDERATION_SERVER")]
    pub federation_server: Option<String>,

    #[serde(rename = "AUTH_SERVER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_server: Option<String>, 

    #[serde(rename = "TRANSFER_SERVER")]
    pub transfer_server: Option<String>,

    #[serde(rename = "TRANSFER_SERVER_SEP0024")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_server_sep0024: Option<String>,

    #[serde(rename = "KYC_SERVER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_server: Option<String>,

    #[serde(rename = "WEB_AUTH_ENDPOINT")]
    pub web_auth_endpoint: String,

    #[serde(rename = "WEB_AUTH_FOR_CONTRACTS_ENDPOINT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_auth_for_contracts_endpoint: Option<String>,

    #[serde(rename = "WEB_AUTH_CONTRACT_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_auth_contract_id: Option<String>,

    #[serde(rename = "SIGNING_KEY")]
    pub signing_key: String,

    #[serde(rename = "HORIZON_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizon_url: Option<String>,

    #[serde(rename = "ACCOUNTS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,

    #[serde(rename = "URI_REQUEST_SIGNING_KEY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_request_signing_key: Option<String>,

    #[serde(rename = "DIRECT_PAYMENT_SERVER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_payment_server: Option<String>,

    #[serde(rename = "ANCHOR_QUOTE_SERVER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_quote_server: Option<String>,

    // Documentation Fields
    #[serde(rename = "DOCUMENTATION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    #[serde(rename = "ORG_NAME")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,

    #[serde(rename = "ORG_DBA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_dba: Option<String>,

    #[serde(rename = "ORG_URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_url: Option<String>,

    #[serde(rename = "ORG_LOGO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_logo: Option<String>,

    #[serde(rename = "ORG_DESCRIPTION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_description: Option<String>,

    #[serde(rename = "ORG_PHYSICAL_ADDRESS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_physical_address: Option<String>,

    #[serde(rename = "ORG_PHYSICAL_ADDRESS_ATTESTATION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_physical_address_attestation: Option<String>,

    #[serde(rename = "ORG_PHONE_NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_phone_number: Option<String>,

    #[serde(rename = "ORG_PHONE_NUMBER_ATTESTATION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_phone_number_attestation: Option<String>,

    #[serde(rename = "ORG_KEYBASE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_keybase: Option<String>,

    #[serde(rename = "ORG_TWITTER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_twitter: Option<String>,

    #[serde(rename = "ORG_GITHUB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_github: Option<String>,

    #[serde(rename = "ORG_OFFICIAL_EMAIL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_official_email: Option<String>,

    #[serde(rename = "ORG_SUPPORT_EMAIL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_support_email: Option<String>,

    #[serde(rename = "ORG_LICENSING_AUTHORITY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_licensing_authority: Option<String>,

    #[serde(rename = "ORG_LICENSE_TYPE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_license_type: Option<String>,

    #[serde(rename = "ORG_LICENSE_NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_license_number: Option<String>,
}


pub struct AnchorService {
    client: Client,
    anchors: Arc<RwLock<HashMap<String, AnchorConfig>>>,
}


impl AnchorService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            anchors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn fetch_anchor_config(&self, slug: &str) -> Result<AnchorConfig, AnchorError> {
        let url = format!("https://{}/.well-known/stellar.toml", slug);
        let url = Url::parse(&url)?;

        let response = self.client.get(url).send().await?;
        let toml_str = response.text().await?;

        let toml: AnchorConfig = toml::from_str(&toml_str)
            .map_err(|e| AnchorError::TomlParseError(e.to_string()))?;

        Ok(toml)
    }
    
    pub async fn upsert_anchor(&self, slug: String, config: AnchorConfig) {
            let mut anchors = self.anchors.write().await;
            anchors.insert(slug, config);
        }
    
    pub async fn get_anchor(&self, slug: &str) -> Result<AnchorConfig, AnchorError> {
        let anchors = self.anchors.read().await;
        anchors.get(slug)
                .cloned()
                .ok_or_else(|| AnchorError::MissingField(format!("anchor not found for {}", slug)))
        }
    }
