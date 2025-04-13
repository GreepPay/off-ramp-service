use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use bigdecimal::BigDecimal;
use stellar_base::PublicKey;
use uuid::Uuid;
use chrono::Utc;
use helpers::stellar_chain::StellarChain;
use stellar_base::asset::{Asset, CreditAsset};
use chrono::NaiveDateTime;
use serde::Deserialize;
use stellar_base::NetworkId;

use helpers::{
    stellartoml::TomlFetcher,
    auth::StellarAuth,
    kyc::KycService,
    withdraw_deposit::TransferService,
    quote::QuoteService,
    info::InfoService,
};
use models::offramp::{
    offramp::{OfframpTransaction, NewOfframpTransaction, OfframpQuote, NewOfframpQuote},
    account::Account,
};

use models::{
    schema::{offramp_transactions, offramp_quotes},
    error::Error,
    common::establish_connection,
};

// Shared state that would normally be in the struct
struct OfframpServices {
    // payment_service: Arc<PaymentService>,
    toml_fetcher: Arc<TomlFetcher>,
    auth_service: Arc<StellarAuth>,
    kyc_service: Arc<KycService>,
    transfer_service: Arc<TransferService>,
    quote_service: Arc<QuoteService>,
    anchor_domain: String,
    jwt_cache: Mutex<Option<String>>,
}
#[derive(Debug, Deserialize)]
pub struct TransactionQueryParams {
    #[serde(default)]
    pub asset_code: Option<String>,
    #[serde(default)]
    pub kind: Option<String>, // "deposit" or "withdrawal"
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default, with = "iso8601::option")]
    pub no_older_than: Option<NaiveDateTime>,
    #[serde(default)]
    pub paging_id: Option<i32>,
}
// Initialize shared services
async fn init_offramp_services(
    anchor_domain: String,
    network: NetworkId,
    jwt_secret: String,
) -> OfframpServices {
    OfframpServices {
        toml_fetcher: Arc::new(TomlFetcher::new()),
        auth_service: Arc::new(StellarAuth::new(anchor_domain.clone(), network, jwt_secret)),
        kyc_service: Arc::new(KycService::new()),
        transfer_service: Arc::new(TransferService::new()),
        quote_service: Arc::new(QuoteService::new()),
        anchor_domain,
        jwt_cache: Mutex::new(None),
    }
}

async fn get_jwt(
    services: &self,
    account_id: &str,
    keypair: &Keypair,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut cache = services.jwt_cache.lock().await;
    if let Some(jwt) = &*cache {
        Ok(jwt.clone())
    } else {
        let jwt = StellarAuth.get_jwt(account_id, keypair).await?;
        *cache = Some(jwt.clone());
        Ok(jwt)
    }
}

pub async fn offramp_funds(
    services: &self,
    account_id: &str,
    keypair: &KeyPair,
    amount: f64,
    dest_currency: &str,
    kyc_fields: Option<serde_json::Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut db_conn = establish_connection().await?;

    // Get account from database
    let account: Account = accounts::table
        .filter(accounts::stellar_address.eq(account_id))
        .first(&mut db_conn)
        .await?;

    let toml = services.toml_fetcher.fetch_toml(&services.anchor_domain).await?;
    let jwt = get_jwt(services, account_id, keypair).await?;

    // Handle KYC
    if let Some(fields) = &kyc_fields {
        let kyc_status = KycService.get_kyc_status(
            toml.kyc_server.as_ref().ok_or("KYC server not configured")?,
            account_id,
            &jwt
        ).await?;

        if kyc_status.status != "ACCEPTED" {
            let kyc_request = KycFields {
                account: account_id.to_string(),
                memo: None,
                memo_type: None,
                fields: fields.clone(),
            };

            services.kyc_service.submit_kyc(
                toml.kyc_server.as_ref().ok_or("KYC server not configured")?,
                kyc_request,
                &jwt
            ).await?;
        }
    }

    // Get quote if available
    let quote = if let Some(quote_server) = toml.quote_server {
        let quote_request = QuoteRequest {
            sell_asset: "stellar:USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN".to_string(),
            buy_asset: format!("iso4217:{}", dest_currency),
            sell_amount: Some(amount.to_string()),
            buy_amount: None,
            context: Some("sep6".to_string()),
        };

        Some(services.quote_service.get_quote(&quote_server, quote_request, Some(&jwt)).await?)
    } else {
        None
    };

    // Initiate withdrawal
    let withdraw_request = WithdrawRequest {
        asset_code: "USDC".to_string(),
        account: account_id.to_string(),
        amount: amount.to_string(),
        dest: None,
        dest_extra: None,
        memo: None,
        memo_type: None,
    };

    let withdraw_response = TransferService.initiate_withdraw(
        &toml.transfer_server,
        withdraw_request,
        &jwt
    ).await?;

    // Save transaction to database
    let new_transaction = NewOfframpTransaction {
        account_id: account.id,
        transaction_id: withdraw_response.id.clone(),
        amount: BigDecimal::from(amount),
        dest_currency: dest_currency.to_string(),
        status: "pending".to_string(),
        kyc_fields: kyc_fields.clone(),
    };

    diesel::insert_into(offramp_transactions::table)
        .values(&new_transaction)
        .execute(&mut db_conn)
        .await?;

    // Save quote if available
    if let Some(quote) = quote {
        let transaction: OfframpTransaction = offramp_transactions::table
            .filter(offramp_transactions::transaction_id.eq(&withdraw_response.id))
            .first(&mut db_conn)
            .await?;

        let new_quote = NewOfframpQuote {
            transaction_id: transaction.id,
            quote_id: quote.id,
            sell_asset: quote.sell_asset,
            buy_asset: quote.buy_asset,
            sell_amount: BigDecimal::from_str(&quote.sell_amount)?,
            buy_amount: BigDecimal::from_str(&quote.buy_amount)?,
            price: BigDecimal::from_str(&quote.price)?,
            expires_at: quote.expires_at,
        };

        diesel::insert_into(offramp_quotes::table)
            .values(&new_quote)
            .execute(&mut db_conn)
            .await?;
    }

    Ok(withdraw_response.id)
}


