// services/sep6.rs
use std::str::FromStr;
use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use stellar_base::KeyPair;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::str::FromStr;
use crate::kyc::CustomerQuery;

use models::{
    common::establish_connection, schema::offramp_service::{sep38_quotes, sep6_transactions},
    withdrawal::{NewSep6Transaction, Sep6Transaction}
};
use crate::{sep10auth::StellarAuth, kyc::Sep12Service, info::Sep38Client};



#[derive(Error, Debug)]
pub enum Sep6Error {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("KYC required: {0}")]
    KYCRequired(String),

    #[error("Quote error: {0}")]
    QuoteError(String),

    #[error("Transaction not found")]
    TransactionNotFound,

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}





#[derive(Debug, Serialize, Deserialize)]

pub struct WithdrawRequest {
    pub asset_code: String,
    pub account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawResponse {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>, 

    pub id: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_fixed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<f64>,
}

#[derive(AsChangeset)]
#[diesel(table_name = sep6_transactions)]
pub struct TransactionUpdate<'a> {
    status: &'a str,
    updated_at: chrono::NaiveDateTime,
    stellar_transaction_id: Option<&'a str>,
    external_transaction_id: Option<&'a str>,
    completed_at: Option<chrono::NaiveDateTime>,
}


pub struct Sep6Service {
    auth: StellarAuth,
    sep12: Sep12Service,
    sep38: Sep38Client,
    signing_key: KeyPair,
}

static SEP6_SERVICE: Lazy<Arc<Sep6Service>> = Lazy::new(|| {
    // Initialize dependencies
    let auth = StellarAuth::global(); // Assuming StellarAuth also uses global pattern
    let sep12 = Sep12Service::global();
    let sep38 = Sep38Client::global();
    let signing_key = KeyPair::from_secret_seed(&std::env::var("SIGNING_KEY_SEED").expect("SIGNING_KEY_SEED must be set"))
        .expect("Invalid signing key");
    
    Arc::new(Sep6Service::new(auth, sep12, sep38, signing_key))
});

impl Sep6Service {
    pub fn new(auth: StellarAuth, sep12: Sep12Service, sep38: Sep38Client, signing_key: KeyPair) -> Self {
        Self {
            auth,
            sep12,
            sep38,
            // Remove the http_client initialization since the field has been removed
            signing_key,
        }
    }
    pub fn global() -> Arc<Self> {
          SEP6_SERVICE.clone()
    }
    pub async fn withdraw(
        &self,
        request: WithdrawRequest,
        auth_token: &str,
    ) -> Result<WithdrawResponse, Sep6Error> {
        // Verify authentication
        let account = self.auth.verify_jwt(auth_token).map_err(|e| Sep6Error::AuthFailed(e.to_string()))?;

        if account != request.account {
            return Err(Sep6Error::AuthFailed("Account mismatch".into()));
        }

        // Check KYC status
        let kyc_status = self.sep12.get_customer(CustomerQuery {
            id: request.id.clone(),
            customer_type: request.customer_type.clone(),
            account: request.account.clone(),
            memo: request.memo.clone(),
            memo_type: request.memo_type.clone(),
            transaction_id: None,
            lang: None,
        }).await;

        if let Err(_) = kyc_status {
            return Err(Sep6Error::KYCRequired("Customer information needed".into()));
        }

        let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        // Create transaction record
        let transaction_id = Uuid::new_v4().to_string();
        let new_transaction = NewSep6Transaction {
            transaction_id: transaction_id.clone(),
            kind: "withdrawal".to_string(),
            status: "pending_user_transfer_start".to_string(),
            account: request.account.clone(),
            memo: request.memo.clone(),
            memo_type: request.memo_type.clone(),
            quote_id: request.quote_id.clone(),
            amount_in: request.amount.as_ref().and_then(|a| BigDecimal::from_str(a).ok()),
            amount_in_asset: Some(request.asset_code.clone()),
            amount_out: None,
            amount_out_asset: None,
        };

        diesel::insert_into(sep6_transactions::table)
            .values(&new_transaction)
            .execute(&mut conn)
            .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        // If quote was provided, validate it
        if let Some(quote_id) = request.quote_id {
            let quote = self.sep38.get_quote(&quote_id, auth_token).await
                .map_err(|e| Sep6Error::QuoteError(e.to_string()))?;

            // Update transaction with quote details
            diesel::update(sep6_transactions::table.filter(sep6_transactions::transaction_id.eq(&transaction_id)))
                .set((
                    sep6_transactions::amount_in.eq(BigDecimal::from_str(&quote.sell_amount)
                        .map_err(|_| Sep6Error::InvalidRequest("Invalid sell amount".into()))?),
                    sep6_transactions::amount_out.eq(BigDecimal::from_str(&quote.buy_amount)
                        .map_err(|_| Sep6Error::InvalidRequest("Invalid buy amount".into()))?),
                    sep6_transactions::amount_in_asset.eq(Some(quote.sell_asset)),
                    sep6_transactions::amount_out_asset.eq(Some(quote.buy_asset)),
                ))
                .execute(&mut conn)
                .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;
        }

        Ok(WithdrawResponse {
            account_id: self.signing_key.public_key().to_string(),
            memo_type: Some("id".to_string()),
            memo: Some(transaction_id.clone()),  // Clone here
            id: transaction_id.clone(),  
            eta: None,
            min_amount: None,
            max_amount: None,
            fee_fixed: None,
            fee_percent: None,
        })
    }

