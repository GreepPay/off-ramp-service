use form::form::{Sep12KycStatusForm, Sep12CreateKycForm, Sep12UpdateKycForm, Sep12DeleteKycForm};

use rocket::form::Form;

use services::sep12::sep12::{delete_account_kyc,update_account_kyc,create_account_kyc, get_account_kyc,Customer };

pub mod form;


pub async fn get_sep12_kyc_status(
    data: Form<Sep12KycStatusForm<'_>>,
) -> Result<Customer, Box<dyn std::error::Error>> {
    Ok(get_account_kyc(data.slug, data.account, data.memo, data.customer_type).await?)
}

pub async fn create_sep12_kyc(
    data: Form<Sep12CreateKycForm<'_>>,
    fields: Vec<(&str, &str)>,
    files: Vec<(&str, Vec<u8>, &str)>,
) -> Result<Customer, Box<dyn std::error::Error>> {
    Ok(create_account_kyc(data.slug,data.account, data.memo, data.customer_type, fields, files).await?)
}

pub async fn update_sep12_kyc(
    data: Form<Sep12UpdateKycForm<'_>>,
    fields: Vec<(&str, &str)>,
    files: Vec<(&str, Vec<u8>, &str)>,
) -> Result<Customer, Box<dyn std::error::Error>> {
    Ok(update_account_kyc(data.slug,data.customer_id, fields, files).await?)
}

pub async fn delete_sep12_kyc(
    data: Form<Sep12DeleteKycForm<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(delete_account_kyc(data.slug, data.account, data.memo).await?)
}

// pub async fn get_sep12_required_verification(
//     data: Form<Sep12RequiredVerificationForm<'_>>,
// ) -> Result<Vec<Field>, Box<dyn std::error::Error>> {
//     Ok(get_required_verification(data.slug,data.account, data.memo, data.customer_type).await?)
// }