pub mod sep24 {
    use bigdecimal::BigDecimal;
    use diesel::prelude::*;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use std::str::FromStr;

    use crate::common::get_anchor_config_details;
    use helpers::{auth::authenticate, keypair::generate_keypair};
    use models::{
        common::establish_connection,
        schema::offramp_service::sep24_withdrawals,
        sep24::NewSep24Withdrawal,
    };

    #[derive(Error, Debug)]
    pub enum Sep24Error {
        #[error("HTTP error: {0}")]
        HttpError(#[from] reqwest::Error),

        #[error("Authentication failed")]
        AuthFailed,

        #[error("Keypair generation failed")]
        KeypairGenerationFailed,

        #[error("Invalid request: {0}")]
        InvalidRequest(String),

        #[error("Transaction not found")]
        TransactionNotFound,

        #[error("Database error: {0}")]
        DatabaseError(String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InteractiveResponse {
        #[serde(rename = "type")]
        pub response_type: String,
        pub url: String,
        pub id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Transaction {
        pub id: String,
        pub kind: String,
        pub status: String,
        pub status_eta: Option<i64>,
        pub more_info_url: Option<String>,
        pub amount_in: Option<String>,
        pub amount_out: Option<String>,
        pub amount_fee: Option<String>,
        pub started_at: String,
        pub completed_at: Option<String>,
        pub stellar_transaction_id: Option<String>,
        pub external_transaction_id: Option<String>,
        pub message: Option<String>,
        pub refunded: Option<bool>,
        // Deposit specific fields
        pub deposit_memo: Option<String>,
        pub deposit_memo_type: Option<String>,
        pub from: Option<String>,
        pub to: Option<String>,
        // Withdrawal specific fields
        pub withdraw_anchor_account: Option<String>,
        pub withdraw_memo: Option<String>,
        pub withdraw_memo_type: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionsResponse {
        pub transactions: Vec<Transaction>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DepositRequest {
        pub asset_code: String,
        pub asset_issuer: Option<String>,
        pub amount: Option<String>,
        pub account: Option<String>,
        pub memo: Option<String>,
        pub memo_type: Option<String>,
        pub email_address: Option<String>,
        #[serde(rename = "type")]
        pub deposit_type: Option<String>,
        pub wallet_name: Option<String>,
        pub wallet_url: Option<String>,
        pub lang: Option<String>,
        pub claimable_balance_supported: Option<bool>,
        pub quote_id: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WithdrawRequest {
        pub asset_code: String,
        pub asset_issuer: Option<String>,
        pub amount: Option<String>,
        pub account: Option<String>,
        pub memo: Option<String>,
        pub memo_type: Option<String>,
        pub wallet_name: Option<String>,
        pub wallet_url: Option<String>,
        pub lang: Option<String>,
        pub refund_memo: Option<String>,
        pub refund_memo_type: Option<String>,
        pub quote_id: Option<String>,
    }
    
    
    // Add this to the existing sep24 module
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssetInfo {
        pub enabled: bool,
        pub fee_fixed: Option<f64>,
        pub fee_percent: Option<f64>,
        pub min_amount: Option<f64>,
        pub max_amount: Option<f64>,
        pub fee_minimum: Option<f64>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FeaturesInfo {
        pub account_creation: Option<bool>,
        pub claimable_balances: Option<bool>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FeeInfo {
        pub enabled: bool,
        pub authentication_required: Option<bool>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InfoResponse {
        pub deposit: std::collections::HashMap<String, AssetInfo>,
        pub withdraw: std::collections::HashMap<String, AssetInfo>,
        pub fee: FeeInfo,
        pub features: FeaturesInfo,
    }
    
    // GET /info
    pub async fn get_info(slug: String, lang: Option<String>) -> Result<InfoResponse, Sep24Error> {
        let client = Client::new();
    
        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;
    
        let transfer_server = anchor_config.general_info.transfer_server.as_ref()
            .ok_or_else(|| Sep24Error::InvalidRequest("Transfer server URL not found".to_string()))?;
    
        let mut request = client.get(&format!("{}/info", transfer_server));
    
        if let Some(l) = lang {
            request = request.query(&[("lang", l)]);
        }
    
        let response = request.send().await?;
    
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error = response.text().await?;
            Err(Sep24Error::InvalidRequest(error))
        }
    }


    // POST /transactions/withdraw/interactive
    pub async fn interactive_withdraw(
        slug: String,
        account: String,
        asset_code: String,
        asset_issuer: Option<String>,
        amount: Option<String>,
        memo: Option<String>,
        memo_type: Option<String>,
        wallet_name: Option<String>,
        wallet_url: Option<String>,
        lang: Option<String>,
        refund_memo: Option<String>,
        refund_memo_type: Option<String>,
        quote_id: Option<String>,
    ) -> Result<InteractiveResponse, Sep24Error> {
        let client = Client::new();
        let keypair = match generate_keypair(&account) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep24Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;

        let jwt = authenticate(&helpers::stellartoml::AnchorService::new(), &slug, &keypair)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;

        let transfer_server = anchor_config.general_info.transfer_server_sep0024.as_ref()
            .ok_or_else(|| Sep24Error::InvalidRequest("Transfer server URL not found".to_string()))?;

        let request = WithdrawRequest {
            asset_code,
            asset_issuer,
            amount,
            account: Some(account),
            memo,
            memo_type,
            wallet_name,
            wallet_url,
            lang,
            refund_memo,
            refund_memo_type,
            quote_id,
        };

        let response = client
            .post(&format!("{}/transactions/withdraw/interactive", transfer_server))
            .bearer_auth(jwt)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let interactive_response: InteractiveResponse = response.json().await?;

            // Save to database
            let mut conn = establish_connection()
                .map_err(|e| Sep24Error::DatabaseError(e.to_string()))?;

            let new_withdrawal = NewSep24Withdrawal {
                transaction_id: interactive_response.id.clone(),
                asset_code: request.asset_code.clone(),  // Also clone this if it's not Copy
                asset_issuer: request.asset_issuer.clone(),
                amount: request.amount.clone().and_then(|a| BigDecimal::from_str(&a).ok()),
                account: request.account.clone(),
                memo: request.memo.clone(),
                memo_type: request.memo_type.clone(),
                status: "incomplete".to_string(),
                started_at: chrono::Utc::now().naive_utc(),
                completed_at: None,
                stellar_transaction_id: None,
                external_transaction_id: None,
                quote_id: request.quote_id.clone(),
                withdraw_anchor_account: None,
                withdraw_memo: request.memo.clone(),
                withdraw_memo_type: request.memo_type.clone(),
                wallet_name: request.wallet_name.clone(),
                wallet_url: request.wallet_url.clone(),
                lang: request.lang.clone(),
                refund_memo: request.refund_memo.clone(),
                refund_memo_type: request.refund_memo_type.clone(),
            };

            diesel::insert_into(sep24_withdrawals::table)
                .values(&new_withdrawal)
                .execute(&mut conn)
                .map_err(|e| Sep24Error::DatabaseError(e.to_string()))?;

            Ok(interactive_response)
        } else {
            let error = response.text().await?;
            Err(Sep24Error::InvalidRequest(error))
        }
    }

    // GET /transactions
    pub async fn get_transactions(
        slug: String,
        account: String,
        asset_code: Option<String>,
        no_older_than: Option<String>,
        limit: Option<i32>,
        kind: Option<String>,
        paging_id: Option<String>,
        lang: Option<String>,
    ) -> Result<TransactionsResponse, Sep24Error> {
        let client = Client::new();
        let keypair = match generate_keypair(&account) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep24Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;

        let jwt = authenticate(&helpers::stellartoml::AnchorService::new(), &slug, &keypair)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;

        let transfer_server = anchor_config.general_info.transfer_server_sep0024.as_ref()
            .ok_or_else(|| Sep24Error::InvalidRequest("Transfer server URL not found".to_string()))?;

        let mut request = client
            .get(&format!("{}/transactions", transfer_server))
            .bearer_auth(jwt);

        if let Some(code) = asset_code {
            request = request.query(&[("asset_code", code)]);
        }
        if let Some(time) = no_older_than {
            request = request.query(&[("no_older_than", time)]);
        }
        if let Some(lim) = limit {
            request = request.query(&[("limit", lim.to_string())]);
        }
        if let Some(k) = kind {
            request = request.query(&[("kind", k)]);
        }
        if let Some(id) = paging_id {
            request = request.query(&[("paging_id", id)]);
        }
        if let Some(l) = lang {
            request = request.query(&[("lang", l)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == 404 {
            Err(Sep24Error::TransactionNotFound)
        } else {
            Err(Sep24Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // GET /transaction
    pub async fn get_transaction(
        slug: String,
        account: String,
        id: Option<String>,
        stellar_transaction_id: Option<String>,
        external_transaction_id: Option<String>,
        lang: Option<String>,
    ) -> Result<Transaction, Sep24Error> {
        let client = Client::new();
        let keypair = match generate_keypair(&account) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep24Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;

        let jwt = authenticate(&helpers::stellartoml::AnchorService::new(), &slug, &keypair)
            .await
            .map_err(|_| Sep24Error::AuthFailed)?;

        let transfer_server = anchor_config.general_info.transfer_server_sep0024.as_ref()
            .ok_or_else(|| Sep24Error::InvalidRequest("Transfer server URL not found".to_string()))?;

        let mut request = client
            .get(&format!("{}/transaction", transfer_server))
            .bearer_auth(jwt);

        if let Some(i) = id {
            request = request.query(&[("id", i)]);
        }
        if let Some(stellar_id) = stellar_transaction_id {
            request = request.query(&[("stellar_transaction_id", stellar_id)]);
        }
        if let Some(external_id) = external_transaction_id {
            request = request.query(&[("external_transaction_id", external_id)]);
        }
        if let Some(l) = lang {
            request = request.query(&[("lang", l)]);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == 404 {
            Err(Sep24Error::TransactionNotFound)
        } else {
            Err(Sep24Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }
}