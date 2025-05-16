use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use url::Url;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorConfig {
    #[serde(rename = "VERSION")]
    pub version: String,
    #[serde(rename = "NETWORK_PASSPHRASE")]
    pub network_passphrase: String,
    #[serde(rename = "FEDERATION_SERVER")]
    pub federation_server: Option<String>,
    #[serde(rename = "AUTH_SERVER")]
    pub auth_server: Option<String>,
    #[serde(rename = "TRANSFER_SERVER")]
    pub transfer_server: Option<String>,
    #[serde(rename = "TRANSFER_SERVER_SEP0024")]
    pub transfer_server_sep0024: Option<String>,
    #[serde(rename = "KYC_SERVER")]
    pub kyc_server: Option<String>,
    #[serde(rename = "WEB_AUTH_ENDPOINT")]
    pub web_auth_endpoint: String,
    #[serde(rename = "WEB_AUTH_FOR_CONTRACTS_ENDPOINT")]
    pub web_auth_for_contracts_endpoint: Option<String>,
    #[serde(rename = "WEB_AUTH_CONTRACT_ID")]
    pub web_auth_contract_id: Option<String>,
    #[serde(rename = "SIGNING_KEY")]
    pub signing_key: String,
    #[serde(rename = "HORIZON_URL")]
    pub horizon_url: Option<String>,
    #[serde(rename = "ACCOUNTS")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "URI_REQUEST_SIGNING_KEY")]
    pub uri_request_signing_key: Option<String>,
    #[serde(rename = "DIRECT_PAYMENT_SERVER")]
    pub direct_payment_server: Option<String>,
    #[serde(rename = "ANCHOR_QUOTE_SERVER")]
    pub anchor_quote_server: Option<String>,
    #[serde(rename = "DOCUMENTATION")]
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    #[serde(rename = "ORG_NAME")]
    pub org_name: Option<String>,
    #[serde(rename = "ORG_DBA")]
    pub org_dba: Option<String>,
    #[serde(rename = "ORG_URL")]
    pub org_url: Option<String>,
    #[serde(rename = "ORG_LOGO")]
    pub org_logo: Option<String>,
    #[serde(rename = "ORG_DESCRIPTION")]
    pub org_description: Option<String>,
    #[serde(rename = "ORG_PHYSICAL_ADDRESS")]
    pub org_physical_address: Option<String>,
    #[serde(rename = "ORG_PHYSICAL_ADDRESS_ATTESTATION")]
    pub org_physical_address_attestation: Option<String>,
    #[serde(rename = "ORG_PHONE_NUMBER")]
    pub org_phone_number: Option<String>,
    #[serde(rename = "ORG_PHONE_NUMBER_ATTESTATION")]
    pub org_phone_number_attestation: Option<String>,
    #[serde(rename = "ORG_KEYBASE")]
    pub org_keybase: Option<String>,
    #[serde(rename = "ORG_TWITTER")]
    pub org_twitter: Option<String>,
    #[serde(rename = "ORG_GITHUB")]
    pub org_github: Option<String>,
    #[serde(rename = "ORG_OFFICIAL_EMAIL")]
    pub org_official_email: Option<String>,
    #[serde(rename = "ORG_SUPPORT_EMAIL")]
    pub org_support_email: Option<String>,
    #[serde(rename = "ORG_LICENSING_AUTHORITY")]
    pub org_licensing_authority: Option<String>,
    #[serde(rename = "ORG_LICENSE_TYPE")]
    pub org_license_type: Option<String>,
    #[serde(rename = "ORG_LICENSE_NUMBER")]
    pub org_license_number: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AnchorConfigDetails {
    pub general_info: GeneralInfo,
    pub documentation: Option<DocumentationInfo>,
}

#[derive(Debug, Clone)]
pub struct GeneralInfo {
    pub version: String,
    pub network_passphrase: String,
    pub federation_server: Option<String>,
    pub auth_server: Option<String>,
    pub transfer_server: Option<String>,
    pub transfer_server_sep0024: Option<String>,
    pub kyc_server: Option<String>,
    pub web_auth_endpoint: String,
    pub web_auth_for_contracts_endpoint: Option<String>,
    pub web_auth_contract_id: Option<String>,
    pub signing_key: String,
    pub horizon_url: Option<String>,
    pub accounts: Option<Vec<String>>,
    pub uri_request_signing_key: Option<String>,
    pub direct_payment_server: Option<String>,
    pub anchor_quote_server: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DocumentationInfo {
    pub org_name: Option<String>,
    pub org_dba: Option<String>,
    pub org_url: Option<String>,
    pub org_logo: Option<String>,
    pub org_description: Option<String>,
    pub org_physical_address: Option<String>,
    pub org_physical_address_attestation: Option<String>,
    pub org_phone_number: Option<String>,
    pub org_phone_number_attestation: Option<String>,
    pub org_keybase: Option<String>,
    pub org_twitter: Option<String>,
    pub org_github: Option<String>,
    pub org_official_email: Option<String>,
    pub org_support_email: Option<String>,
    pub org_licensing_authority: Option<String>,
    pub org_license_type: Option<String>,
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

    pub async fn fetch_anchor_config(
        &self,
        homepage_url: &str,
    ) -> Result<AnchorConfig, AnchorError> {
        let url = format!("{}/.well-known/stellar.toml", homepage_url);
        let url = Url::parse(&url)?;

        let response = self.client.get(url).send().await?;
        let toml_str = response.text().await?;

        let toml: AnchorConfig =
            toml::from_str(&toml_str).map_err(|e| AnchorError::TomlParseError(e.to_string()))?;

        Ok(toml)
    }

    pub async fn upsert_anchor(&self, slug: String, config: AnchorConfig) {
        let mut anchors = self.anchors.write().await;
        anchors.insert(slug, config);
    }

    pub async fn get_anchor(&self, slug: &str) -> Result<AnchorConfig, AnchorError> {
        let mut anchor_homepage_map: HashMap<String, String> = HashMap::new();

        // Set default supported anchors
        anchor_homepage_map.insert("mykobo".to_string(), "https://www.mykobo.co".to_string());

        let current_anchor = anchor_homepage_map
            .get(slug)
            .cloned()
            .ok_or_else(|| AnchorError::MissingField(format!("anchor not found for {}", slug)));

        if current_anchor.is_ok() {
            let anchor_config = self
                .fetch_anchor_config(current_anchor.unwrap().as_str())
                .await;

            anchor_config
        } else {
            Err(AnchorError::MissingField(format!(
                "anchor not found for {}",
                slug
            )))
        }
    }

    pub async fn get_anchor_config_details(
        &self,
        slug: &str,
    ) -> Result<AnchorConfigDetails, AnchorError> {
        let anchor_config = self.get_anchor(slug).await?;
        let details = AnchorConfigDetails {
            general_info: GeneralInfo {
                version: anchor_config.version.clone(),
                network_passphrase: anchor_config.network_passphrase.clone(),
                federation_server: anchor_config.federation_server.clone(),
                auth_server: anchor_config.auth_server.clone(),
                transfer_server: anchor_config.transfer_server.clone(),
                transfer_server_sep0024: anchor_config.transfer_server_sep0024.clone(),
                kyc_server: anchor_config.kyc_server.clone(),
                web_auth_endpoint: anchor_config.web_auth_endpoint.clone(),
                web_auth_for_contracts_endpoint: anchor_config
                    .web_auth_for_contracts_endpoint
                    .clone(),
                web_auth_contract_id: anchor_config.web_auth_contract_id.clone(),
                signing_key: anchor_config.signing_key.clone(),
                horizon_url: anchor_config.horizon_url.clone(),
                accounts: anchor_config.accounts.clone(),
                uri_request_signing_key: anchor_config.uri_request_signing_key.clone(),
                direct_payment_server: anchor_config.direct_payment_server.clone(),
                anchor_quote_server: anchor_config.anchor_quote_server.clone(),
            },
            documentation: match anchor_config.documentation {
                Some(doc) => Some(DocumentationInfo {
                    org_name: doc.org_name.clone(),
                    org_dba: doc.org_dba.clone(),
                    org_url: doc.org_url.clone(),
                    org_logo: doc.org_logo.clone(),
                    org_description: doc.org_description.clone(),
                    org_physical_address: doc.org_physical_address.clone(),
                    org_physical_address_attestation: doc.org_physical_address_attestation.clone(),
                    org_phone_number: doc.org_phone_number.clone(),
                    org_phone_number_attestation: doc.org_phone_number_attestation.clone(),
                    org_keybase: doc.org_keybase.clone(),
                    org_twitter: doc.org_twitter.clone(),
                    org_github: doc.org_github.clone(),
                    org_official_email: doc.org_official_email.clone(),
                    org_support_email: doc.org_support_email.clone(),
                    org_licensing_authority: doc.org_licensing_authority.clone(),
                    org_license_type: doc.org_license_type.clone(),
                    org_license_number: doc.org_license_number.clone(),
                }),
                __ => None,
            },
        };

        Ok(details)
    }
}
