pub mod offramp {
    use uuid::Uuid;
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use diesel::prelude::*;
    use diesel::PgConnection;
    use helpers::info::AssetInfo;
    use helpers::{
        auth::StellarAuth,
        info::InfoService,
        kyc::{KycFields, KycService},
        quote::{QuoteRequest, QuoteService},
        stellartoml::{TomlFetcher, StellarToml},
        withdraw_deposit::{TransactionStatus, TransferService, WithdrawRequest},
    };
    use std::fmt;
    use std::error::Error;
             
    use models::{
        offramp::{NewOfframpQuote, NewOfframpTransaction, OfframpTransaction},
        schema::offramp_service::{accounts, offramp_quotes, offramp_transactions},
        common::establish_connection,

    };

    use stellar_base::Network;
    use stellar_sdk::Keypair;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct OfframpService {
        network: Network,
        jwt_secret: String,
        default_sell_asset: String,
        toml_fetcher: TomlFetcher, // Box implements Debug regardless of contents
        transfer_service: TransferService,
    }


    
    #[derive(Debug)]
    pub enum OfframpError {
        Config(String),
        Currency,
        Amount,
        Database(String),
        Stellar(String),
        Auth(String),
        Kyc(String),
        Quote(String),
        Transfer(String),
        NotFound,
    }
    
