use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::schema::offramp_service::sep24_withdrawals;


#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Sep24Withdrawal {
    pub id: Uuid,
    pub transaction_id: String,
    pub asset_code: String,
    pub asset_issuer: Option<String>,
    pub amount: Option<BigDecimal>,
    pub account: Option<String>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub status: String,
    pub started_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub stellar_transaction_id: Option<String>,
    pub external_transaction_id: Option<String>,
    pub quote_id: Option<String>,
    pub withdraw_anchor_account: Option<String>,
    pub withdraw_memo: Option<String>,
    pub withdraw_memo_type: Option<String>,
    pub wallet_name: Option<String>,
    pub wallet_url: Option<String>,
    pub lang: Option<String>,
    pub refund_memo: Option<String>,
    pub refund_memo_type: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sep24_withdrawals)]
pub struct NewSep24Withdrawal {
    pub transaction_id: String,
    pub asset_code: String,
    pub asset_issuer: Option<String>,
    pub amount: Option<BigDecimal>,
    pub account: Option<String>,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub status: String,
    pub started_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub stellar_transaction_id: Option<String>,
    pub external_transaction_id: Option<String>,
    pub quote_id: Option<String>,
    pub withdraw_anchor_account: Option<String>,
    pub withdraw_memo: Option<String>,
    pub withdraw_memo_type: Option<String>,
    pub wallet_name: Option<String>,
    pub wallet_url: Option<String>,
    pub lang: Option<String>,
    pub refund_memo: Option<String>,
    pub refund_memo_type: Option<String>,
}