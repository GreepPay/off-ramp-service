use rocket::serde::json::Json;

use services::sep31::sep31::{get_info, create_transaction, get_transaction, update_transaction, set_transaction_callback, Sep31Info, TransactionResponse};
use form::form::{ Sep31InfoForm, Sep31TransactionRequestForm, Sep31GetTransactionForm, Sep31UpdateTransactionForm, Sep31SetTransactionCallbackForm };
pub mod form;

pub async fn get_info(
    data: Json<Sep31InfoForm>,
) -> Result<Vec<Sep31Info>, Box<dyn std::error::Error>> {
    Ok(get_info(
        data.slug.clone()
    ).await?)
}

pub async fn create_transaction(
    data: Json<Sep31TransactionRequestForm>,
) -> Result<TransactionResponse, Box<dyn std::error::Error>> {
    Ok(create_transaction(
        data.slug.clone(),
        data.account.clone(),
        data.amount.clone(),
        data.asset_code.clone(),
        data.asset_issuer.clone(),
        data.destination_asset.clone(),
        data.quote_id.clone(),
        data.sender_id.clone(),
        data.receiver_id.clone(),
        data.lang.clone(),
        data.refund_memo.clone(),
        data.refund_memo_type.clone(),
    ).await?)
}

pub async fn get_transaction(
    data: Json<Sep31GetTransactionForm>,
) -> Result<Sep31TransactionResponse, Box<dyn std::error::Error>> {
    Ok(get_transaction(
        data.slug.clone(),
        data.account.clone(),
        data.transaction_id.clone(),
    ).await?)
}

pub async fn update_transaction(
    data: Json<Sep31UpdateTransactionForm>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(update_transaction(
        data.slug.clone(),
        data.account.clone(),
        data.transaction_id.clone(),
        data.fields.clone(),
    ).await?)
}

pub async fn set_transaction_callback(
    data: Json<Sep31SetTransactionCallbackForm>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(set_transaction_callback(
        data.slug.clone(),
        data.account.clone(),
        data.transaction_id.clone(),
        data.callback_url.clone(),
    ).await?)
}