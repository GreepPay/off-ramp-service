use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use stellar_base::Asset;
use controllers::api::api::{failure, success, ApiResponse};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("Transfer failed: {0}")]
    TransferFailed(String),
    #[error("Transaction not found")]
    TransactionNotFound,
}

#[derive(Debug, Serialize)]
pub struct WithdrawRequest {
    pub asset_code: String,
    pub account: String,
    pub amount: String,
    pub dest: Option<String>,
    pub dest_extra: Option<String>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawResponse {
    pub id: String,
    pub account_id: Option<String>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub eta: Option<u64>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_fixed: Option<f64>,
    pub fee_percent: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionStatus {
    pub id: String,
    pub status: String,
    pub amount_in: Option<String>,
    pub amount_out: Option<String>,
    pub amount_fee: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub stellar_transaction_id: Option<String>,
    pub external_transaction_id: Option<String>,
}

pub struct TransferService {
    client: Client,
}

impl TransferService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_info(
        &self,
        transfer_server: &str,
    ) -> Result<status::Custom<Json<ApiResponse<serde_json::Value>>>, status::Custom<Json<ApiResponse<()>>>> {
        let response = self.client
            .get(&format!("{}/info", transfer_server))
            .send()
            .await
            .map_err(|e| {
                failure(
                    &format!("Failed to connect to transfer server: {}", e),
                    Status::BadGateway,
                )
            })?;

        if !response.status().is_success() {
            return Err(failure(
                &format!("Transfer server info request failed: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest),
            ));
        }

        let info = response.json().await.map_err(|e| {
            failure(
                &format!("Failed to parse transfer server info: {}", e),
                Status::InternalServerError,
            )
        })?;

        Ok(success(
            "Transfer info retrieved successfully",
            info,
            Status::Ok,
        ))
    }

    pub async fn initiate_withdraw(
        &self,
        transfer_server: &str,
        request: WithdrawRequest,
        jwt: &str,
    ) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
        let response = self.client
            .post(&format!("{}/withdraw", transfer_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                failure(
                    &format!("Withdraw request failed: {}", e),
                    Status::BadGateway,
                )
            })?;

        if !response.status().is_success() {
            return Err(failure(
                &format!("Withdraw initiation failed: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest),
            ));
        }

        let withdraw_response = response.json().await.map_err(|e| {
            failure(
                &format!("Failed to parse withdraw response: {}", e),
                Status::InternalServerError,
            )
        })?;

        Ok(success(
            "Withdraw initiated successfully",
            withdraw_response,
            Status::Ok,
        ))
    }

    pub async fn get_transaction_status(
        &self,
        transfer_server: &str,
        transaction_id: &str,
        jwt: &str,
    ) -> Result<status::Custom<Json<ApiResponse<TransactionStatus>>>, status::Custom<Json<ApiResponse<()>>>> {
        let response = self.client
            .get(&format!("{}/transaction", transfer_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .query(&[("id", transaction_id)])
            .send()
            .await
            .map_err(|e| {
                failure(
                    &format!("Transaction status request failed: {}", e),
                    Status::BadGateway,
                )
            })?;

        match response.status().as_u16() {
            200 => {
                let status = response.json().await.map_err(|e| {
                    failure(
                        &format!("Failed to parse transaction status: {}", e),
                        Status::InternalServerError,
                    )
                })?;
                Ok(success(
                    "Transaction status retrieved",
                    status,
                    Status::Ok,
                ))
            }
            404 => Err(failure(
                "Transaction not found",
                Status::NotFound,
            )),
            _ => Err(failure(
                &format!("Transaction status check failed: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest),
            )),
        }
    }
}