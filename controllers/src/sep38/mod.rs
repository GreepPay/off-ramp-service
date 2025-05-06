use rocket::form::Form;

use services::sep38::Sep38Service;
use form::form::{ Sep38PriceForm, Sep38QuoteForm, Sep38GetQuoteForm };
pub mod form;

pub async fn get_sep38_info(
    sep38_service: &Sep38Service,
) -> Result<Vec<services::sep38::AssetInfo>, Box<dyn std::error::Error>> {
    Ok(sep38_service.get_exchange_info().await?)
}
pub async fn get_sep38_price(
    data: Form<Sep38PriceForm>,
    sep38_service: &Sep38Service,
) -> Result<services::sep38::PriceResponse, Box<dyn std::error::Error>> {
    Ok(sep38_service.get_exchange_prices(
        data.sell_asset.clone(),
        data.buy_asset.clone(),
        data.sell_amount.clone(),
        data.buy_amount.clone(),
        Some(data.sell_delivery_method.clone().unwrap_or_default()),
        Some(data.buy_delivery_method.clone().unwrap_or_default()),
        Some(data.country_code.clone().unwrap_or_default()),
        data.context.clone(),
    ).await?)
}

pub async fn create_sep38_quote(
    data: Form<Sep38QuoteForm<'_>>,
    sep38_service: &Sep38Service,
) -> Result<services::sep38::QuoteResponse, Box<dyn std::error::Error>> {
    Ok(sep38_service.quote_exchange_price(
        data.account,
        data.sell_asset,
        data.buy_asset,
        data.sell_amount,
        data.buy_amount,
        data.expire_after,
        data.sell_delivery_method,
        data.buy_delivery_method,
        data.country_code,
        data.context,
    ).await?)
}

pub async fn get_sep38_quote(
    data: Form<Sep38GetQuoteForm<'_>>,
    sep38_service: &Sep38Service,
) -> Result<services::sep38::QuoteResponse, Box<dyn std::error::Error>> {
    Ok(sep38_service.get_quote(
        data.account,
        data.quote_id,
    ).await?)
}