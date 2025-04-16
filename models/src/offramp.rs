use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::*;



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



#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: Uuid,
    pub stellar_address: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: String,
    pub kyc_status: String,
    pub last_login: Option<NaiveDateTime>,
    pub last_kyc_submitted: Option<NaiveDateTime>,
    pub phone: Option<String>,
    pub balance: Option<BigDecimal>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]

pub struct AssetInfo{
    pub asset_code: String,
    pub asset_issuer: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub fee_fixed: Option<f64>,
    pub fee_percent: Option<f64>,
    pub sep12: Option<serde_json::Value>,       // For SEP-12 fields
    pub sep38: Option<serde_json::Value>,       // For SEP-38 contexts
    pub extra_fields: Option<serde_json::Value>, // Additional custom fields
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub stellar_address: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub status: String,
    pub kyc_status: String,
    pub phone: Option<String>,
    pub memo: Option<String>, 
    pub memo_type: Option<String>,
}

#[derive(Debug, diesel::AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub kyc_status: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub last_kyc_submitted: Option<NaiveDateTime>,
    pub phone: Option<String>,
    pub balance: Option<BigDecimal>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
}