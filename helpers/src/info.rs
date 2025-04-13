// Add to transfer_service.rs
use serde_json::Value;
use controllers::{
    api::api::{failure, success, ApiResponse},
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

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

impl InfoService {
    pub async fn get_info(
        &self,
        transfer_server: &str,
    ) -> Result<status::Custom<Json<ApiResponse<InfoResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
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
                &format!("Transfer server returned error: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest),
            ));
        }

        let info: InfoResponse = response.json().await.map_err(|e| {
            failure(
                &format!("Failed to parse transfer server response: {}", e),
                Status::InternalServerError,
            )
        })?;

        Ok(success(
            "Transfer info retrieved successfully",
            info,
            Status::Ok,
        ))
    }

    pub async fn get_asset_info(
        &self,
        transfer_server: &str,
        asset_code: &str,
        operation: &str, // "deposit" or "withdraw"
    ) -> Result<status::Custom<Json<ApiResponse<AssetInfo>>>, status::Custom<Json<ApiResponse<()>>>> {
        let info_response = self.get_info(transfer_server).await?;
        let info = info_response.into_inner().data.ok_or_else(|| {
            failure(
                "No data received from transfer server",
                Status::InternalServerError,
            )
        })?;

        match operation {
            "deposit" => {
                if let Some(deposit_info) = info.deposit.get(asset_code) {
                    Ok(success(
                        "Asset deposit info retrieved",
                        AssetInfo {
                            asset_code: asset_code.to_string(),
                            asset_issuer: None,
                            min_amount: deposit_info.min_amount,
                            max_amount: deposit_info.max_amount,
                            fee_fixed: deposit_info.fee_fixed,
                            fee_percent: deposit_info.fee_percent,
                            sep12: None,
                            sep38: None,
                            extra_fields: None,
                        },
                        Status::Ok,
                    ))
                } else {
                    Err(failure(
                        &format!("Asset {} not supported for deposit", asset_code),
                        Status::BadRequest,
                    ))
                }
            }
            "withdraw" => {
                if let Some(withdraw_info) = info.withdraw.get(asset_code) {
                    Ok(success(
                        "Asset withdraw info retrieved",
                        AssetInfo {
                            asset_code: asset_code.to_string(),
                            asset_issuer: None,
                            min_amount: withdraw_info.min_amount,
                            max_amount: withdraw_info.max_amount,
                            fee_fixed: withdraw_info.fee_fixed,
                            fee_percent: withdraw_info.fee_percent,
                            sep12: None,
                            sep38: None,
                            extra_fields: None,
                        },
                        Status::Ok,
                    ))
                } else {
                    Err(failure(
                        &format!("Asset {} not supported for withdraw", asset_code),
                        Status::BadRequest,
                    ))
                }
            }
            _ => Err(failure(
                "Invalid operation type (must be 'deposit' or 'withdraw')",
                Status::BadRequest,
            )),
        }
    }
}