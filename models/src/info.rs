use diesel::{Insertable, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::*;

// For Queryable structs (reading from DB)
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep38_assets)]
pub struct Sep38Asset {
    pub id: i32,
    pub asset: String,
    pub sell_delivery_methods: Option<serde_json::Value>,
    pub buy_delivery_methods: Option<serde_json::Value>,
    pub country_codes: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// For Insertable structs (writing to DB)
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = sep38_assets)]
pub struct NewSep38Asset<'a> {
    pub asset: &'a str,
    pub sell_delivery_methods: Option<&'a serde_json::Value>,
    pub buy_delivery_methods: Option<&'a serde_json::Value>,
    pub country_codes: Option<&'a serde_json::Value>,
    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceResponse {
    pub sell_asset: String,
    pub buy_asset: String,
    pub price: String,
    pub total_price: String,
    pub sell_amount: Option<String>,
    pub buy_amount: Option<String>,
    pub fee_details: Option<serde_json::Value>,
    pub fee_total: Option<String>,
    pub fee_asset: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep38_quotes)]
pub struct Sep38Quote {
    pub id: Uuid,
    pub original_quote_id: String,
    pub sell_asset: String,
    pub buy_asset: String,
    pub sell_amount: BigDecimal,
    pub buy_amount: BigDecimal,
    pub price: BigDecimal,
    pub total_price: BigDecimal,
    pub fee_total: BigDecimal,
    pub fee_asset: String,
    pub fee_details: Option<serde_json::Value>,
    pub sell_delivery_method: Option<String>,
    pub buy_delivery_method: Option<String>,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub context: String,
    pub transaction_id: Option<Uuid>,
}

#[derive(Debug, Clone, Insertable, Serialize)]
#[diesel(table_name = sep38_quotes)]
pub struct NewSep38Quote<'a> {
    pub original_quote_id: &'a str,
    pub sell_asset: &'a str,
    pub buy_asset: &'a str,
    pub sell_amount: BigDecimal,
    pub buy_amount: BigDecimal,
    pub price: BigDecimal,
    pub total_price: BigDecimal,
    pub fee_total: BigDecimal,
    pub fee_asset: &'a str,
    pub fee_details: Option<&'a serde_json::Value>,
    pub sell_delivery_method: Option<&'a str>,
    pub buy_delivery_method: Option<&'a str>,
    pub expires_at: NaiveDateTime,
    pub context: &'a str,
    pub transaction_id: Option<Uuid>,
}
