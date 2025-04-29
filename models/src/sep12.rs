use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::offramp_service::*;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = sep12_customers)]
pub struct Sep12Customer {
    pub id: Uuid,
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub customer_type: String,
    pub status: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
    pub kyc_verified: bool,
    pub verification_status: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_verified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = sep12_customers)]
pub struct NewSep12Customer {
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub customer_type: String,
    pub status: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
}

// Customer Files
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep12_customer_files)]
pub struct Sep12CustomerFile {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size: i64,
    pub storage_path: String,
    pub purpose: String,
    pub uploaded_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep12_customer_files)]
pub struct NewSep12CustomerFile {
    pub customer_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size: i64,
    pub storage_path: String,
    pub purpose: String,
}

// Transactions
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep12_transactions)]
pub struct Sep12Transaction {
    pub id: Uuid,
    pub transaction_id: String,
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub customer_id: Uuid,
    pub status: String,
    pub required_fields: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep12_transactions)]
pub struct NewSep12Transaction {
    pub transaction_id: String,
    pub account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub customer_id: Uuid,
    pub status: String,
}

// Callbacks
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep12_callbacks)]
pub struct Sep12Callback {
    pub id: Uuid,
    pub account: String,
    pub url: String,
    pub last_attempt: Option<NaiveDateTime>,
    pub last_status: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep12_callbacks)]
pub struct NewSep12Callback {
    pub account: String,
    pub url: String,
}

// Verifications
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = sep12_verifications)]
pub struct Sep12Verification {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub method: String,
    pub status: String,
    pub verified_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sep12_verifications)]
pub struct NewSep12Verification {
    pub customer_id: Uuid,
    pub method: String,
    pub status: String,
}