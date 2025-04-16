use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuoteError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("Quote failed: {0}")]
    QuoteFailed(String),
    #[error("Invalid response status: {0}")]
    InvalidStatus(u16),
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
    ) -> Result<Vec<String>, QuoteError> {
        let response = self.client
            .get(&format!("{}/prices", quote_server))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(QuoteError::InvalidStatus(response.status().as_u16()));
        }

        let pairs = response.json().await?;
        Ok(pairs)
    }

    pub async fn get_quote(
        &self,
        quote_server: &str,
        request: QuoteRequest,
        jwt: Option<&str>,
    ) -> Result<QuoteResponse, QuoteError> {
        let mut req = self.client
            .post(&format!("{}/quote", quote_server))
            .json(&request);

        if let Some(token) = jwt {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            return Err(QuoteError::InvalidStatus(response.status().as_u16()));
        }

        let quote = response.json().await?;
        Ok(quote)
    }
}