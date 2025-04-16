use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KycError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("KYC submission failed: {0}")]
    SubmissionFailed(String),
    #[error("KYC check failed: {0}")]
    CheckFailed(String),
}

#[derive(Debug, Serialize)]
pub struct KycFields {
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct KycStatus {
    pub status: String,
    pub fields: Option<Vec<String>>,
}

pub struct KycService {
    client: Client,
}

impl KycService {
    pub fn new() -> Self {
        KycService {
            client: Client::new(),
        }
    }

    pub async fn get_kyc_status(
        &self,
        kyc_server: &str,
        account_id: &str,
        jwt: &str,
    ) -> Result<KycStatus, KycError> {
        let response = self.client
            .get(&format!("{}/customer", kyc_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .query(&[("account", account_id)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(KycError::CheckFailed(
                format!("KYC check failed with status: {}", response.status())
            ));
        }

        let status: KycStatus = response.json().await?;
        Ok(status)
    }

    pub async fn submit_kyc(
        &self,
        kyc_server: &str,
        kyc_fields: KycFields,
        jwt: &str,
    ) -> Result<KycStatus, KycError> {
        let response = self.client
            .put(&format!("{}/customer", kyc_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&kyc_fields)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(KycError::SubmissionFailed(
                format!("KYC submission failed with status: {}", response.status())
            ));
        }

        let status: KycStatus = response.json().await?;
        Ok(status)
    }
}