    pub async fn withdraw_exchange(
        &self,
        request: WithdrawRequest,
        auth_token: &str,
    ) -> Result<WithdrawResponse, Sep6Error> {
        // Verify authentication
        let account = self.auth.verify_jwt(auth_token)
            .map_err(|e| Sep6Error::AuthFailed(e.to_string()))?;

        if account != request.account {
            return Err(Sep6Error::AuthFailed("Account mismatch".into()));
        }

        // Quote ID is required for exchange withdrawals
        let quote_id = request.quote_id
            .ok_or_else(|| Sep6Error::InvalidRequest("quote_id is required for exchange withdrawals".into()))?;

        // Get the quote from SEP-38 service
        let quote = self.sep38.get_quote(&quote_id, auth_token).await
            .map_err(|e| Sep6Error::QuoteError(e.to_string()))?;

        // Validate the quote is still valid
        if quote.expires_at < Utc::now() {
            return Err(Sep6Error::QuoteError("Quote has expired".into()));
        }


        // Check KYC status
        let kyc_status = self.sep12.get_customer(CustomerQuery {
            id: request.id.clone(),
            customer_type: request.customer_type.clone(),
            account: request.account.clone(),
            memo: request.memo.clone(),
            memo_type: request.memo_type.clone(),
            transaction_id: None,
            lang: None,
        }).await;

        if let Err(_) = kyc_status {
            return Err(Sep6Error::KYCRequired("Customer information needed".into()));
        }

        let mut conn = establish_connection()
            .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        // Create transaction record
        let transaction_id = Uuid::new_v4().to_string();
        let new_transaction = NewSep6Transaction {
            transaction_id: transaction_id.clone(),
            kind: "withdrawal-exchange".to_string(),
            status: "pending_user_transfer_start".to_string(),
            account: request.account.clone(),
            memo: request.memo.clone(),
            memo_type: request.memo_type.clone(),
            quote_id: Some(quote_id.clone()),
            amount_in: Some(BigDecimal::from_str(&quote.sell_amount)
                .map_err(|_| Sep6Error::InvalidRequest("Invalid sell amount".into()))?),
            amount_in_asset: Some(quote.sell_asset.clone()),
            amount_out: Some(BigDecimal::from_str(&quote.buy_amount)
                .map_err(|_| Sep6Error::InvalidRequest("Invalid buy amount".into()))?),
            amount_out_asset: Some(quote.buy_asset.clone()),
        };

        diesel::insert_into(sep6_transactions::table)
            .values(&new_transaction)
            .execute(&mut conn)
            .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        // Update the SEP-38 quote with our transaction ID
        diesel::update(sep38_quotes::table.filter(sep38_quotes::original_quote_id.eq(&quote_id)))
            .set(sep38_quotes::transaction_id.eq(Uuid::parse_str(&transaction_id).unwrap()))
            .execute(&mut conn)
            .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        Ok(WithdrawResponse {
            account_id: self.signing_key.public_key().to_string(),
            memo_type: Some("id".to_string()),
            memo: Some(transaction_id.clone()),
            id: transaction_id,
            eta: None,
            min_amount: None,
            max_amount: None,
            fee_fixed: Some(quote.fee.total.parse().unwrap_or(0.0)),
            fee_percent: None, 
        })
    }

    pub async fn get_transaction(
        &self,
        id: &str,
        auth_token: &str,
    ) -> Result<Sep6Transaction, Sep6Error> {
        let account = self.auth.verify_jwt(auth_token).map_err(|e| Sep6Error::AuthFailed(e.to_string()))?;

        let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        let transaction: Sep6Transaction = sep6_transactions::table
            .filter(sep6_transactions::transaction_id.eq(id))
            .first(&mut conn)
            .map_err(|_| Sep6Error::TransactionNotFound)?;

        if transaction.account != account {
            return Err(Sep6Error::AuthFailed("Transaction does not belong to account".into()));
        }

        Ok(transaction)
    }

    pub async fn get_transactions(
        &self,
        account: &str,
        auth_token: &str,
        asset_code: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Sep6Transaction>, Sep6Error> {
        let verified_account = self.auth.verify_jwt(auth_token).map_err(|e| Sep6Error::AuthFailed(e.to_string()))?;

        if verified_account != account {
            return Err(Sep6Error::AuthFailed("Account mismatch".into()));
        }

        let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        let mut query = sep6_transactions::table
            .filter(sep6_transactions::account.eq(account))
            .order_by(sep6_transactions::created_at.desc())
            .into_boxed();

        if let Some(code) = asset_code {
            query = query.filter(sep6_transactions::amount_in_asset.eq(code));
        }

        if let Some(lim) = limit {
            query = query.limit(lim);
        }

        let transactions = query.load::<Sep6Transaction>(&mut conn)
            .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        Ok(transactions)
    }
    pub async fn update_transaction_status(
        &self,
        transaction_id: &str,
        status: &str,
        stellar_tx_id: Option<&str>,
        external_tx_id: Option<&str>,
    ) -> Result<(), Sep6Error> {


        let mut conn = establish_connection().map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        let update = TransactionUpdate {
            status,
            updated_at: Utc::now().naive_utc(),
            stellar_transaction_id: stellar_tx_id,
            external_transaction_id: external_tx_id,
            completed_at: if status == "completed" {
                Some(Utc::now().naive_utc())
            } else {
                None
            },
        };

        diesel::update(sep6_transactions::table.filter(sep6_transactions::transaction_id.eq(transaction_id)))
            .set(update)
            .execute(&mut conn)
            .map_err(|e| Sep6Error::DatabaseError(e.to_string()))?;

        Ok(())
    }


}