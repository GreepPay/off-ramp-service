// SEP31
// Helper Methods:
// Post transactions 
// Update transactions 
// Get single transactions 
// PUT Transaction Callback
// 



pub mod sep31 {
    use bigdecimal::BigDecimal;
    use diesel::prelude::*;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use crate::common::get_anchor_config_details;
    use helpers::{auth::authenticate, keypair::generate_keypair};
    use models::{
        common::establish_connection,
        schema::offramp_service::sep31_transactions,
        sep31::NewSep31Transaction,
    };

    #[derive(Error, Debug)]
    pub enum Sep31Error {
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
    pub struct AssetInfo {
        pub asset: String,
        pub quotes_supported: bool,
        pub quotes_required: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_fixed: Option<BigDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_percent: Option<BigDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_amount: Option<BigDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_amount: Option<BigDecimal>,
        pub funding_methods: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InfoResponse {
        pub receive: Vec<AssetInfo>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionRequest {
        pub amount: BigDecimal,
        pub asset_code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_issuer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub destination_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_id: Option<String>,
        pub sender_id: String,
        pub receiver_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lang: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refund_memo: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refund_memo_type: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionResponse {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_memo_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_memo: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Transaction {
        pub id: String,
        pub status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status_eta: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status_message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount_in: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount_in_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount_out: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount_out_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount_fee: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount_fee_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_details: Option<FeeDetails>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_memo_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_memo: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub started_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub completed_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stellar_transaction_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_transaction_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub refunds: Option<Refunds>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required_info_message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required_info_updates: Option<serde_json::Value>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FeeDetails {
        pub total: String,
        pub asset: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub details: Option<Vec<FeeComponent>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FeeComponent {
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        pub amount: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Refunds {
        pub amount_refunded: String,
        pub amount_fee: String,
        pub payments: Vec<RefundPayment>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RefundPayment {
        pub id: String,
        pub amount: String,
        pub fee: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CallbackRequest {
        pub url: String,
    }

    // 1. GET /info
    pub async fn get_info(slug: &str) -> Result<InfoResponse, Sep31Error> {
        let client = Client::new();

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
            .await
            .map_err(|_| Sep31Error::AuthFailed)?;

        let direct_payment_server = &anchor_config.general_info.direct_payment_server;
        let direct_payment_server_str = direct_payment_server.as_ref().map_or_else(
            || "".to_string(),
            |s| s.to_string(),
        );

        let response = client
            .get(&format!("{}/info", direct_payment_server_str))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Sep31Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // 2. POST /transactions
    pub async fn create_transaction(
        slug: &str,
        account: &str,
        request: TransactionRequest,
    ) -> Result<TransactionResponse, Sep31Error> {
        let client = Client::new();

        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep31Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
            .await
            .map_err(|_| Sep31Error::AuthFailed)?;

        let jwt = match authenticate(
            &helpers::stellartoml::AnchorService::new(),
            slug,
            account,
            &keypair,
        )
        .await
        {
            Ok(token) => token,
            Err(_) => return Err(Sep31Error::AuthFailed),
        };

        let direct_payment_server = &anchor_config.general_info.direct_payment_server;
        let direct_payment_server_str = direct_payment_server.as_ref().map_or_else(
            || "".to_string(),
            |s| s.to_string(),
        );

        let response = client
            .post(&format!("{}/transactions", direct_payment_server_str))
            .bearer_auth(jwt)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let transaction_response: TransactionResponse = response.json().await?;

            // Save to database
            let mut conn = establish_connection().map_err(|e| Sep31Error::DatabaseError(e.to_string()))?;

            let new_transaction = NewSep31Transaction {
                transaction_id: transaction_response.id.clone(),
                account: account.to_string(),
                amount: request.amount,
                asset_code: request.asset_code,
                asset_issuer: request.asset_issuer,
                destination_asset: request.destination_asset,
                quote_id: request.quote_id,
                sender_id: request.sender_id,
                receiver_id: request.receiver_id,
                stellar_account_id: transaction_response.stellar_account_id.clone(),
                stellar_memo_type: transaction_response.stellar_memo_type.clone(),
                stellar_memo: transaction_response.stellar_memo.clone(),
                status: "pending_sender".to_string(),
            };

            diesel::insert_into(sep31_transactions::table)
                .values(&new_transaction)
                .execute(&mut conn)
                .map_err(|e| Sep31Error::DatabaseError(e.to_string()))?;

            Ok(transaction_response)
        } else {
            let error = response.text().await?;
            Err(Sep31Error::InvalidRequest(error))
        }
    }

    // 3. GET /transactions/:id
    pub async fn get_transaction(
        slug: &str,
        account: &str,
        transaction_id: &str,
    ) -> Result<Transaction, Sep31Error> {
        let client = Client::new();

        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep31Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
            .await
            .map_err(|_| Sep31Error::AuthFailed)?;

        let jwt = match authenticate(
            &helpers::stellartoml::AnchorService::new(),
            slug,
            account,
            &keypair,
        )
        .await
        {
            Ok(token) => token,
            Err(_) => return Err(Sep31Error::AuthFailed),
        };

        let direct_payment_server = &anchor_config.general_info.direct_payment_server;
        let direct_payment_server_str = direct_payment_server.as_ref().map_or_else(
            || "".to_string(),
            |s| s.to_string(),
        );

        let response = client
            .get(&format!("{}/transactions/{}", direct_payment_server_str, transaction_id))
            .bearer_auth(jwt)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == 404 {
            Err(Sep31Error::TransactionNotFound)
        } else {
            Err(Sep31Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // 4. PATCH /transactions/:id
    pub async fn update_transaction(
        slug: &str,
        account: &str,
        transaction_id: &str,
        fields: serde_json::Value,
    ) -> Result<Transaction, Sep31Error> {
        let client = Client::new();

        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep31Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
            .await
            .map_err(|_| Sep31Error::AuthFailed)?;

        let jwt = match authenticate(
            &helpers::stellartoml::AnchorService::new(),
            slug,
            account,
            &keypair,
        )
        .await
        {
            Ok(token) => token,
            Err(_) => return Err(Sep31Error::AuthFailed),
        };

        let direct_payment_server = &anchor_config.general_info.direct_payment_server;
        let direct_payment_server_str = direct_payment_server.as_ref().map_or_else(
            || "".to_string(),
            |s| s.to_string(),
        );

        let response = client
            .patch(&format!("{}/transactions/{}", direct_payment_server_str, transaction_id))
            .bearer_auth(jwt)
            .json(&fields)
            .send()
            .await?;

        if response.status().is_success() {
            let transaction: Transaction = response.json().await?;
            let mut conn = establish_connection().map_err(|e| Sep31Error::DatabaseError(e.to_string()))?;

            diesel::update(sep31_transactions::table)
                .filter(sep31_transactions::transaction_id.eq(transaction_id))
                .set((
                    sep31_transactions::status.eq(transaction.status.clone()),
                    sep31_transactions::stellar_transaction_id.eq(transaction.stellar_transaction_id.clone()),
    
                ))
                .execute(&mut conn)
                .map_err(|e| Sep31Error::DatabaseError(e.to_string()))?;

            Ok(transaction)
        } else if response.status() == 404 {
            Err(Sep31Error::TransactionNotFound)
        } else {
            let error = response.text().await?;
            Err(Sep31Error::InvalidRequest(error))
        }
    }

    // 5. PUT /transactions/:id/callback
    pub async fn set_transaction_callback(
        slug: &str,
        account: &str,
        transaction_id: &str,
        callback_url: &str,
    ) -> Result<(), Sep31Error> {
        let client = Client::new();

        let keypair = match generate_keypair() {
            Ok(kp) => kp,
            Err(_) => return Err(Sep31Error::KeypairGenerationFailed),
        };

        let anchor_config = get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), slug)
            .await
            .map_err(|_| Sep31Error::AuthFailed)?;

        let jwt = match authenticate(
            &helpers::stellartoml::AnchorService::new(),
            slug,
            account,
            &keypair,
        )
        .await
        {
            Ok(token) => token,
            Err(_) => return Err(Sep31Error::AuthFailed),
        };

        let direct_payment_server = &anchor_config.general_info.direct_payment_server;
        let direct_payment_server_str = direct_payment_server.as_ref().map_or_else(
            || "".to_string(),
            |s| s.to_string(),
        );

        let response = client
            .put(&format!("{}/transactions/{}/callback", direct_payment_server_str, transaction_id))
            .bearer_auth(jwt)
            .json(&CallbackRequest {
                url: callback_url.to_string(),
            })
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else if response.status() == 404 {
            Err(Sep31Error::TransactionNotFound)
        } else {
            let error = response.text().await?;
            Err(Sep31Error::InvalidRequest(error))
        }
    }
}