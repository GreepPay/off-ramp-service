use serde::{Deserialize, Serialize};
use reqwest::Client;
use thiserror::Error;
use chrono::{DateTime, Utc,TimeZone};
use uuid::Uuid;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::upsert::excluded;


use models::{
    info::{Sep38Quote, NewSep38Quote, PriceResponse},
    schema::offramp_service::{ sep38_quotes, sep38_assets},
    common::establish_connection,

};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DeliveryMethod {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub asset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_delivery_methods: Option<Vec<DeliveryMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_delivery_methods: Option<Vec<DeliveryMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceAsset {
    pub asset: String,
    pub price: String,
    pub decimals: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeDetail {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fee {
    pub total: String,
    pub asset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<FeeDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub sell_asset: String,
    pub buy_asset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_delivery_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_delivery_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub id: String,
    pub expires_at: DateTime<Utc>,
    pub total_price: String,
    pub price: String,
    pub sell_asset: String,
    pub sell_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_delivery_method: Option<String>,
    pub buy_asset: String,
    pub buy_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_delivery_method: Option<String>,
    pub fee: Fee,
}

#[derive(Error, Debug)]
pub enum Sep38Error {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Server returned error: {0}")]
    ServerError(String),
    #[error("Asset not supported: {0}")]
    AssetNotSupported(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Quote not found")]
    QuoteNotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct Sep38Client {
    client: Client,
    base_url: String,
}


impl Sep38Client {
    pub fn new(base_url: String) -> Self {
        Sep38Client {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_info(&self) -> Result<Vec<AssetInfo>, Sep38Error> {
        let url = format!("{}/info", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(Sep38Error::ServerError(format!("Status: {}", response.status())));
        }

        let info: Vec<AssetInfo> = response.json().await?;
        
        let mut conn = establish_connection()
            .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
            
        for asset in &info {
            let delivery_methods = serde_json::to_value(asset.sell_delivery_methods.clone())
                .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
            let buy_methods = serde_json::to_value(asset.buy_delivery_methods.clone())
                .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
            let countries = serde_json::to_value(asset.country_codes.clone())
                .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;

            diesel::insert_into(sep38_assets::table)
                .values((
                    sep38_assets::asset.eq(&asset.asset),
                    sep38_assets::sell_delivery_methods.eq(delivery_methods),
                    sep38_assets::buy_delivery_methods.eq(buy_methods),
                    sep38_assets::country_codes.eq(countries),
                ))
                .on_conflict(sep38_assets::asset)
                .do_update()
                .set((
                    sep38_assets::sell_delivery_methods.eq(excluded(sep38_assets::sell_delivery_methods)),
                    sep38_assets::buy_delivery_methods.eq(excluded(sep38_assets::buy_delivery_methods)),
                    sep38_assets::country_codes.eq(excluded(sep38_assets::country_codes)),
                    sep38_assets::updated_at.eq(diesel::dsl::now),
                ))
                .execute(&mut conn)
                .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
        }

        Ok(info)
    }

    pub async fn get_prices(
        &self,
        sell_asset: Option<&str>,
        buy_asset: Option<&str>,
        sell_amount: Option<&str>,
        buy_amount: Option<&str>,
        sell_delivery_method: Option<&str>,
        buy_delivery_method: Option<&str>,
        country_code: Option<&str>,
    ) -> Result<Vec<PriceAsset>, Sep38Error> {
        let mut url = format!("{}/prices", self.base_url);
        let mut query = vec![];

        if let Some(asset) = sell_asset {
            query.push(format!("sell_asset={}", asset));
        }
        if let Some(asset) = buy_asset {
            query.push(format!("buy_asset={}", asset));
        }
        if let Some(amount) = sell_amount {
            query.push(format!("sell_amount={}", amount));
        }
        if let Some(amount) = buy_amount {
            query.push(format!("buy_amount={}", amount));
        }
        if let Some(method) = sell_delivery_method {
            query.push(format!("sell_delivery_method={}", method));
        }
        if let Some(method) = buy_delivery_method {
            query.push(format!("buy_delivery_method={}", method));
        }
        if let Some(code) = country_code {
            query.push(format!("country_code={}", code));
        }

        if !query.is_empty() {
            url.push_str(&format!("?{}", query.join("&")));
        }

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(Sep38Error::ServerError(format!("Status: {}", response.status())));
        }

        response.json().await.map_err(|e| e.into())
    }

    pub async fn get_price(
        &self,
        sell_asset: &str,
        buy_asset: &str,
        sell_amount: Option<&str>,
        buy_amount: Option<&str>,
        sell_delivery_method: Option<&str>,
        buy_delivery_method: Option<&str>,
        country_code: Option<&str>,
        context: &str,
    ) -> Result<PriceResponse, Sep38Error> {
        let mut url = format!("{}/price", self.base_url);
        let mut query = vec![
            format!("sell_asset={}", sell_asset),
            format!("buy_asset={}", buy_asset),
            format!("context={}", context),
        ];

        if let Some(amount) = sell_amount {
            query.push(format!("sell_amount={}", amount));
        }
        if let Some(amount) = buy_amount {
            query.push(format!("buy_amount={}", amount));
        }
        if let Some(method) = sell_delivery_method {
            query.push(format!("sell_delivery_method={}", method));
        }
        if let Some(method) = buy_delivery_method {
            query.push(format!("buy_delivery_method={}", method));
        }
        if let Some(code) = country_code {
            query.push(format!("country_code={}", code));
        }

        url.push_str(&format!("?{}", query.join("&")));

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(Sep38Error::ServerError(format!("Status: {}", response.status())));
        }

        response.json().await.map_err(|e| e.into())
    }

    pub async fn create_quote(
        &self,
        request: QuoteRequest,
        auth_token: &str,
        transaction_id: Option<Uuid>,
    ) -> Result<QuoteResponse, Sep38Error> {
        let url = format!("{}/quote", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", auth_token))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(Sep38Error::ServerError(format!("Status: {}", response.status())));
        }

        let quote: QuoteResponse = response.json().await?;
        
        let mut conn = establish_connection()
            .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
            
        let fee_details_json = match &quote.fee.details {
            Some(details) => Some(serde_json::to_value(details)
                .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?),
            None => None,
        };

        let new_quote = NewSep38Quote {
            original_quote_id: &quote.id,
            sell_asset: &quote.sell_asset,
            buy_asset: &quote.buy_asset,
            sell_amount: BigDecimal::from_str(&quote.sell_amount)
                .map_err(|_| Sep38Error::InvalidRequest("Invalid sell amount".to_string()))?,
            buy_amount: BigDecimal::from_str(&quote.buy_amount)
                .map_err(|_| Sep38Error::InvalidRequest("Invalid buy amount".to_string()))?,
            price: BigDecimal::from_str(&quote.price)
                .map_err(|_| Sep38Error::InvalidRequest("Invalid price".to_string()))?,
            total_price: BigDecimal::from_str(&quote.total_price)
                .map_err(|_| Sep38Error::InvalidRequest("Invalid total price".to_string()))?,
            fee_total: BigDecimal::from_str(&quote.fee.total)
                .map_err(|_| Sep38Error::InvalidRequest("Invalid fee total".to_string()))?,
            fee_asset: &quote.fee.asset,
            fee_details: fee_details_json.as_ref(),
            sell_delivery_method: quote.sell_delivery_method.as_deref(),
            buy_delivery_method: quote
                .buy_delivery_method.as_deref(),
            expires_at: quote.expires_at.naive_utc(),
            context: &request.context,
            transaction_id,
        };


        diesel::insert_into(sep38_quotes::table)
            .values(&new_quote)
            .execute(&mut conn)
            .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;

        Ok(quote)
    }

    pub async fn get_quote(
        &self,
        quote_id: &str,
        auth_token: &str,
    ) -> Result<QuoteResponse, Sep38Error> {
        let mut conn = establish_connection()
            .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;
            
        let db_quote: Option<Sep38Quote> = sep38_quotes::table
            .filter(sep38_quotes::original_quote_id.eq(quote_id))
            .first(&mut conn)
            .optional()
            .map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;


        if let Some(quote) = db_quote {
            return Ok(QuoteResponse {
                id: quote.original_quote_id,
                expires_at: Utc.from_utc_datetime(&quote.expires_at),
                total_price: quote.total_price.to_string(),
                price: quote.price.to_string(),
                sell_asset: quote.sell_asset,
                sell_amount: quote.sell_amount.to_string(),
                sell_delivery_method: quote.sell_delivery_method,
                buy_asset: quote.buy_asset,
                buy_amount: quote.buy_amount.to_string(),
                buy_delivery_method: quote.buy_delivery_method,
                fee: Fee {
                    total: quote.fee_total.to_string(),
                    asset: quote.fee_asset,
                    details: quote.fee_details.and_then(|v| serde_json::from_value(v).ok()),
                },
            });
        }

        let url = format!("{}/quote/{}", self.base_url, quote_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth_token))
            .send()
            .await?;
        
        if response.status() == 404 {
            return Err(Sep38Error::QuoteNotFound);
        }
        
        if !response.status().is_success() {
            return Err(Sep38Error::ServerError(format!("Status: {}", response.status())));
        }

        response.json().await.map_err(|e| e.into())
    }
}