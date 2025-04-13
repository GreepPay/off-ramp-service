use rocket::{get, post, routes};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::form::Form;
use controllers::{
    api::api::{failure, success, ApiResponse},
    offramp,
};

use rocket::form::Form;
use stellar_base::KeyPair;
use services::offrampService::{offramp_funds, get_transaction, check_transaction_status};



pub mod form;

pub async fn offramp_funds_controller(
    data: Form<form::OfframpForm<'_>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let keypair = KeyPair::from_secret(data.secret_key)?;
    let response = offramp_funds(
        data.account_id,
        &keypair,
        data.amount,
        data.dest_currency,
        data.kyc_fields.clone(),
    ).await?;
    Ok(response.id)
}

pub async fn check_transaction_status_controller(
    data: Form<form::TransactionStatusForm<'_>>,
) -> Result<TransactionStatus, Box<dyn std::error::Error>> {
    let keypair = KeyPair::from_secret(data.secret_key)?;
    let status = check_transaction_status(
        data.transaction_id,
        data.account_id,
        &keypair,
    ).await?;
    
    Ok(status)
}

pub async fn get_transactions_controller(
    data: Form<form::TransactionQueryForm<'_>>,
) -> Result<Vec<TransactionStatus>, Box<dyn std::error::Error>> {
    let keypair = KeyPair::from_secret(data.secret_key)?;
    let transactions = get_transactions(
        data.account_id,
        &keypair,
    ).await?;
    
    Ok(transactions)
}

pub async fn get_transaction_controller(
    data: Form<form::SingleTransactionQueryForm<'_>>,
) -> Result<TransactionStatus, Box<dyn std::error::Error>> {
    let keypair = KeyPair::from_secret(data.secret_key)?;
    let offramp_service = OfframpService::new();
    
    let transaction = offramp_service.get_transaction(
        data.transaction_id,
        data.account_id,
        &keypair,
    ).await?;
    
    Ok(transaction)
}

pub async fn get_asset_info_controller(
    data: Form<form::AssetInfoForm<'_>>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let transfer_service = TransferService::new();
    
    let info = transfer_service.get_info(&format!(
        "{}?asset={}&type={}",
        transfer_service.base_url(),
        data.asset_code,
        data.operation_type.unwrap_or("withdraw")
    )).await?;
    
    Ok(info)
}

pub async fn check_utilization_controller(
    data: Form<form::UtilizationForm<'_>>,
    jwt: Option<&str>,
) -> Result<UtilizationResponse, Box<dyn std::error::Error>> {
    let utilization_service = UtilizationService::new();
    
    let response = utilization_service.check_utilization(
        data.asset_code,
        data.account,
        jwt,
    ).await?;
    
    Ok(response)
}