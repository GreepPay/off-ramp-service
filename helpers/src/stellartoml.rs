use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;
use url::Url;


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

#[derive(Debug, Clone, Deserialize)]
pub struct StellarToml {
    #[serde(rename = "WEB_AUTH_ENDPOINT")]
    pub web_auth_endpoint: String,
    #[serde(rename = "KYC_SERVER")]
    pub kyc_server: Option<String>,
    #[serde(rename = "TRANSFER_SERVER")]
    pub transfer_server: String,
    #[serde(rename = "ANCHOR_QUOTE_SERVER")]
    pub quote_server: Option<String>,
    #[serde(rename = "SIGNING_KEY")]
    pub signing_key: String,
    #[serde(rename = "DOCUMENTATION")]
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Documentation {
    pub org_name: Option<String>,
    pub org_url: Option<String>,
    pub org_logo: Option<String>,
}

pub struct TomlFetcher {
    client: Client,
}

impl TomlFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_toml(&self, domain: &str) -> Result<StellarToml, TomlError> {
        let url = format!("https://{}/.well-known/stellar.toml", domain);
        let url = Url::parse(&url)?;
        
        let response = self.client.get(url).send().await?;
        let toml_str = response.text().await?;
        
        let toml: StellarToml = toml::from_str(&toml_str)
            .map_err(|e| TomlError::TomlParseError(e.to_string()))?;
        
        Ok(toml)
    }
}