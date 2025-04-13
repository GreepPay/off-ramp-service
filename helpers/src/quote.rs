use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use controllers::api::api::{failure, success, ApiResponse};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
#[derive(Error, Debug)]
pub enum QuoteError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("Quote failed: {0}")]
    QuoteFailed(String),
}

#[derive(Debug, Serialize)]
pub struct QuoteRequest {
    pub sell_asset: String,
    pub buy_asset: String,
    pub sell_amount: Option<String>,
    pub buy_amount: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    pub id: String,
    pub price: String,
    pub sell_asset: String,
    pub buy_asset: String,
    pub sell_amount: String,
    pub buy_amount: String,
    pub expires_at: String,
    pub fee: Option<QuoteFee>,
}

#[derive(Debug, Deserialize)]
pub struct QuoteFee {
    pub total: String,
    pub asset: String,
}

pub struct QuoteService {
    client: Client,
}

impl QuoteService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_supported_pairs(
        &self,
        quote_server: &str,
    ) -> Result<status::Custom<Json<ApiResponse<Vec<String>>>>, status::Custom<Json<ApiResponse<()>>>> {
        let response = self.client
            .get(&format!("{}/prices", quote_server))
            .send()
            .await
            .map_err(|e| {
                failure(
                    &format!("Failed to connect to quote server: {}", e),
                    Status::BadGateway,
                )
            })?;

        if !response.status().is_success() {
            return Err(failure(
                &format!("Quote server returned error: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest),
            ));
        }

        let pairs = response.json().await.map_err(|e| {
            failure(
                &format!("Failed to parse supported pairs: {}", e),
                Status::InternalServerError,
            )
        })?;

        Ok(success(
            "Supported pairs retrieved successfully",
            pairs,
            Status::Ok,
        ))
    }

    pub async fn get_quote(
        &self,
        quote_server: &str,
        request: QuoteRequest,
        jwt: Option<&str>,
    ) -> Result<status::Custom<Json<ApiResponse<QuoteResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
        let mut req = self.client
            .post(&format!("{}/quote", quote_server))
            .json(&request);

        if let Some(token) = jwt {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let response = req.send().await.map_err(|e| {
            failure(
                &format!("Quote request failed: {}", e),
                Status::BadGateway,
            )
        })?;

        if !response.status().is_success() {
            return Err(failure(
                &format!("Quote request rejected: {}", response.status()),
                Status::from_code(response.status().as_u16()).unwrap_or(Status::BadRequest),
            ));
        }

        let quote = response.json().await.map_err(|e| {
            failure(
                &format!("Failed to parse quote response: {}", e),
                Status::InternalServerError,
            )
        })?;

        Ok(success(
            "Quote retrieved successfully",
            quote,
            Status::Ok,
        ))
    }
}