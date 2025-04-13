use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use jsonwebtoken::Header;
use controllers::{
    api::api::{failure, success, ApiResponse},
};
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
    pub async fn get_kyc_status(
        &self,
        kyc_server: &str,
        account_id: &str,
        jwt: &str,
    ) -> Result<status::Custom<Json<ApiResponse<KycStatus>>>, status::Custom<Json<ApiResponse<()>>> {
        let response = self.client
            .get(&format!("{}/customer", kyc_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .query(&[("account", account_id)])
            .send()
            .await
            .map_err(|e| {
                failure(&format!("KYC check request failed: {}", e), Status::BadRequest)
            })?;

        if !response.status().is_success() {
            return Err(failure(
                &format!("KYC check failed with status: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest)
            ));
        }

        let status: KycStatus = response.json().await.map_err(|e| {
            failure(&format!("Failed to parse KYC response: {}", e), Status::InternalServerError)
        })?;

        Ok(success("KYC status retrieved successfully", status, Status::Ok))
    }

    pub async fn submit_kyc(
        &self,
        kyc_server: &str,
        kyc_fields: KycFields,
        jwt: &str,
    ) -> Result<status::Custom<Json<ApiResponse<KycStatus>>>, status::Custom<Json<ApiResponse<()>>> {
        let response = self.client
            .put(&format!("{}/customer", kyc_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&kyc_fields)
            .send()
            .await
            .map_err(|e| {
                failure(&format!("KYC submission request failed: {}", e), Status::BadRequest)
            })?;

        if !response.status().is_success() {
            return Err(failure(
                &format!("KYC submission failed with status: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest)
            ));
        }

        let status: KycStatus = response.json().await.map_err(|e| {
            failure(&format!("Failed to parse KYC response: {}", e), Status::InternalServerError)
        })?;

        Ok(success("KYC submitted successfully", status, Status::Ok))
    }
}