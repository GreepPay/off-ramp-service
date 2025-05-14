use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use uuid::Uuid;

use crate::schema::offramp_service::{sep12_customer_files, sep12_customers};

#[derive(Queryable, Identifiable, Debug)]
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
    pub date_of_birth: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[diesel(table_name = sep12_customer_files)]
#[diesel(belongs_to(Sep12Customer, foreign_key = customer_id))]
pub struct Sep12CustomerFile {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size: i64,
    pub storage_path: String,
    pub purpose: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
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
    pub date_of_birth: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = sep12_customer_files)]
pub struct NewSep12CustomerFile {
    pub customer_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size: i64,
    pub storage_path: String,
    pub purpose: String,
}
