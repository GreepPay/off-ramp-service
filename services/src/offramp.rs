
pub mod offramp {
    
use uuid::Uuid; 
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use helpers::info::AssetInfo;
use helpers::{
    auth::StellarAuth,
    info::InfoService,
    kyc::{KycFields, KycService},
    quote::{QuoteRequest, QuoteService},
    stellartoml::{TomlFetcher, StellarToml},
    withdraw_deposit::{TransactionStatus, TransferService, WithdrawRequest},
};
use models::{
    common::establish_connection,
    offramp::{Account, NewOfframpQuote, NewOfframpTransaction, OfframpTransaction },
    schema::offramp_service::{accounts, offramp_quotes, offramp_transactions},
};
use serde::Deserialize;
use stellar_base::Network;
use stellar_sdk::Keypair;
use std::str::FromStr;



#[derive(Debug, Deserialize)]
pub struct TransactionQueryParams {
    pub asset_code: Option<String>,
    pub kind: Option<String>,
    pub limit: Option<i64>,
    pub no_older_than: Option<NaiveDateTime>,
    pub paging_id: Option<i32>,
}

/* Core Offramp Functionality */
pub async fn offramp_funds(
    anchor_domain: &str,
    account_id: &str,
    keypair: &Keypair,
    status: &str,
    amount: f64,
    dest_currency: &str,
    kyc_fields: Option<serde_json::Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher = TomlFetcher::new();
    // 1. Fetch TOML config
    let toml: StellarToml= fetcher.fetch_toml(anchor_domain).await?;
    
   let transfer_server = &toml.transfer_server;
    
    // 2. Authenticate
    let jwt = StellarAuth::new(anchor_domain.to_string(), Network::new_public(), "JWT_SECRET".to_string())
        .authenticate(account_id, keypair)
        .await?;

    // 3. Handle KYC
    if let Some(fields) = kyc_fields {
        // Get the KYC server URL from the TOML config
        let kyc_server_url = toml.kyc_server.as_ref().ok_or("KYC server not configured")?;
    
        // Instantiate the KycService
        let kyc_service = KycService::new();
    
        // Use the instance to call get_kyc_status
        let kyc_status = kyc_service.get_kyc_status(
            kyc_server_url,
            account_id,
            &jwt,
        ).await?;
        
        if kyc_status.status != "ACCEPTED" {
            let kyc_server = toml.kyc_server.as_ref().ok_or("KYC server not configured")?;
            
            kyc_service.submit_kyc(
                kyc_server,
                KycFields {
                    account: account_id.to_string(),
                    memo: None,
                    memo_type: None,
                    fields,
                },
                &jwt,
            ).await?;
        }
    }
        // 4. Get quote if available
        let quote_service = QuoteService::new();
        let quote = if let Some(quote_server) = &toml.quote_server {
            Some(quote_service.get_quote(
                quote_server,
                QuoteRequest {
                    sell_asset: "stellar:USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN".to_string(),
                    buy_asset: format!("iso4217:{}", dest_currency),
                    sell_amount: Some(amount.to_string()),
                    buy_amount: None,
                    context: Some("sep6".to_string()),
                },
                Some(jwt.as_str()), 
            ).await?)
        } else {
            None
        };
    
        let transfer_service = TransferService::new();
        let withdraw_response = transfer_service.process_withdrawal(
            transfer_server,
            WithdrawRequest {
                asset_code: "USDC".to_string(),
                account: account_id.to_string(),
                amount: amount.to_string(),
                dest: None,
                dest_extra: None,
                memo: None,
                memo_type: None,
            },
            &jwt
        ).await?;
        
        let mut conn = establish_connection().map_err(|e| format!("DB connection failed: {}", e))?;
        
        // First get the ID by stellar_address
        let account_id: Uuid = accounts::table
            .filter(accounts::stellar_address.eq(account_id))
            .select(accounts::id)
            .first(&mut conn)
            .map_err(|e| format!("Failed to find account ID: {}", e))?;
        
        let account = accounts::table
            .find(account_id)
            .first::<Account>(&mut conn)
            .map_err(|e| format!("Failed to fetch account: {}", e))?;
        
        // Create new transaction (async)
        let new_tx = NewOfframpTransaction {
            account_id: account.id,
            transaction_id: &withdraw_response.id.clone(),
            amount: BigDecimal::from_str(&amount.to_string())
                .map_err(|e| format!("Invalid amount: {}", e))?,
            dest_currency: &dest_currency.to_string(),
            status: status,
        };
        
        diesel::insert_into(offramp_transactions::table)
            .values(&new_tx)
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert transaction: {}", e))?;
        
        // Handle quote if available (async)
        let tx = diesel::insert_into(offramp_transactions::table)
            .values(&new_tx)
            .get_result::<OfframpTransaction>(&mut conn)
            .map_err(|e| format!("Failed to insert transaction: {}", e))?;
        
        // Then create the quote if available
        if let Some(quote) = quote {
            diesel::insert_into(offramp_quotes::table)
                .values(NewOfframpQuote {
                    transaction_id: tx.id,  // Use the actual transaction ID
                    quote_id: &quote.id,
                    sell_asset: &quote.sell_asset,
                    buy_asset: &quote.buy_asset,
                    sell_amount: BigDecimal::from_str(&quote.sell_amount)?,
                    buy_amount: BigDecimal::from_str(&quote.buy_amount)?,
                    price: BigDecimal::from_str(&quote.price)?,
                    expires_at: Utc::now().naive_utc(),
                })
                .execute(&mut conn)
                .map_err(|e| format!("Failed to insert quote: {}", e))?;
        }
        
        Ok(withdraw_response.id)
    }

   


/* Transaction Status Functions */
pub async fn get_transaction(
    anchor_domain: &str,
    transaction_id: &str,
    account_id: &str,
    keypair: &Keypair,
) -> Result<TransactionStatus, Box<dyn std::error::Error>> {
    let mut conn = establish_connection().map_err(|e| format!("DB connection failed: {}", e))?;
    
    // Check database first
    let tx: OfframpTransaction = offramp_transactions::table
        .filter(offramp_transactions::transaction_id.eq(transaction_id))
        .first(&mut conn)?;

    let account_id: Uuid = accounts::table
        .filter(accounts::stellar_address.eq(account_id))
        .select(accounts::id)
        .first(&mut conn)
        .map_err(|e| format!("Failed to find account ID: {}", e))?;
    
    let account = accounts::table
        .find(account_id)
        .first::<Account>(&mut conn)
        .map_err(|e| format!("Failed to fetch account: {}", e))?;

    if tx.account_id != account.id {
        return Err("Transaction not found".into());
    }

    if tx.status == "completed" {
        return Ok(TransactionStatus {
            id: tx.transaction_id,
            status: tx.status,
            amount_in: Some(tx.amount.to_string()),
            amount_out: Some(tx.amount.to_string()),
            amount_fee: Some("0".to_string()),
            started_at: Some(tx.created_at.to_string()),
            completed_at: Some(tx.updated_at.to_string()),
            stellar_transaction_id: None,
            external_transaction_id: None,
        });
    }

    let fetcher = TomlFetcher::new();
    // 1. Fetch TOML config
    let toml: StellarToml= fetcher.fetch_toml(anchor_domain).await?;
    
    
   let transfer_server = &toml.transfer_server;
    
    // 2. Authenticate
    let jwt = StellarAuth::new(anchor_domain.to_string(), Network::new_public(), "JWT_SECRET".to_string())
        .authenticate(account_id.to_string().as_str(), keypair)
        .await?;
     let transfer_service = TransferService::new();
    let status = transfer_service.check_transaction_status(
        transfer_server,
        transaction_id,
        &jwt
    ).await?;

    // Update database
    diesel::update(offramp_transactions::table)
        .filter(offramp_transactions::id.eq(tx.id))
        .set((
            offramp_transactions::status.eq(&status.status),
            offramp_transactions::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    Ok(status)
}


/* Asset Info Function */
pub async fn get_asset_info(
    anchor_domain: &str,
    asset_code: &str,
    operation_type: Option<&str>,
) -> Result<AssetInfo, Box<dyn std::error::Error>> {  // Changed return type
    let fetcher = TomlFetcher::new();
    let toml: StellarToml = fetcher.fetch_toml(anchor_domain).await?;
    let transfer_server = &toml.transfer_server;
    let info_service = InfoService::new();
    
    info_service.get_asset_info(
        transfer_server,
        asset_code,
        operation_type.unwrap_or("withdraw")
    ).await
    .map_err(|e| e.into())  
}

}




