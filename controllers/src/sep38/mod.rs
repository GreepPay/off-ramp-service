use rocket::form::Form;

use services::sep38::sep38::{get_exchange_info,get_exchange_prices,quote_exchange_price,get_quote,AssetInfo,PriceResponse,QuoteResponse, };
use form::form::{ Sep38PriceForm, Sep38QuoteForm, Sep38GetQuoteForm , Sep38InfoForm};
pub mod form;

pub async fn get_sep38_info(
      data: Form<Sep38InfoForm<'_>>,
) -> Result<Vec<AssetInfo>, Box<dyn std::error::Error>> {
    Ok(get_exchange_info(
        data.slug
    ).await?)
}
pub async fn get_sep38_price(
    data: Form<Sep38PriceForm<'_>>,
) -> Result<PriceResponse, Box<dyn std::error::Error>> {
    Ok(get_exchange_prices(
        data.slug,
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
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    Ok(quote_exchange_price(
        data.slug,
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
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    Ok(get_quote(
        data.slug,
        data.account,
        data.quote_id,
    ).await?)
}