pub async fn get_transaction(
    services: &self,
    transaction_id: &str,
    account_id: &str,
    keypair: &KeyPair,
) -> Result<TransactionStatus, Box<dyn std::error::Error>> {
    let mut db_conn = establish_connection().await?;
    
    // First check our database
    let transaction: OfframpTransaction = offramp_transactions::table
        .filter(offramp_transactions::transaction_id.eq(transaction_id))
        .first(&mut db_conn)
        .await?;

    // Verify account ownership
    let account: Account = accounts::table
        .filter(accounts::stellar_address.eq(account_id))
        .first(&mut db_conn)
        .await?;
    
    if transaction.account_id != account.id {
        return Err(Box::new(Error::Unauthorized));
    }

    // If we have complete info, return it
    if transaction.status == "completed" {
        return Ok(TransactionStatus {
            id: transaction.transaction_id,
            kind: "withdrawal".to_string(),
            status: transaction.status,
            amount_in: transaction.amount.to_string(),
            amount_out: transaction.amount.to_string(), // Adjust if different
            amount_fee: "0".to_string(), // Calculate actual fee
            started_at: transaction.created_at.to_string(),
            completed_at: Some(transaction.updated_at.to_string()),
            stellar_transaction_id: transaction.stellar_tx_hash,
            withdraw_anchor_account: transaction.anchor_account,
            withdraw_memo: transaction.memo,
            withdraw_memo_type: transaction.memo_type,
        });
    }

    // Otherwise query anchor
    let toml = services.toml_fetcher.fetch_toml(&services.anchor_domain).await?;
    let jwt = get_jwt(services, account_id, keypair).await?;
    
    let status = services.transfer_service.get_transaction_status(
        &toml.transfer_server,
        transaction_id,
        &jwt
    ).await?;

    // Update our database
    diesel::update(offramp_transactions::table)
        .filter(offramp_transactions::id.eq(transaction.id))
        .set((
            offramp_transactions::status.eq(&status.status),
            offramp_transactions::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut db_conn)
        .await?;

    Ok(status)
}
pub async fn check_transaction_status(
    services: &self,
    transaction_id: &str,
    account_id: &str,
    keypair: &Keypair,
) -> Result<TransactionStatus, Box<dyn std::error::Error>> {
    let mut db_conn = establish_connection().await?;
    
    // First check our database
    let transaction: OfframpTransaction = offramp_transactions::table
        .filter(offramp_transactions::transaction_id.eq(transaction_id))
        .first(&mut db_conn)
        .await?;
    
    // If status is already completed in our DB, return it
    if transaction.status == "completed" || transaction.status == "failed" {
        return Ok(TransactionStatus {
            id: transaction.transaction_id,
            status: transaction.status,
            // ... other fields
        });
    }
    
    // Otherwise check with the anchor
    let toml = services.toml_fetcher.fetch_toml(&services.anchor_domain).await?;
    let jwt = get_jwt(services, account_id, keypair).await?;
    
    let status = services.transfer_service.get_transaction_status(
        &toml.transfer_server,
        transaction_id,
        &jwt
    ).await?;
    
    // Update our database with the new status
    diesel::update(offramp_transactions::table)
        .filter(offramp_transactions::transaction_id.eq(transaction_id))
        .set((
            offramp_transactions::status.eq(&status.status),
            offramp_transactions::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut db_conn)
        .await?;
    
    Ok(status)
}

pub async fn get_transactions(
        &self,
        account_id: &str,
        keypair: &KeyPair,
        params: TransactionQueryParams,
    ) -> Result<Vec<TransactionStatus>, Error> {
        let mut db_conn = establish_connection().await?;
        
        // First check our database
        let mut query = offramp_transactions::table
            .inner_join(accounts::table.on(offramp_transactions::account_id.eq(accounts::id)))
            .filter(accounts::stellar_address.eq(account_id))
            .into_boxed();
        
        // Apply filters from params using helper methods
        self.apply_transaction_filters(&mut query, &params);
        
        let transactions: Vec<(OfframpTransaction, Account)> = query
            .order_by(offramp_transactions::created_at.desc())
            .load(&mut db_conn)
            .await?;
        
        // Check if we can return cached results
        if self.all_transactions_completed(&transactions) {
            return Ok(self.format_completed_transactions(transactions));
        }
        
        // Otherwise query anchor for incomplete transactions
        self.update_incomplete_transactions(account_id, keypair, transactions).await
    }

    // Helper method to apply filters
    fn apply_transaction_filters(
        &self,
        query: &mut BoxedQuery<'_, diesel::pg::Pg>,
        params: &TransactionQueryParams,
    ) {
        if let Some(asset_code) = &params.asset_code {
            query.filter(offramp_transactions::asset_code.eq(asset_code));
        }
        if let Some(kind) = &params.kind {
            query.filter(offramp_transactions::kind.eq(kind));
        }
        if let Some(limit) = params.limit {
            query.limit(limit);
        }
        if let Some(no_older_than) = &params.no_older_than {
            query.filter(offramp_transactions::created_at.ge(no_older_than));
        }
        if let Some(paging_id) = params.paging_id {
            query.filter(offramp_transactions::id.lt(paging_id));
        }
    }

    // Helper to check if all transactions are complete
    fn all_transactions_completed(&self, transactions: &[(OfframpTransaction, Account)]) -> bool {
        transactions.iter()
            .all(|(t, _)| t.status == "completed" || t.status == "failed")
    }

    // Helper to format completed transactions
    fn format_completed_transactions(
        &self,
        transactions: Vec<(OfframpTransaction, Account)>,
    ) -> Vec<TransactionStatus> {
        transactions.into_iter()
            .map(|(t, _)| TransactionStatus {
                id: t.transaction_id,
                kind: "withdrawal".to_string(),
                status: t.status,
                amount_in: t.amount.to_string(),
                amount_out: t.amount.to_string(),
                amount_fee: "0".to_string(),
                started_at: t.created_at.to_string(),
                completed_at: Some(t.updated_at.to_string()),
                stellar_transaction_id: t.stellar_tx_hash,
                withdraw_anchor_account: t.anchor_account,
                withdraw_memo: t.memo,
                withdraw_memo_type: t.memo_type,
            })
            .collect()
    }

    // Helper to update incomplete transactions
    async fn update_incomplete_transactions(
        &self,
        account_id: &str,
        keypair: &KeyPair,
        transactions: Vec<(OfframpTransaction, Account)>,
    ) -> Result<Vec<TransactionStatus>, Error> {
        let mut db_conn = establish_connection().await?;
        let toml = self.toml_fetcher.fetch_toml(&self.anchor_domain).await?;
        let jwt = self.get_jwt(account_id, keypair).await?;
        
        let mut results = Vec::new();
        
        for (transaction, _) in transactions {
            if transaction.status == "completed" || transaction.status == "failed" {
                results.push(self.format_transaction_status(transaction));
            } else {
                let status = self.transfer_service.get_transaction_status(
                    &toml.transfer_server,
                    &transaction.transaction_id,
                    &jwt
                ).await?;
                
                diesel::update(offramp_transactions::table)
                    .filter(offramp_transactions::id.eq(transaction.id))
                    .set((
                        offramp_transactions::status.eq(&status.status),
                        offramp_transactions::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(&mut db_conn)
                    .await?;
                
                results.push(status);
            }
        }
        
        Ok(results)
    }

    // Better implementation for get_asset_info using InfoService
    pub async fn get_asset_info(
        &self,
        asset_code: &str,
        operation_type: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        let toml = self.toml_fetcher.fetch_toml(&self.anchor_domain).await?;
        let transfer_server = toml.transfer_server
            .as_ref()
            .ok_or(Error::ConfigurationError("No transfer server configured".into()))?;
        
        self.info_service.get_asset_info(
            transfer_server,
            asset_code,
            operation_type.unwrap_or("withdraw")
        ).await
    }