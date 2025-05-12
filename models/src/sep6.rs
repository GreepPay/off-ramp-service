use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::{
  sep6_refund_payments, sep6_refunds,sep6_transactions
};

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep6_transactions)]
pub struct Sep6Transaction {
    pub id: Uuid,
    pub transaction_id: String,
    pub kind: String,
    pub status: String,
    pub status_eta: Option<i64>,
    pub more_info_url: Option<String>,
    pub amount_in: Option<BigDecimal>,
    pub amount_in_asset: Option<String>,
    pub amount_out: Option<BigDecimal>,
    pub amount_out_asset: Option<String>,
    pub amount_fee: Option<BigDecimal>,
    pub amount_fee_asset: Option<String>,
    pub fee_details: Option<String>,
    pub quote_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub external_extra: Option<String>,
    pub external_extra_text: Option<String>,
    pub deposit_memo: Option<String>,
    pub deposit_memo_type: Option<String>,
    pub withdraw_anchor_account: Option<String>,
    pub withdraw_memo: Option<String>,
    pub withdraw_memo_type: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub user_action_required_by: Option<NaiveDateTime>,
    pub stellar_transaction_id: Option<String>,
    pub external_transaction_id: Option<String>,
    pub message: Option<String>,
    pub refunded: Option<bool>,
    pub refunds: Option<String>,
    pub required_info_message: Option<String>,
    pub required_info_updates: Option<String>,
    pub instructions: Option<String>,
    pub claimable_balance_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = sep6_transactions)]
pub struct NewSep6Transaction {
    pub transaction_id: String,
    pub kind: String,
    pub status: String,
    pub status_eta: Option<i64>,
    pub more_info_url: Option<String>,
    pub amount_in: Option<BigDecimal>,
    pub amount_in_asset: Option<String>,
    pub amount_out: Option<BigDecimal>,
    pub amount_out_asset: Option<String>,
    pub amount_fee: Option<BigDecimal>,
    pub amount_fee_asset: Option<String>,
    pub fee_details: Option<String>,
    pub quote_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub external_extra: Option<String>,
    pub external_extra_text: Option<String>,
    pub deposit_memo: Option<String>,
    pub deposit_memo_type: Option<String>,
    pub withdraw_anchor_account: Option<String>,
    pub withdraw_memo: Option<String>,
    pub withdraw_memo_type: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub user_action_required_by: Option<NaiveDateTime>,
    pub stellar_transaction_id: Option<String>,
    pub external_transaction_id: Option<String>,
    pub message: Option<String>,
    pub refunded: Option<bool>,
    pub refunds: Option<String>,
    pub required_info_message: Option<String>,
    pub required_info_updates: Option<String>,
    pub instructions: Option<String>,
    pub claimable_balance_id: Option<String>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep6_refunds)]
pub struct Sep6Refund {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub amount_refunded: BigDecimal,
    pub amount_fee: BigDecimal,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = sep6_refunds)]
pub struct NewSep6Refund {
    pub transaction_id: Uuid,
    pub amount_refunded: BigDecimal,
    pub amount_fee: BigDecimal,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep6_refund_payments)]
pub struct Sep6RefundPayment {
    pub id: Uuid,
    pub refund_id: Uuid,
    pub payment_id: String,
    pub id_type: String,
    pub amount: BigDecimal,
    pub fee: BigDecimal,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = sep6_refund_payments)]
pub struct NewSep6RefundPayment {
    pub refund_id: Uuid,
    pub payment_id: String,
    pub id_type: String,
    pub amount: BigDecimal,
    pub fee: BigDecimal,
}