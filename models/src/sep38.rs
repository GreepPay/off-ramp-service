// src/models/sep38.rs

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::{sep38_assets,
      sep38_quotes,
    
};

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

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = sep38_assets)]
pub struct NewSep38Asset<'a> {
    pub asset: &'a str,
    pub sell_delivery_methods: Option<&'a serde_json::Value>,
    pub buy_delivery_methods: Option<&'a serde_json::Value>,
    pub country_codes: Option<&'a serde_json::Value>,
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

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = sep38_quotes)]
pub struct NewSep38Quote {
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
    pub context: String,
    pub transaction_id: Option<Uuid>,
}