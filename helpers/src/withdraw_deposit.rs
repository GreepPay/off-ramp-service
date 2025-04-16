use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;


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

    pub async fn fetch_info(
        &self,
        transfer_server: &str,
    ) -> Result<serde_json::Value, TransferError> {
        let response = self.client
            .get(&format!("{}/info", transfer_server))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TransferError::TransferFailed(
                format!("Transfer server info request failed: {}", response.status())
            ));
        }

        response.json().await
            .map_err(|e| TransferError::TransferFailed(
                format!("Failed to parse transfer server info: {}", e)
            ))
    }

    pub async fn process_withdrawal(
        &self,
        transfer_server: &str,
        request: WithdrawRequest,
        jwt: &str,
    ) -> Result<WithdrawResponse, TransferError> {
        let response = self.client
            .post(&format!("{}/withdraw", transfer_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TransferError::TransferFailed(
                format!("Withdraw initiation failed: {}", response.status())
            ));
        }

        response.json().await
            .map_err(|e| TransferError::TransferFailed(
                format!("Failed to parse withdraw response: {}", e)
            ))
    }

    pub async fn check_transaction_status(
        &self,
        transfer_server: &str,
        transaction_id: &str,
        jwt: &str,
    ) -> Result<TransactionStatus, TransferError> {
        let response = self.client
            .get(&format!("{}/transaction", transfer_server))
            .header("Authorization", format!("Bearer {}", jwt))
            .query(&[("id", transaction_id)])
            .send()
            .await?;

        match response.status().as_u16() {
            200 => response.json().await
                .map_err(|e| TransferError::TransferFailed(
                    format!("Failed to parse transaction status: {}", e)
                )),
            404 => Err(TransferError::TransactionNotFound),
            _ => Err(TransferError::TransferFailed(
                format!("Transaction status check failed: {}", response.status())
            )),
        }
    }
}