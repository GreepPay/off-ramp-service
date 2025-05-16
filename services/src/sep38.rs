// SEP38
// Helper functions:
// Exchange Info
// Exchange Prices
// Quote exchange Price
pub mod sep38 {
    use bigdecimal::BigDecimal;
    use diesel::prelude::*;
    use reqwest::Client; // This is the correct import for Client
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    // Import the FromStr trait to use from_str
    use crate::common::get_anchor_config_details;
    use helpers::{auth::authenticate, keypair::generate_keypair};
    use std::str::FromStr;

    use models::{
        common::establish_connection, schema::offramp_service::sep38_quotes, sep38::NewSep38Quote,
    };

    #[derive(Error, Debug)]
    pub enum Sep38Error {
        #[error("HTTP error: {0}")]
        HttpError(#[from] reqwest::Error),

        #[error("Authentication failed")]
        AuthFailed,

        #[error("Keypair failed")]
        Keypairgenerationfailed,

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
        #[serde(
            rename = "sell_delivery_method",
            skip_serializing_if = "Option::is_none"
        )]
        pub sell_delivery_method: Option<String>,
        #[serde(
            rename = "buy_delivery_method",
            skip_serializing_if = "Option::is_none"
        )]
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
        #[serde(
            rename = "sell_delivery_method",
            skip_serializing_if = "Option::is_none"
        )]
        pub sell_delivery_method: Option<String>,
        #[serde(
            rename = "buy_delivery_method",
            skip_serializing_if = "Option::is_none"
        )]
        pub buy_delivery_method: Option<String>,
        #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
        pub country_code: Option<String>,
        pub context: String,
    }

    // 1. GET /exchange_info
    pub async fn get_exchange_info(slug: String) -> Result<Vec<AssetInfo>, Sep38Error> {
        let client = Client::new();

        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
                .await
                .map_err(|_| Sep38Error::AuthFailed)?;

        println!("anchor_config - {:?}", anchor_config);

        let quote_server = &anchor_config.general_info.anchor_quote_server;
        // Unwrap the Option or provide a default value
        let quote_server_str = quote_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );

        let response = client
            .get(&format!("{}/info", quote_server_str))
            .send()
            .await?;

        if response.status().is_success() {
            let info: serde_json::Value = response.json().await?;
            let assets = info["assets"].as_array().ok_or(Sep38Error::InvalidRequest(
                "Invalid response format".to_string(),
            ))?;

            let mut result = Vec::new();
            for asset in assets {
                result.push(AssetInfo {
                    asset: asset["asset"].as_str().unwrap_or_default().to_string(),
                    sell_delivery_methods: asset["sell_delivery_methods"].as_array().map(
                        |methods| {
                            methods
                                .iter()
                                .map(|m| DeliveryMethod {
                                    name: m["name"].as_str().unwrap_or_default().to_string(),
                                    description: m["description"]
                                        .as_str()
                                        .unwrap_or_default()
                                        .to_string(),
                                })
                                .collect()
                        },
                    ),
                    buy_delivery_methods: asset["buy_delivery_methods"].as_array().map(|methods| {
                        methods
                            .iter()
                            .map(|m| DeliveryMethod {
                                name: m["name"].as_str().unwrap_or_default().to_string(),
                                description: m["description"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                            })
                            .collect()
                    }),
                    country_codes: asset["country_codes"].as_array().map(|codes| {
                        codes
                            .iter()
                            .filter_map(|c| c.as_str())
                            .map(|s| s.to_string())
                            .collect()
                    }),
                });
            }

            Ok(result)
        } else {
            Err(Sep38Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // 2. GET /exchange_prices
    pub async fn get_exchange_prices(
        slug: String,
        sell_asset: String,
        buy_asset: String,
        sell_amount: String,
        buy_amount: String,
        sell_delivery_method: Option<String>,
        buy_delivery_method: Option<String>,
        country_code: Option<String>,
        context: String,
    ) -> Result<PriceResponse, Sep38Error> {
        let client = Client::new();

        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
                .await
                .map_err(|_| Sep38Error::AuthFailed)?;
        let quote_server = &anchor_config.general_info.anchor_quote_server;
        // Unwrap the Option or provide a default value
        let quote_server_str = quote_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );

        let mut request = client.get(&format!("{}/prices", quote_server_str));

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

        if let Some(method) = &sell_delivery_method {
            if !method.is_empty() {
                request = request.query(&[("sell_delivery_method", method)]);
            }
        }

        if let Some(method) = &buy_delivery_method {
            if !method.is_empty() {
                request = request.query(&[("buy_delivery_method", method)]);
            }
        }

        if let Some(code) = &country_code {
            if !code.is_empty() {
                request = request.query(&[("country_code", code)]);
            }
        }

        request = request.query(&[("context", &context)]);
        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Sep38Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }

    // 4. POST /quote_exchange_price
    pub async fn quote_exchange_price(
        slug: String,
        account: String,
        sell_asset: String,
        buy_asset: String,
        sell_amount: Option<String>,
        buy_amount: Option<String>,
        expire_after: Option<String>,
        sell_delivery_method: Option<String>,
        buy_delivery_method: Option<String>,
        country_code: Option<String>,
        context: String,
    ) -> Result<QuoteResponse, Sep38Error> {
        let client = Client::new();
        let keypair = match generate_keypair(account.as_str()) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep38Error::Keypairgenerationfailed),
        };
        // Get anchor configuration for authentication
        // Get anchor configuration for authentication
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
                .await
                .map_err(|_| Sep38Error::AuthFailed)?;

        let jwt = match authenticate(&helpers::stellartoml::AnchorService::new(), &slug, &keypair)
            .await
        {
            Ok(token) => token,
            Err(_) => return Err(Sep38Error::AuthFailed),
        };
        let quote_server = &anchor_config.general_info.anchor_quote_server;
        // Unwrap the Option or provide a default value
        let quote_server_str = quote_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );

        let request = client
            .post(&format!("{}/quote", quote_server_str))
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
            let mut conn =
                establish_connection().map_err(|e| Sep38Error::DatabaseError(e.to_string()))?;

            let new_quote = NewSep38Quote {
                original_quote_id: quote.id.clone(),
                sell_asset: quote.sell_asset.clone(),
                buy_asset: quote.buy_asset.clone(),
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
                fee_asset: quote.fee.asset.clone(),
                fee_details: quote
                    .fee
                    .details
                    .as_ref()
                    .map(|details| serde_json::to_value(details).unwrap()),
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

    // 5. GET /quote
    pub async fn get_quote(
        account: String,
        quote_id: String,
        slug: String,
    ) -> Result<QuoteResponse, Sep38Error> {
        let client = Client::new();
        // KeyPair::random() returns a Result, so we need to handle it
        let keypair = match generate_keypair(account.as_str()) {
            Ok(kp) => kp,
            Err(_) => return Err(Sep38Error::Keypairgenerationfailed),
        };

        // Get anchor configuration for authentication
        let anchor_config =
            get_anchor_config_details(&helpers::stellartoml::AnchorService::new(), &slug)
                .await
                .map_err(|_| Sep38Error::AuthFailed)?;

        let jwt = match authenticate(&helpers::stellartoml::AnchorService::new(), &slug, &keypair)
            .await
        {
            Ok(token) => token,
            Err(_) => return Err(Sep38Error::AuthFailed),
        };

        let quote_server = &anchor_config.general_info.anchor_quote_server;
        // Unwrap the Option or provide a default value
        let quote_server_str = quote_server.as_ref().map_or_else(
            || "".to_string(), // Default value if None
            |s| s.to_string(), // Use the string value if Some
        );

        let response = client
            .get(&format!("{}/quote/{}", quote_server_str, quote_id))
            .bearer_auth(jwt)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == 404 {
            Err(Sep38Error::QuoteNotFound)
        } else {
            Err(Sep38Error::InvalidRequest(format!(
                "Status: {}",
                response.status()
            )))
        }
    }
}
