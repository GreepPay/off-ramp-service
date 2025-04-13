use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use bigdecimal::BigDecimal;

use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = offramp_transactions)]
pub struct OfframpTransaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_id: String,
    pub amount: BigDecimal,
    pub dest_currency: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = offramp_transactions)]
pub struct NewOfframpTransaction<'a> {
    pub account_id: Uuid,
    pub transaction_id: &'a str,
    pub amount: BigDecimal,
    pub dest_currency: &'a str,
    pub status: &'a str,
}

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = offramp_quotes)]
pub struct OfframpQuote {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub quote_id: String,
    pub sell_asset: String,
    pub buy_asset: String,
    pub sell_amount: BigDecimal,
    pub buy_amount: BigDecimal,
    pub price: BigDecimal,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = offramp_quotes)]
pub struct NewOfframpQuote<'a> {
    pub transaction_id: Uuid,
    pub quote_id: &'a str,
    pub sell_asset: &'a str,
    pub buy_asset: &'a str,
    pub sell_amount: BigDecimal,
    pub buy_amount: BigDecimal,
    pub price: BigDecimal,
    pub expires_at: NaiveDateTime,
}