// SEP38
// Helper functions:
// Exchange Info
// Exchange Prices
// Exchange  Fees
// Exchange rate
// Quote exchange Price 

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use stellar_base::KeyPair;
use serde::{Deserialize, Serialize};
use thiserror::Error;
// Import the FromStr trait to use from_str
use std::str::FromStr;
use crate::sep10::StellarAuth;

use models::{
    common::establish_connection,
    sep38::NewSep38Quote,
    schema::offramp_service::sep38_quotes,
};

#[derive(Error, Debug)]
pub enum Sep38Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Authentication failed")]
    AuthFailed,
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Quote not found")]
    QuoteNotFound,
    
    #[error("Asset not supported")]
    AssetNotSupported,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FeeDetail {
    pub total: String,
    #[serde(skip)]
    pub asset: String,
    #[serde(rename = "asset")]
    pub asset_string: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<FeeComponent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuoteRequest {
    #[serde(rename = "id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FeeComponent {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub asset: String,
    pub sell_delivery_methods: Option<Vec<DeliveryMethod>>,
    pub buy_delivery_methods: Option<Vec<DeliveryMethod>>,
    pub country_codes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryMethod {
    pub name: String,
    pub description: String,
}

// GET /price request
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceRequest {
    #[serde(rename = "sell_asset")]
    pub sell_asset: String,
    #[serde(rename = "buy_asset")]
    pub buy_asset: String,
    #[serde(rename = "sell_amount", skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(rename = "buy_amount", skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(rename = "sell_delivery_method", skip_serializing_if = "Option::is_none")]
    pub sell_delivery_method: Option<String>,
    #[serde(rename = "buy_delivery_method", skip_serializing_if = "Option::is_none")]
    pub buy_delivery_method: Option<String>,
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    pub context: String,
}

// GET /price response
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    #[serde(rename = "total_price")]
    pub total_price: String,
    pub price: String,
    #[serde(rename = "sell_amount")]
    pub sell_amount: String,
    #[serde(rename = "buy_amount")]
    pub buy_amount: String,
    pub fee: FeeDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceAsset {
    pub asset: String,
    pub price: String,
    pub decimals: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub id: String,
    pub expires_at: String,
    pub total_price: String,
    pub price: String,
    pub sell_asset: String,
    pub sell_amount: String,
    pub sell_delivery_method: Option<String>,
    pub buy_asset: String,
    pub buy_amount: String,
    pub buy_delivery_method: Option<String>,
    pub fee: Fee,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fee {
    pub total: String,
    pub asset: String,
    pub details: Option<Vec<FeeDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteRequest {
    #[serde(rename = "sell_asset")]
    pub sell_asset: String,
    #[serde(rename = "buy_asset")]
    pub buy_asset: String,
    #[serde(rename = "sell_amount", skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(rename = "buy_amount", skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(rename = "expire_after", skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<String>,
    #[serde(rename = "sell_delivery_method", skip_serializing_if = "Option::is_none")]
    pub sell_delivery_method: Option<String>,
    #[serde(rename = "buy_delivery_method", skip_serializing_if = "Option::is_none")]
    pub buy_delivery_method: Option<String>,
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    pub context: String,
}

pub struct Sep38Service {
    client: reqwest::Client,
    auth_service: StellarAuth,
    quote_server: String,
    keypair: KeyPair,
}

impl Sep38Service {
    pub fn new(auth_service: StellarAuth, keypair: KeyPair, quote_server: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            auth_service,
            quote_server,
            keypair,
        
        }
    }

    // 1. GET /exchange_info
    pub async fn get_exchange_info(&self) -> Result<Vec<AssetInfo>, Sep38Error> {
        let response = self.client
            .get(&format!("{}/info", self.quote_server))
            .send()
            .await?;
        
        if response.status().is_success() {
            let info: serde_json::Value = response.json().await?;
            let assets = info["assets"].as_array().ok_or(Sep38Error::InvalidRequest("Invalid response format".to_string()))?;
            
            let mut result = Vec::new();
            for asset in assets {
                result.push(AssetInfo {
                    asset: asset["asset"].as_str().unwrap_or_default().to_string(),
                    sell_delivery_methods: asset["sell_delivery_methods"].as_array().map(|methods| {
                        methods.iter().map(|m| DeliveryMethod {
                            name: m["name"].as_str().unwrap_or_default().to_string(),
                            description: m["description"].as_str().unwrap_or_default().to_string(),
                        }).collect()
                    }),
                    buy_delivery_methods: asset["buy_delivery_methods"].as_array().map(|methods| {
                        methods.iter().map(|m| DeliveryMethod {
                            name: m["name"].as_str().unwrap_or_default().to_string(),
                            description: m["description"].as_str().unwrap_or_default().to_string(),
                        }).collect()
                    }),
                    country_codes: asset["country_codes"].as_array().map(|codes| {
                        codes.iter().filter_map(|c| c.as_str()).map(|s| s.to_string()).collect()
                    }),
                });
            }
            
            Ok(result)
        } else {
            Err(Sep38Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 2. GET /exchange_prices
    pub async fn get_exchange_prices(
        &self,
        sell_asset: String,
        buy_asset: String,
        sell_amount: String,
        buy_amount: String,
        sell_delivery_method: String,
        buy_delivery_method: String,
        country_code: String,
        context: String,
    ) -> Result<PriceResponse, Sep38Error> {
        let mut request = self.client
            .get(&format!("{}/prices", self.quote_server));

        if !sell_asset.is_empty() {
            request = request.query(&[("sell_asset", &sell_asset)]);
        }

        if !buy_asset.is_empty() {
            request = request.query(&[("buy_asset", &buy_asset)]);
        }

        if !sell_amount.is_empty() {
            request = request.query(&[("sell_amount", &sell_amount)]);
        }

        if !buy_amount.is_empty() {
            request = request.query(&[("buy_amount", &buy_amount)]);
        }

        if !sell_delivery_method.is_empty() {
            request = request.query(&[("sell_delivery_method", &sell_delivery_method)]);
        }

        if !buy_delivery_method.is_empty() {
            request = request.query(&[("buy_delivery_method", &buy_delivery_method)]);
        }

        if !country_code.is_empty() {
            request = request.query(&[("country_code", &country_code)]);
        }

        request = request.query(&[("context", &context)]);
        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Sep38Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 3. GET /exchange_fees
    pub async fn get_exchange_fees(
        &self,
        sell_asset: &str,
        buy_asset: &str,
        sell_amount: Option<&str>,
        buy_amount: Option<&str>,
        sell_delivery_method: Option<&str>,
        buy_delivery_method: Option<&str>,
        country_code: Option<&str>,
        context: &str,
    ) -> Result<QuoteResponse, Sep38Error> {
        let mut request = self.client
            .get(&format!("{}/price", self.quote_server))
            .query(&[("sell_asset", sell_asset)])
            .query(&[("buy_asset", buy_asset)])
            .query(&[("context", context)]);
        
        if let Some(amount) = sell_amount {
            request = request.query(&[("sell_amount", amount)]);
        }
        
        if let Some(amount) = buy_amount {
            request = request.query(&[("buy_amount", amount)]);
        }
        
        if let Some(method) = sell_delivery_method {
            request = request.query(&[("sell_delivery_method", method)]);
        }
        
        if let Some(method) = buy_delivery_method {
            request = request.query(&[("buy_delivery_method", method)]);
        }
        
        if let Some(code) = country_code {
            request = request.query(&[("country_code", code)]);
        }
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Sep38Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }

    // 4. GET /exchange_rate
    pub async fn get_exchange_rate(
        &self,
        sell_asset: &str,
        buy_asset: &str,
        sell_amount: Option<&str>,
        buy_amount: Option<&str>,
        sell_delivery_method: Option<&str>,
        buy_delivery_method: Option<&str>,
        country_code: Option<&str>,
        context: &str,
    ) -> Result<String, Sep38Error> {
        let quote = self.get_exchange_fees(
            sell_asset,
            buy_asset,
            sell_amount,
            buy_amount,
            sell_delivery_method,
            buy_delivery_method,
            country_code,
            context,
        ).await?;
        
        Ok(quote.price)
    }

    // 5. POST /quote_exchange_price
    pub async fn quote_exchange_price(
        &self,
        account: &str,
        sell_asset: &str,
        buy_asset: &str,
        sell_amount: Option<&str>,
        buy_amount: Option<&str>,
        expire_after: Option<&str>,
        sell_delivery_method: Option<&str>,
        buy_delivery_method: Option<&str>,
        country_code: Option<&str>,
        context: &str,
    ) -> Result<QuoteResponse, Sep38Error> {
        let jwt = self.auth_service.authenticate(account, &self.keypair )
            .await
            .map_err(|_| Sep38Error::AuthFailed)?;
            
        let request = self.client
            .post(&format!("{}/quote", self.quote_server))
            .bearer_auth(jwt)
            .json(&serde_json::json!({
                "sell_asset": sell_asset,
                "buy_asset": buy_asset,
                "sell_amount": sell_amount,
                "buy_amount": buy_amount,
                "expire_after": expire_after,
                "sell_delivery_method": sell_delivery_method,
                "buy_delivery_method": buy_delivery_method,
                "country_code": country_code,
                "context": context,
            }));
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            let quote: QuoteResponse = response.json().await?;
            
            // Save to database
            let mut conn = establish_connection().map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
            
        

            let new_quote = NewSep38Quote {
                original_quote_id: quote.id.clone(),
                sell_asset: quote.sell_asset.clone(),
                buy_asset: quote.buy_asset.clone(),
                sell_amount: BigDecimal::from_str(&quote.sell_amount).map_err(|_| Sep38Error::InvalidRequest("Invalid sell amount".to_string()))?,
                buy_amount: BigDecimal::from_str(&quote.buy_amount).map_err(|_| Sep38Error::InvalidRequest("Invalid buy amount".to_string()))?,
                price: BigDecimal::from_str(&quote.price).map_err(|_| Sep38Error::InvalidRequest("Invalid price".to_string()))?,
                total_price: BigDecimal::from_str(&quote.total_price).map_err(|_| Sep38Error::InvalidRequest("Invalid total price".to_string()))?,
                fee_total: BigDecimal::from_str(&quote.fee.total).map_err(|_| Sep38Error::InvalidRequest("Invalid fee total".to_string()))?,
                fee_asset: quote.fee.asset.clone(),
                fee_details: quote.fee.details.as_ref().map(|details| {
                    serde_json::to_value(details).unwrap()
                }),
                sell_delivery_method: quote.sell_delivery_method.clone(),
                buy_delivery_method: quote.buy_delivery_method.clone(),
                expires_at: chrono::DateTime::parse_from_rfc3339(&quote.expires_at)
                    .map_err(|_| Sep38Error::InvalidRequest("Invalid expires_at".to_string()))?
                    .naive_utc(),
                context: context.to_string(),
                transaction_id: None,
            };
            
            diesel::insert_into(sep38_quotes::table)
                .values(&new_quote)
                .execute(&mut conn)
                .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;

            Ok(quote)
        } else {
            let error = response.text().await?;
            Err(Sep38Error::InvalidRequest(error))
        }
    }

    // 6. GET /quote
    pub async fn get_quote(&self, account: &str, quote_id: &str) -> Result<QuoteResponse, Sep38Error> {
        let jwt = self.auth_service.authenticate(account, &self.keypair)
            .await
            .map_err(|_| Sep38Error::AuthFailed)?;
            
        let response = self.client
            .get(&format!("{}/quote/{}", self.quote_server, quote_id))
            .bearer_auth(jwt)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == 404 {
            Err(Sep38Error::QuoteNotFound)
        } else {
            Err(Sep38Error::InvalidRequest(format!("Status: {}", response.status())))
        }
    }
}