use rocket::{form::Form, get, http::Status, post, response::status, serde::json::Json};
use controllers::{
use stellar_base::KeyPair;
use controllers::{
    api::api::{failure, success, ApiResponse},
    offramp,
};



#[post("/offramp", data = "<form>")]
pub async fn offramp_funds<'r>(
    form: Form<offramp::form::OfframpForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<String>>>, status::Custom<Json<ApiResponse<()>>>> {
    let transaction_id = offramp::offramp_funds_controller(form)
        .await
        .map_err(|e| {
            failure(
                format!("Failed to initiate offramp: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Offramp initiated successfully",
        transaction_id,
        Status::Ok,
    ))
}

#[get("/transaction/status?<transaction_id>&<account_id>&<secret_key>")]
pub async fn check_transaction_status(
    transaction_id: &str,
    account_id: &str,
    secret_key: &str,
) -> Result<status::Custom<Json<ApiResponse<crate::services::offramp::TransactionStatus>>>, status::Custom<Json<ApiResponse<()>>>> {
    let form = offramp::form::TransactionStatusForm {
        transaction_id,
        account_id,
        secret_key,
    };

    let status = offramp::check_transaction_status_controller(Form::from(form))
        .await
        .map_err(|e| {
            failure(
                format!("Failed to check transaction status: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Transaction status fetched successfully",
        status,
        Status::Ok,
    ))
}

#[get("/transactions?<account_id>&<secret_key>")]
pub async fn get_transactions(
    account_id: &str,
    secret_key: &str,
) -> Result<status::Custom<Json<ApiResponse<Vec<crate::services::offramp::TransactionStatus>>>>, status::Custom<Json<ApiResponse<()>>>> {
    let form = offramp::form::TransactionQueryForm {
        account_id,
        secret_key,
    };

    let transactions = offramp::get_transactions_controller(Form::from(form))
        .await
        .map_err(|e| {
            failure(
                format!("Failed to get transactions: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Transactions fetched successfully",
        transactions,
        Status::Ok,
    ))
}

#[get("/transaction?<transaction_id>&<account_id>&<secret_key>")]
pub async fn get_transaction(
    transaction_id: &str,
    account_id: &str,
    secret_key: &str,
) -> Result<status::Custom<Json<ApiResponse<crate::services::offramp::TransactionStatus>>>, status::Custom<Json<ApiResponse<()>>>> {
    let form = offramp::form::SingleTransactionQueryForm {
        transaction_id,
        account_id,
        secret_key,
    };

    let transaction = offramp::get_transaction_controller(Form::from(form))
        .await
        .map_err(|e| {
            failure(
                format!("Failed to get transaction: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Transaction fetched successfully",
        transaction,
        Status::Ok,
    ))
}

#[get("/asset/info?<asset_code>&<operation_type>")]
pub async fn get_asset_info(
    asset_code: &str,
    operation_type: Option<&str>,
) -> Result<status::Custom<Json<ApiResponse<serde_json::Value>>>, status::Custom<Json<ApiResponse<()>>>> {
    let form = offramp::form::AssetInfoForm {
        asset_code,
        operation_type,
    };

    let info = offramp::get_asset_info_controller(Form::from(form))
        .await
        .map_err(|e| {
            failure(
                format!("Failed to get asset info: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Asset info fetched successfully",
        info,
        Status::Ok,
    ))
}

#[get("/utilization?<asset_code>&<account>&<jwt>")]
pub async fn check_utilization(
    asset_code: &str,
    account: Option<&str>,
    jwt: Option<&str>,
) -> Result<status::Custom<Json<ApiResponse<crate::models::models::UtilizationResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    let form = offramp::form::UtilizationForm {
        asset_code,
        account,
    };

    let utilization = offramp::check_utilization_controller(Form::from(form), jwt)
        .await
        .map_err(|e| {
            failure(
                format!("Failed to check utilization: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Utilization checked successfully",
        utilization,
        Status::Ok,
    ))
}