use rocket::form::Form;
use services::info::{Sep38Client,AssetInfo, PriceAsset, QuoteResponse, QuoteRequest};
use models::info::PriceResponse;
use form::form::{
    PricesRequestForm,
    PriceRequestForm,
    QuoteRequestForm,
};

use std::error::Error;

pub mod form;

// GET /info
pub async fn get_info_controller() -> Result<Vec<AssetInfo>, Box<dyn Error>> {
    let client = Sep38Client::global();
    Ok(client.get_info().await?)
}

// GET /prices
pub async fn get_prices_controller(
    form: Form<PricesRequestForm<'_>>,
) -> Result<Vec<PriceAsset>, Box<dyn Error>> {
    let client = Sep38Client::global();
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
) -> Result<PriceResponse, Box<dyn Error>> {
    let client = Sep38Client::global();
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
) -> Result<QuoteResponse, Box<dyn Error>> {
    let client = Sep38Client::global();
    let quote_request: QuoteRequest = form.into_inner().into();
    Ok(client.create_quote(quote_request).await?)
}

pub async fn get_quote_controller(
    id: &str,
) -> Result<QuoteResponse, Box<dyn Error>> {
    let client = Sep38Client::global();
    Ok(client.get_quote(id).await?)
}