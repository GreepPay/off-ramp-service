use rocket::form::FromForm;
use serde_json::Value;

#[derive(FromForm)]
pub struct GetJwtForm<'a> {
    pub account_id: &'a str,
    pub secret_key: &'a str,
}

#[derive(FromForm)]
pub struct OfframpForm<'a> {
    pub account_id: &'a str,
    pub secret_key: &'a str,
    pub amount: f64,
    pub dest_currency: &'a str,
    pub kyc_fields: Option<Value>,
}

#[derive(FromForm)]
pub struct TransactionStatusForm<'a> {
    pub transaction_id: &'a str,
    pub account_id: &'a str,
    pub secret_key: &'a str,
}

#[derive(FromForm)]
pub struct AssetInfoForm<'a> {
    pub asset_code: &'a str,
    pub operation_type: Option<&'a str>,
}

#[derive(FromForm)]
pub struct UtilizationForm<'a> {
    pub asset_code: &'a str,
    pub account: Option<&'a str>,
}

#[derive(FromForm)]
pub struct TransactionQueryForm<'a> {
    pub account_id: &'a str,
    pub secret_key: &'a str,
}

#[derive(FromForm)]
pub struct SingleTransactionQueryForm<'a> {
    pub transaction_id: &'a str,
    pub account_id: &'a str,
    pub secret_key: &'a str,
}