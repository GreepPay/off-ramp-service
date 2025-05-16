use rocket::serde::json::Json;

use services::sep38::sep38::{get_exchange_info,get_exchange_prices,quote_exchange_price,get_quote,AssetInfo,PriceResponse,QuoteResponse, };
use form::form::{ Sep38PriceForm, Sep38QuoteForm, Sep38GetQuoteForm , Sep38InfoForm};
pub mod form;

pub async fn get_sep38_info(
          data: Json<Sep38InfoForm>,
) -> Result<Vec<AssetInfo>, Box<dyn std::error::Error>> {
    Ok(get_exchange_info(
        data.slug.clone()
    ).await?)
}
pub async fn get_sep38_price(
    data: Json<Sep38PriceForm>,
) -> Result<PriceResponse, Box<dyn std::error::Error>> {
    Ok(get_exchange_prices(
        data.slug.clone(),
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
   data: Json<Sep38QuoteForm>,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    Ok(quote_exchange_price(
        data.slug.clone(),
        data.account.clone(),
        data.sell_asset.clone(),
       data.buy_asset.clone(),
        data.sell_amount.clone(),
        data.buy_amount.clone(),
        data.expire_after.clone(),
        data.sell_delivery_method.clone(),
        data.buy_delivery_method.clone(),
        data.country_code.clone(),
        data.context.clone(),
    ).await?)
}

pub async fn get_sep38_quote(
      data: Json<Sep38GetQuoteForm>,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    Ok(get_quote(
        data.slug.clone(),
        data.account.clone(),
        data.quote_id.clone(),
    ).await?)
}