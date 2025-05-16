// src/models/sep31.rs

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::sep31_transactions;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep31_transactions)]
pub struct Sep31Transaction {
    pub id: Uuid,
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub transaction_id: String,
    pub amount: BigDecimal,
    pub asset_code: String,
    pub asset_issuer: Option<String>,
    pub destination_asset: Option<String>,
    pub quote_id: Option<String>,
    pub sender_id: String,
    pub receiver_id: String,
    pub stellar_account_id: Option<String>,
    pub stellar_memo_type: Option<String>,
    pub stellar_memo: Option<String>,
    pub status: String,
    pub status_eta: Option<i64>,
    pub status_message: Option<String>,
    pub amount_in: Option<BigDecimal>,
    pub amount_in_asset: Option<String>,
    pub amount_out: Option<BigDecimal>,
    pub amount_out_asset: Option<String>,
    pub amount_fee: Option<BigDecimal>,
    pub amount_fee_asset: Option<String>,
    pub fee_details: Option<serde_json::Value>,
    pub started_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub stellar_transaction_id: Option<String>,
    pub external_transaction_id: Option<String>,
    pub refunds: Option<serde_json::Value>,
    pub required_info_message: Option<String>,
    pub required_info_updates: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = sep31_transactions)]
pub struct NewSep31Transaction {
    pub transaction_id: String,
    pub account: String,
    pub amount: BigDecimal,
    pub asset_code: String,
    pub asset_issuer: Option<String>,
    pub destination_asset: Option<String>,
    pub quote_id: Option<String>,
    pub sender_id: String,
    pub receiver_id: String,
    pub stellar_account_id: Option<String>,
    pub stellar_memo_type: Option<String>,
    pub stellar_memo: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeDetails {
    pub total: String,
    pub asset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<FeeComponent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeComponent {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Refunds {
    pub amount_refunded: String,
    pub amount_fee: String,
    pub payments: Vec<RefundPayment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundPayment {
    pub id: String,
    pub amount: String,
    pub fee: String,
}