    impl fmt::Display for OfframpError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                OfframpError::Config(msg) => write!(f, "Configuration error: {}", msg),
                OfframpError::Currency => write!(f, "Invalid currency format (must be 3 uppercase letters)"),
                OfframpError::Amount => write!(f, "Invalid amount (must be positive)"),
                OfframpError::Database(msg) => write!(f, "Database error: {}", msg),
                OfframpError::Stellar(msg) => write!(f, "Stellar error: {}", msg),
                OfframpError::Auth(msg) => write!(f, "Authentication error: {}", msg),
                OfframpError::Kyc(msg) => write!(f, "KYC error: {}", msg),
                OfframpError::Quote(msg) => write!(f, "Quote error: {}", msg),
                OfframpError::Transfer(msg) => write!(f, "Transfer error: {}", msg),
                OfframpError::NotFound => write!(f, "Transaction not found"),
            }
        }
    }
    
    impl Error for OfframpError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None // No underlying error for these cases
        }
    }
    impl OfframpService {
        pub fn new(
            network: Network,
            jwt_secret: String,
            default_sell_asset: String,
        ) -> Self {
            Self {
                network,
                jwt_secret,
                default_sell_asset,
                toml_fetcher: TomlFetcher::new(),
                transfer_service: TransferService::new(),
            }
        }

        fn validate_account(&self, account_id: &str) -> Result<(), OfframpError> {
            stellar_base::PublicKey::from_account_id(account_id)
                .map_err(|_| OfframpError::Config("Invalid Stellar account ID".into()))?;
            Ok(())
        }

        fn validate_currency(&self, currency: &str) -> Result<(), OfframpError> {
            if currency.len() != 3 || !currency.chars().all(|c| c.is_ascii_uppercase()) {
                return Err(OfframpError::Currency);
            }
            Ok(())
        }

        fn validate_amount(&self, amount: f64) -> Result<(), OfframpError> {
            if amount <= 0.0 {
                return Err(OfframpError::Amount);
            }
            Ok(())
        }

        fn establish_connection(&self) -> Result<PgConnection, OfframpError> {
            establish_connection()
                .map_err(|e| OfframpError::Database(format!("Connection failed: {}", e)))
        }

        pub async fn offramp_funds(
            &self,
            anchor_domain: &str,
            account_id: &str,
            keypair: &Keypair,
            amount: f64,
            dest_currency: &str,
            kyc_fields: Option<serde_json::Value>,
        ) -> Result<String, OfframpError> {
            // Validate inputs
            self.validate_account(account_id)?;
            self.validate_currency(dest_currency)?;
            self.validate_amount(amount)?;

            // 1. Fetch TOML config
            let toml = self.toml_fetcher.fetch_toml(anchor_domain).await
                .map_err(|e| OfframpError::Config(format!("Failed to fetch TOML: {}", e)))?;
            
            let transfer_server = &toml.transfer_server;
            
            // 2. Authenticate
            let auth = StellarAuth::new(
                anchor_domain.to_string(),
                self.network.clone(),
                self.jwt_secret.clone(),
            );
            let jwt = auth.authenticate(account_id, keypair).await
                .map_err(|e| OfframpError::Auth(format!("Authentication failed: {}", e)))?;

            // 3. Handle KYC
            if let Some(fields) = kyc_fields {
                let kyc_server = toml.kyc_server.as_ref()
                    .ok_or(OfframpError::Kyc("KYC server not configured".into()))?;
                
                let kyc_service = KycService::new();
                let kyc_status = kyc_service.get_kyc_status(kyc_server, account_id, &jwt).await
                    .map_err(|e| OfframpError::Kyc(format!("KYC check failed: {}", e)))?;
                
                if kyc_status.status != "ACCEPTED" {
                    kyc_service.submit_kyc(
                        kyc_server,
                        KycFields {
                            account: account_id.to_string(),
                            memo: None,
                            memo_type: None,
                            fields,
                        },
                        &jwt,
                    ).await
                    .map_err(|e| OfframpError::Kyc(format!("KYC submission failed: {}", e)))?;
                }
            }

            // 4. Get quote if available
            let quote_service = QuoteService::new();
            let quote = if let Some(quote_server) = &toml.quote_server {
                Some(quote_service.get_quote(
                    quote_server,
                    QuoteRequest {
                        sell_asset: self.default_sell_asset.clone(),
                        buy_asset: format!("iso4217:{}", dest_currency),
                        sell_amount: Some(amount.to_string()),
                        buy_amount: None,
                        context: Some("sep6".to_string()),
                    },
                    Some(&jwt),
                ).await
                .map_err(|e| OfframpError::Quote(format!("Quote failed: {}", e)))?)
            } else {
                None
            };

            // 5. Process withdrawal
            let withdraw_response = self.transfer_service.process_withdrawal(
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
                &jwt,
            ).await
            .map_err(|e| OfframpError::Transfer(format!("Withdrawal failed: {}", e)))?;

            // 6. Save transaction to database
            let mut conn = self.establish_connection()?;
            
            let account_id: Uuid = accounts::table
                .filter(accounts::stellar_address.eq(account_id))
                .select(accounts::id)
                .first(&mut conn)
                .map_err(|e| OfframpError::Database(format!("Failed to find account ID: {}", e)))?;
            
            let new_tx = NewOfframpTransaction {
                account_id,
                transaction_id: &withdraw_response.id,
                amount: BigDecimal::from_str(&amount.to_string())
                    .map_err(|_| OfframpError::Amount)?,
                dest_currency,
                status: "pending_user_transfer_start", // Initial status
            };
            
            let tx = diesel::insert_into(offramp_transactions::table)
                .values(&new_tx)
                .get_result::<OfframpTransaction>(&mut conn)
                .map_err(|e| OfframpError::Database(format!("Failed to insert transaction: {}", e)))?;

            // 7. Save quote if available
            if let Some(quote) = quote {
                diesel::insert_into(offramp_quotes::table)
                    .values(NewOfframpQuote {
                        transaction_id: tx.id,
                        quote_id: &quote.id,
                        sell_asset: &quote.sell_asset,
                        buy_asset: &quote.buy_asset,
                        sell_amount: BigDecimal::from_str(&quote.sell_amount)
                            .map_err(|_| OfframpError::Config("Invalid sell amount".into()))?,
                        buy_amount: BigDecimal::from_str(&quote.buy_amount)
                            .map_err(|_| OfframpError::Config("Invalid buy amount".into()))?,
                        price: BigDecimal::from_str(&quote.price)
                            .map_err(|_| OfframpError::Config("Invalid price".into()))?,
                        expires_at: Utc::now().naive_utc(),
                    })
                    .execute(&mut conn)
                    .map_err(|e| OfframpError::Database(format!("Failed to insert quote: {}", e)))?;
            }
            
            Ok(withdraw_response.id)
        }

        pub async fn get_transaction_status(
            &self,
            anchor_domain: &str,
            transaction_id: &str,
            account_id: &str,
            keypair: &Keypair,
        ) -> Result<TransactionStatus, OfframpError> {
            // 1. Fetch TOML config
            let toml = self.toml_fetcher.fetch_toml(anchor_domain).await
                .map_err(|e| OfframpError::Config(format!("Failed to fetch TOML: {}", e)))?;

            // 2. Authenticate
            let auth = StellarAuth::new(
                anchor_domain.to_string(),
                self.network.clone(),
                self.jwt_secret.clone(),
            );
            let jwt = auth.authenticate(account_id, keypair).await
                .map_err(|e| OfframpError::Auth(format!("Authentication failed: {}", e)))?;

            // 3. Check transaction status with anchor
            let status = self.transfer_service.check_transaction_status(
                &toml.transfer_server,
                transaction_id,
                &jwt,
            ).await
            .map_err(|e| OfframpError::Transfer(format!("Status check failed: {}", e)))?;

            // 4. Update database if status changed
            if status.status != "pending_user_transfer_start" {
                let mut conn = self.establish_connection()?;
                
                diesel::update(offramp_transactions::table)
                    .filter(offramp_transactions::transaction_id.eq(transaction_id))
                    .set((
                        offramp_transactions::status.eq(&status.status),
                        offramp_transactions::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(&mut conn)
                    .map_err(|e| OfframpError::Database(format!("Failed to update status: {}", e)))?;
            }

            Ok(status)
        }
        
        // /* Asset Info Function */
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
}
