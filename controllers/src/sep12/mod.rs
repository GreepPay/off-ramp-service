use form::form::{Sep12KycStatusForm, Sep12CreateKycForm, Sep12UpdateKycForm, Sep12DeleteKycForm, Sep12RequiredVerificationForm};

use rocket::form::Form;

use services::sep12::Sep12Service;

pub mod form;


pub async fn get_sep12_kyc_status(
    data: Form<Sep12KycStatusForm<'_>>,
    sep12_service: &Sep12Service,
) -> Result<services::sep12::Customer, Box<dyn std::error::Error>> {
    Ok(sep12_service.get_account_kyc(data.account, data.memo, data.customer_type).await?)
}

pub async fn create_sep12_kyc(
    data: Form<Sep12CreateKycForm<'_>>,
    fields: Vec<(&str, &str)>,
    files: Vec<(&str, Vec<u8>, &str)>,
    sep12_service: &Sep12Service,
) -> Result<services::sep12::Customer, Box<dyn std::error::Error>> {
    Ok(sep12_service.create_account_kyc(data.account, data.memo, data.customer_type, fields, files).await?)
}

pub async fn update_sep12_kyc(
    data: Form<Sep12UpdateKycForm<'_>>,
    fields: Vec<(&str, &str)>,
    files: Vec<(&str, Vec<u8>, &str)>,
    sep12_service: &Sep12Service,
) -> Result<services::sep12::Customer, Box<dyn std::error::Error>> {
    Ok(sep12_service.update_account_kyc(data.customer_id, fields, files).await?)
}

pub async fn delete_sep12_kyc(
    data: Form<Sep12DeleteKycForm<'_>>,
    sep12_service: &Sep12Service,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(sep12_service.delete_account_kyc(data.account, data.memo).await?)
}

pub async fn get_sep12_required_verification(
    data: Form<Sep12RequiredVerificationForm<'_>>,
    sep12_service: &Sep12Service,
) -> Result<Vec<services::sep12::Field>, Box<dyn std::error::Error>> {
    Ok(sep12_service.get_required_verification(data.account, data.memo, data.customer_type).await?)
}