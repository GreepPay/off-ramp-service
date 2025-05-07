pub mod sep6{
use std::collections::HashMap;
use bigdecimal::BigDecimal;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
use diesel::RunQueryDsl; 

use helpers::{
    auth::authenticate,
    keypair::generate_keypair
};

use crate::common::get_anchor_config_details;


use models::{
    common::establish_connection,
    sep6::{Sep6Transaction, NewSep6Transaction},
    schema::offramp_service::sep6_transactions,
};

#[derive(Error, Debug)]
pub enum Sep6Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Transaction not found")]
    TransactionNotFound,

    #[error("Anchor not supported")]
    AnchorNotSupported,

    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositResponse {
    pub how: Option<String>,
    pub instructions: Option<HashMap<String, InstructionField>>,
    pub id: Option<String>,
    pub eta: Option<i32>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_fixed: Option<f64>,
    pub fee_percent: Option<f64>,
    pub extra_info: Option<ExtraInfo>,
}
#[derive(Error, Debug)]
pub enum KeyPairError {
    #[error("Generation failed")]
    GenerationFailed,

    #[error("Invalid key")]
    InvalidKey,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid format")]
    InvalidFormat,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionField {
    pub value: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraInfo {
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawResponse {
    pub account_id: Option<String>,
    pub memo_type: Option<String>,
    pub memo: Option<String>,
    pub id: Option<String>,
    pub eta: Option<i32>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_fixed: Option<f64>,
    pub fee_percent: Option<f64>,
    pub extra_info: Option<ExtraInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub deposit: HashMap<String, DepositAssetInfo>,
    #[serde(rename = "deposit-exchange")]
    pub deposit_exchange: HashMap<String, ExchangeAssetInfo>,
    pub withdraw: HashMap<String, WithdrawAssetInfo>,
    #[serde(rename = "withdraw-exchange")]
    pub withdraw_exchange: HashMap<String, ExchangeAssetInfo>,
    pub fee: FeeInfo,
    pub transactions: EndpointInfo,
    pub transaction: EndpointInfo,
    pub features: Features,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositAssetInfo {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,
    pub funding_methods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_fixed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawAssetInfo {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,
    pub funding_methods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_fixed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeAssetInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_required: Option<bool>,
    pub funding_methods: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeInfo {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Features {
    pub account_creation: bool,
    pub claimable_balances: bool,
}


    // 1. GET /info
    pub async fn get_anchor_info( slug: &str) -> Result<InfoResponse, Sep6Error> {
        let client = Client::new();
        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug).await
            .map_err(|_| Sep6Error::AuthFailed)?;
        let transfer_server = &anchor_config.general_info.transfer_server;
        // Unwrap the Option or provide a default value
        let transfer_server_str = transfer_server.as_ref().map_or_else(
            || "".to_string(),  // Default value if None
            |s| s.to_string()   // Use the string value if Some
        );

        let url = format!("{}/info", transfer_server_str);
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Sep6Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 2. GET /transactions
    pub async fn get_transactions(    
        slug: &str,
        account: &str,
        asset_code: Option<&str>,
        no_older_than: Option<&str>,
        limit: Option<i32>,
        kind: Option<Vec<&str>>,
        paging_id: Option<&str>,
    ) -> Result<Vec<Sep6Transaction>, Sep6Error> {
        let client = Client::new();
      
        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug).await
            .map_err(|_| Sep6Error::AuthFailed)?;
        let transfer_server = &anchor_config.general_info.transfer_server;
        // Unwrap the Option or provide a default value
        let transfer_server_str = transfer_server.as_ref().map_or_else(
            || "".to_string(),  // Default value if None
            |s| s.to_string()   // Use the string value if Some
        );
    
        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };
        let jwt = match authenticate(&helpers::stellartoml::AnchorService::new(),slug,account, &keypair).await {
            Ok(token) => token,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };

        let mut request = client
            .get(&format!("{}/transactions",  transfer_server_str))
            .bearer_auth(jwt);

        if let Some(code) = asset_code {
            request = request.query(&[("asset_code", code)]);
        }

        if let Some(date) = no_older_than {
            request = request.query(&[("no_older_than", date)]);
        }

        if let Some(lim) = limit {
            request = request.query(&[("limit", lim)]);
        }

        if let Some(kinds) = kind {
            for k in kinds {
                request = request.query(&[("kind", k)]);
            }
        }

        if let Some(id) = paging_id {
            request = request.query(&[("paging_id", id)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let transactions: Vec<Sep6Transaction> = response.json().await?;

            // Save to database
            let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

            for tx in &transactions {
                let new_tx = NewSep6Transaction {
                    transaction_id: tx.transaction_id.clone(),
                    kind: tx.kind.clone(),
                    status: tx.status.clone(),
                    status_eta: tx.status_eta,
                    more_info_url: tx.more_info_url.clone(),
                    amount_in: tx.amount_in.clone(),
                    amount_in_asset: tx.amount_in_asset.clone(),
                    amount_out: tx.amount_out.clone(),
                    amount_out_asset: tx.amount_out_asset.clone(),
                    amount_fee: tx.amount_fee.clone(),
                    amount_fee_asset: tx.amount_fee_asset.clone(),
                    fee_details: tx.fee_details.clone(),
                    quote_id: tx.quote_id.clone(),
                    from: tx.from.clone(),
                    to: tx.to.clone(),
                    external_extra: tx.external_extra.clone(),
                    external_extra_text: tx.external_extra_text.clone(),
                    deposit_memo: tx.deposit_memo.clone(),
                    deposit_memo_type: tx.deposit_memo_type.clone(),
                    withdraw_anchor_account: tx.withdraw_anchor_account.clone(),
                    withdraw_memo: tx.withdraw_memo.clone(),
                    withdraw_memo_type: tx.withdraw_memo_type.clone(),
                    started_at: tx.started_at,
                    updated_at: tx.updated_at,
                    completed_at: tx.completed_at,
                    user_action_required_by: tx.user_action_required_by,
                    stellar_transaction_id: tx.stellar_transaction_id.clone(),
                    external_transaction_id: tx.external_transaction_id.clone(),
                    message: tx.message.clone(),
                    refunded: tx.refunded,
                    refunds: tx.refunds.clone(),
                    required_info_message: tx.required_info_message.clone(),
                    required_info_updates: tx.required_info_updates.clone(),
                    instructions: tx.instructions.clone(),
                    claimable_balance_id: tx.claimable_balance_id.clone(),

                };

                diesel::insert_into(sep6_transactions::table)
                    .values(&new_tx)
                    .on_conflict(sep6_transactions::transaction_id)
                    .do_update()
                    .set(&new_tx)
                    .execute(&mut conn)
                    .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;
            }

            Ok(transactions)
        } else if response.status() == 404 {
            Err(Sep6Error::TransactionNotFound)
        } else {
            Err(Sep6Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 3. GET /transaction
    pub async fn get_transaction(
      
        slug: &str,
        account: &str,
        id: Option<&str>,
        stellar_transaction_id: Option<&str>,
        external_transaction_id: Option<&str>,
    ) -> Result<Sep6Transaction, Sep6Error> {
        let client = Client::new();
        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };//trying to use keypair error, buggy, will be back
        
        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug).await
            .map_err(|_| Sep6Error::AuthFailed)?;
        let transfer_server = &anchor_config.general_info.transfer_server;
        // Unwrap the Option or provide a default value
        let transfer_server_str = transfer_server.as_ref().map_or_else(
            || "".to_string(),  // Default value if None
            |s| s.to_string()   // Use the string value if Some
        );
    
        let jwt = match authenticate(&helpers::stellartoml::AnchorService::new(),slug,account, &keypair).await {
            Ok(token) => token,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };

        let mut request = client
            .get(&format!("{}/transaction", transfer_server_str))
            .bearer_auth(jwt);

        if let Some(tx_id) = id {
            request = request.query(&[("id", tx_id)]);
        }

        if let Some(stellar_id) = stellar_transaction_id {
            request = request.query(&[("stellar_transaction_id", stellar_id)]);
        }

        if let Some(external_id) = external_transaction_id {
            request = request.query(&[("external_transaction_id", external_id)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let tx: Sep6Transaction = response.json().await?;

            // Save to database
            let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

            let new_tx = NewSep6Transaction {
                transaction_id: tx.transaction_id.clone(),
                kind: tx.kind.clone(),
                status: tx.status.clone(),
                status_eta: tx.status_eta,
                more_info_url: tx.more_info_url.clone(),
                amount_in: tx.amount_in.clone(),
                amount_in_asset: tx.amount_in_asset.clone(),
                amount_out: tx.amount_out.clone(),
                amount_out_asset: tx.amount_out_asset.clone(),
                amount_fee: tx.amount_fee.clone(),
                amount_fee_asset: tx.amount_fee_asset.clone(),
                fee_details: tx.fee_details.clone(),
                quote_id: tx.quote_id.clone(),
                from: tx.from.clone(),
                to: tx.to.clone(),
                external_extra: tx.external_extra.clone(),
                external_extra_text: tx.external_extra_text.clone(),
                deposit_memo: tx.deposit_memo.clone(),
                deposit_memo_type: tx.deposit_memo_type.clone(),
                withdraw_anchor_account: tx.withdraw_anchor_account.clone(),
                withdraw_memo: tx.withdraw_memo.clone(),
                withdraw_memo_type: tx.withdraw_memo_type.clone(),
                started_at: tx.started_at,
                updated_at: tx.updated_at,
                completed_at: tx.completed_at,
                user_action_required_by: tx.user_action_required_by,
                stellar_transaction_id: tx.stellar_transaction_id.clone(),
                external_transaction_id: tx.external_transaction_id.clone(),
                message: tx.message.clone(),
                refunded: tx.refunded,
                refunds: tx.refunds.clone(),
                required_info_message: tx.required_info_message.clone(),
                required_info_updates: tx.required_info_updates.clone(),
                instructions: tx.instructions.clone(),
                claimable_balance_id: tx.claimable_balance_id.clone(),
            };

            diesel::insert_into(sep6_transactions::table)
                .values(&new_tx)
                .on_conflict(sep6_transactions::transaction_id)
                .do_update()
                .set(&new_tx)
                .execute(&mut conn)
                .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

            Ok(tx)
        } else if response.status() == 404 {
            Err(Sep6Error::TransactionNotFound)
        } else {
            Err(Sep6Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 4. GET /withdraw-exchange
    pub async fn get_withdraw_exchange(
   
        slug: &str,
        account: &str,
        source_asset: &str,
        destination_asset: &str,
        amount: &str,
        quote_id: Option<&str>,
        funding_method: &str,
        memo: Option<&str>,
        on_change_callback: Option<&str>,
        country_code: Option<&str>,
        refund_memo: Option<&str>,
        refund_memo_type: Option<&str>,
    ) -> Result<WithdrawResponse, Sep6Error> {
        let client = Client::new();
        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };//trying to use keypair error, buggy, will be back
        
        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug).await
            .map_err(|_| Sep6Error::AuthFailed)?;
        let transfer_server = &anchor_config.general_info.transfer_server;
        // Unwrap the Option or provide a default value
        let transfer_server_str = transfer_server.as_ref().map_or_else(
            || "".to_string(),  // Default value if None
            |s| s.to_string()   // Use the string value if Some
        );
        let jwt = match authenticate(&helpers::stellartoml::AnchorService::new(),slug,account, &keypair).await{
            Ok(token) => token,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };

        let mut request = client
            .get(&format!("{}/withdraw-exchange", transfer_server_str))
            .bearer_auth(jwt)
            .query(&[
                ("source_asset", source_asset),
                ("destination_asset", destination_asset),
                ("amount", amount),
                ("funding_method", funding_method),
            ]);

        if let Some(qid) = quote_id {
            request = request.query(&[("quote_id", qid)]);
        }

        if let Some(m) = memo {
            request = request.query(&[("memo", m)]);
        }

        if let Some(cb) = on_change_callback {
            request = request.query(&[("on_change_callback", cb)]);
        }

        if let Some(code) = country_code {
            request = request.query(&[("country_code", code)]);
        }

        if let Some(rm) = refund_memo {
            request = request.query(&[("refund_memo", rm)]);
        }

        if let Some(rmt) = refund_memo_type {
            request = request.query(&[("refund_memo_type", rmt)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let withdraw_response: WithdrawResponse = response.json().await?;

            // Save transaction to database if id is provided
            if let Some(tx_id) = &withdraw_response.id {
                let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

                let new_tx = NewSep6Transaction {
                    transaction_id: tx_id.clone(),
                    kind: "withdraw-exchange".to_string(),
                    status: "pending_user_transfer_start".to_string(), // Default status
                    status_eta: None,
                    more_info_url: None,
                    amount_in: Some(BigDecimal::from_str(amount).map_err(|_| Sep6Error::InvalidRequest("Invalid amount".to_string()))?),
                    amount_in_asset: Some(source_asset.to_string()),
                    amount_out: None, 
                    amount_out_asset: Some(destination_asset.to_string()),
                    amount_fee: None, 
                    amount_fee_asset: None,
                    fee_details: None,
                    quote_id: quote_id.map(|s| s.to_string()),
                    from: None,
                    to: None,
                    external_extra: None,
                    external_extra_text: None,
                    deposit_memo: None,
                    deposit_memo_type: None,
                    withdraw_anchor_account: None,
                    withdraw_memo: None,
                    withdraw_memo_type: None, 
                    started_at: Some(Utc::now().naive_utc()),
                    updated_at: None,
                    completed_at: None,
                    user_action_required_by: None,
                    stellar_transaction_id: None,
                    external_transaction_id: None,
                    message: None,
                    refunded: None,
                    refunds: None,
                    required_info_message: None,
                    required_info_updates: None,
                    instructions: None,
                    claimable_balance_id: None,

                };

                diesel::insert_into(sep6_transactions::table)
                    .values(&new_tx)
                    .execute(&mut conn)
                    .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;
            }

            Ok(withdraw_response)
        } else {
            Err(Sep6Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 5. GET /withdraw
    pub async fn get_withdraw(
        slug: &str,
        account: &str,
        asset_code: &str,
        funding_method: &str,
        memo: Option<&str>,
        on_change_callback: Option<&str>,
        amount: Option<&str>,
        country_code: Option<&str>,
        refund_memo: Option<&str>,
        refund_memo_type: Option<&str>,
    ) -> Result<WithdrawResponse, Sep6Error> {
        let client = Client::new();
        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };
        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug).await
            .map_err(|_| Sep6Error::AuthFailed)?;
        let transfer_server = &anchor_config.general_info.transfer_server;
        // Unwrap the Option or provide a default value
        let transfer_server_str = transfer_server.as_ref().map_or_else(
            || "".to_string(),  // Default value if None
            |s| s.to_string()   // Use the string value if Some
        );
        let jwt = match authenticate(&helpers::stellartoml::AnchorService::new(),slug,account, &keypair).await {
            Ok(token) => token,
            Err(_) => return Err(Sep6Error::AuthFailed),
        };

        let mut request = client
            .get(&format!("{}/withdraw",transfer_server_str))
            .bearer_auth(jwt)
            .query(&[
                ("asset_code", asset_code),
                ("funding_method", funding_method),
            ]);

        if let Some(m) = memo {
            request = request.query(&[("memo", m)]);
        }

        if let Some(cb) = on_change_callback {
            request = request.query(&[("on_change_callback", cb)]);
        }

        if let Some(amt) = amount {
            request = request.query(&[("amount", amt)]);
        }

        if let Some(code) = country_code {
            request = request.query(&[("country_code", code)]);
        }

        if let Some(rm) = refund_memo {
            request = request.query(&[("refund_memo", rm)]);
        }

        if let Some(rmt) = refund_memo_type {
            request = request.query(&[("refund_memo_type", rmt)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let withdraw_response: WithdrawResponse = response.json().await?;

            // Save transaction to database if id is provided
            if let Some(tx_id) = &withdraw_response.id {
                let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

                let new_tx = NewSep6Transaction {
                    transaction_id: tx_id.clone(),
                    kind: "withdraw".to_string(),
                    status: "pending_user_transfer_start".to_string(), // Default status
                    status_eta: None,
                    more_info_url: None,
                    amount_in: amount.map(|a| BigDecimal::from_str(a).map_err(|_| Sep6Error::InvalidRequest("Invalid amount".to_string()))).transpose()?,
                    amount_in_asset: Some(asset_code.to_string()),
                    amount_out: None, 
                    amount_out_asset: None,
                    amount_fee: None, 
                    amount_fee_asset: None,
                    fee_details: None,
                    quote_id: None,
                    from: None,
                    to: None,
                    external_extra: None,
                    external_extra_text: None,
                    deposit_memo: None,
                    deposit_memo_type: None,
                    withdraw_anchor_account: withdraw_response.account_id.clone(),
                    withdraw_memo: withdraw_response.memo.clone(),
                    withdraw_memo_type: withdraw_response.memo_type.clone(),
                    started_at: Some(Utc::now().naive_utc()),
                    updated_at: None,
                    completed_at: None,
                    user_action_required_by: None,
                    stellar_transaction_id: None,
                    external_transaction_id: None,
                    message: None,
                    refunded: None,
                    refunds: None,
                    required_info_message: None,
                    required_info_updates: None,
                    instructions: None,
                    claimable_balance_id: None,

                };

                diesel::insert_into(sep6_transactions::table)
                    .values(&new_tx)
                    .execute(&mut conn)
                    .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;
            }

            Ok(withdraw_response)
        } else {
            Err(Sep6Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }
}
