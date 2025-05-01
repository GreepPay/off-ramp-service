// models/sep6.rs
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::*;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep6_transactions )]
pub struct Sep6Transaction {
    pub id: Uuid,
    pub transaction_id: String,
    pub kind: String, // "withdrawal" or "withdrawal-exchange"
    pub status: String,
    pub status_eta: Option<i64>,
    pub more_info_url: Option<String>,
    pub amount_in: Option<BigDecimal>,
    pub amount_in_asset: Option<String>,
    pub amount_out: Option<BigDecimal>,
    pub amount_out_asset: Option<String>,
    pub amount_fee: Option<BigDecimal>,
    pub amount_fee_asset: Option<String>,
    pub quote_id: Option<String>,
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub withdraw_anchor_account: Option<String>,
    pub withdraw_memo: Option<String>,
    pub withdraw_memo_type: Option<String>,
    pub external_transaction_id: Option<String>,
    pub stellar_transaction_id: Option<String>,
    pub refunded: Option<bool>,
    pub required_info_updates: Option<serde_json::Value>,
    pub required_info_message: Option<String>,
    pub claimable_balance_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub user_action_required_by: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name =  sep6_transactions)]
pub struct NewSep6Transaction {
    pub transaction_id: String,
    pub kind: String,
    pub status: String,
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub quote_id: Option<String>,
    pub amount_in: Option<BigDecimal>,
    pub amount_in_asset: Option<String>,
    pub amount_out: Option<BigDecimal>,
    pub amount_out_asset: Option<String>,
}