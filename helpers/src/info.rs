use serde_json::Value;
use thiserror::Error;
use serde::Deserialize;
use serde::Serialize;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    #[serde(rename = "asset")]
    pub asset_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_issuer: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_fixed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sep12: Option<Sep12Info>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sep38: Option<Sep38Info>,
    #[serde(flatten)]
    pub extra_fields: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sep12Info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sep38Info {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositInfo {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_fixed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawInfo {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_fixed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub deposit: std::collections::HashMap<String, DepositInfo>,
    pub withdraw: std::collections::HashMap<String, WithdrawInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Value>,
}

#[derive(Error, Debug)]
pub enum InfoError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Transfer server returned error: {0}")]
    TransferServerError(String),
    #[error("Asset not supported for operation: {0}")]
    AssetNotSupported(String),
    #[error("Invalid operation type (must be 'deposit' or 'withdraw')")]
    InvalidOperation,
    #[error("No data received from transfer server")]
    NoData,
}

pub struct InfoService {
    client: Client,
}

impl InfoService {
    pub fn new() -> Self {
        InfoService {
            client: Client::new(),
        }
    }

    pub async fn get_info(
        &self,
        transfer_server: &str,
    ) -> Result<InfoResponse, InfoError> {
        let response = self.client
            .get(&format!("{}/info", transfer_server))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InfoError::TransferServerError(
                format!("Status code: {}", response.status())
            ));
        }

        let info: InfoResponse = response.json().await?;
        Ok(info)
    }

    pub async fn get_asset_info(
        &self,
        transfer_server: &str,
        asset_code: &str,
        operation: &str, // "deposit" or "withdraw"
    ) -> Result<AssetInfo, InfoError> {
        let info = self.get_info(transfer_server).await?;

        match operation {
            "deposit" => {
                if let Some(deposit_info) = info.deposit.get(asset_code) {
                    Ok(AssetInfo {
                        asset_code: asset_code.to_string(),
                        asset_issuer: None,
                        min_amount: deposit_info.min_amount,
                        max_amount: deposit_info.max_amount,
                        fee_fixed: deposit_info.fee_fixed,
                        fee_percent: deposit_info.fee_percent,
                        sep12: None,
                        sep38: None,
                        extra_fields: None,
                    })
                } else {
                    Err(InfoError::AssetNotSupported(
                        format!("Asset {} not supported for deposit", asset_code)
                    ))
                }
            }
            "withdraw" => {
                if let Some(withdraw_info) = info.withdraw.get(asset_code) {
                    Ok(AssetInfo {
                        asset_code: asset_code.to_string(),
                        asset_issuer: None,
                        min_amount: withdraw_info.min_amount,
                        max_amount: withdraw_info.max_amount,
                        fee_fixed: withdraw_info.fee_fixed,
                        fee_percent: withdraw_info.fee_percent,
                        sep12: None,
                        sep38: None,
                        extra_fields: None,
                    })
                } else {
                    Err(InfoError::AssetNotSupported(
                        format!("Asset {} not supported for withdraw", asset_code)
                    ))
                }
            }
            _ => Err(InfoError::InvalidOperation),
        }
    }
}