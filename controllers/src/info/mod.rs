use rocket::form::Form;
use services::info::{Sep38Client,AssetInfo,PriceAsset,QuoteResponse,QuoteRequest};
use models::info::PriceResponse;
use form::form::{
    PricesRequestForm,
    PriceRequestForm,
    QuoteRequestForm,

};
use rocket::State;
use std::sync::Arc;
use uuid::Uuid;

pub mod form;

// GET /info
pub async fn get_info_controller(
    client: &State<Arc<Sep38Client>>,
) -> Result<Vec<AssetInfo>, Box<dyn std::error::Error>> {
    Ok(client.get_info().await?)
}

// GET /prices
pub async fn get_prices_controller(
    form: Form<PricesRequestForm<'_>>,
    client: &State<Arc<Sep38Client>>,
) -> Result<Vec<PriceAsset>, Box<dyn std::error::Error>> {
    Ok(client.get_prices(
        form.sell_asset,
        form.buy_asset,
        form.sell_amount,
        form.buy_amount,
        form.sell_delivery_method,
        form.buy_delivery_method,
        form.country_code,
    ).await?)
}

// GET /price
pub async fn get_price_controller(
    form: Form<PriceRequestForm<'_>>,
    client: &State<Arc<Sep38Client>>,
) -> Result<PriceResponse, Box<dyn std::error::Error>> {
    Ok(client.get_price(
        form.sell_asset,
        form.buy_asset,
        form.sell_amount,
        form.buy_amount,
        form.sell_delivery_method,
        form.buy_delivery_method,
        form.country_code,
        form.context,
    ).await?)
}


pub async fn create_quote_controller(
    form: Form<QuoteRequestForm<'_>>,
    client: &State<Arc<Sep38Client>>,
    auth_token: &str,
    transaction_id: Option<Uuid>,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    let quote_request: QuoteRequest = form.into_inner().into();
    Ok(client.create_quote(quote_request, auth_token, transaction_id).await?)
}

pub async fn get_quote_controller(
    id: &str,
    client: &State<Arc<Sep38Client>>,
    auth_token: &str,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    Ok(client.get_quote(id, auth_token).await?)
}