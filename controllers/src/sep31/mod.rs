use rocket::serde::json::Json;

use services::sep31::sep31::{get_info, create_transaction, get_transaction, update_transaction, set_transaction_callback, InfoResponse, TransactionResponse, TransactionRequest, Transaction};
use form::form::{ Sep31InfoForm, Sep31TransactionRequestForm, Sep31GetTransactionForm, Sep31UpdateTransactionForm, Sep31SetTransactionCallbackForm};
pub mod form;

pub async fn get_sep31_info(
    data: Json<Sep31InfoForm>,
) -> Result<InfoResponse, Box<dyn std::error::Error>> {
    Ok(get_info(
        data.slug.clone().as_str(),
    ).await?)
}

pub async fn create_sep31_transaction(
    data: Json<Sep31TransactionRequestForm>,
) -> Result<TransactionResponse, Box<dyn std::error::Error>> {
    Ok(create_transaction(
        data.slug.clone().as_str(),
        data.account.clone().as_str(),
        TransactionRequest {
            amount: data.amount.parse().unwrap(),
            asset_code: data.asset_code.clone(),
            asset_issuer: data.asset_issuer.clone(),
            destination_asset: data.destination_asset.clone(),
            quote_id: data.quote_id.clone(),
            sender_id: data.sender_id.clone(),
            receiver_id: data.receiver_id.clone(),
            lang: data.lang.clone(),
            refund_memo: data.refund_memo.clone(),
            refund_memo_type: data.refund_memo_type.clone(),
        }
    ).await?)
}

pub async fn get_sep31_transaction(
    data: Json<Sep31GetTransactionForm>,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    Ok(get_transaction(
        data.slug.clone().as_str(),
        data.account.clone().as_str(),
        data.transaction_id.clone().as_str(),
    ).await?)
}

pub async fn update_sep31_transaction(
    data: Json<Sep31UpdateTransactionForm>,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let fields: serde_json::Value = serde_json::from_str(&data.fields)?;
    Ok(update_transaction(
        data.slug.clone().as_str(),
        data.account.clone().as_str(),
        data.transaction_id.clone().as_str(),
        fields,
    ).await?)
}

pub async fn set_sep31_transaction_callback(
    data: Json<Sep31SetTransactionCallbackForm>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(set_transaction_callback(
        data.slug.clone().as_str(),
        data.account.clone().as_str(),
        data.transaction_id.clone().as_str(),
        data.callback_url.clone().as_str(),
    ).await?)
}
