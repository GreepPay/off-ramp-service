use form::form::{Sep12KycStatusForm, Sep12UpdateKycForm, Sep12DeleteKycForm, Sep12FieldsAndFiles};


use rocket::serde::json::Json;
use tokio::io::AsyncReadExt;


use services::sep12::sep12::{delete_account_kyc,update_account_kyc,create_account_kyc, get_account_kyc,Customer, Sep12Error };

pub mod form;


pub async fn get_sep12_kyc_status(
    data: Json<Sep12KycStatusForm>,
) -> Result<Customer, Box<dyn std::error::Error>> {
    Ok(get_account_kyc(
        &data.slug, 
        &data.account, 
        data.memo.as_deref(), 
        data.customer_type.as_deref()
    ).await?)
}


pub async fn create_sep12_kyc<'v>(
    form: Sep12FieldsAndFiles<'v>,
) -> Result<Customer, Sep12Error> {
    let mut files = Vec::new();

    for file_field in &form.files {
        let mut bytes = Vec::new();
        let mut reader = file_field.data.open().await
            .map_err(|e| Sep12Error::InvalidRequest(format!("File open failed: {}", e)))?;
        reader.read_to_end(&mut bytes).await
            .map_err(|e| Sep12Error::InvalidRequest(format!("File read failed: {}", e)))?;

        files.push((
            file_field.name.clone(),
            bytes,
            file_field.content_type.clone()
        ));
    }

    create_account_kyc(
        &form.slug,
        &form.account,
        form.memo.as_deref(),
        &form.customer_type,
        form.fields.clone(),
        files
    ).await
}


pub async fn update_sep12_kyc<'v>(
    form:Sep12UpdateKycForm<'v>,
) -> Result<Customer, Sep12Error> {
    let mut files = Vec::new();

    for file_field in &form.files {

        let mut bytes = Vec::new();
        let mut reader = file_field.data.open().await
            .map_err(|e| Sep12Error::InvalidRequest(format!("File open failed: {}", e)))?;
        reader.read_to_end(&mut bytes).await
            .map_err(|e| Sep12Error::InvalidRequest(format!("File read failed: {}", e)))?;

        files.push((
            file_field.name.clone(), 
            bytes, 
            file_field.content_type.clone()
        ));
    }

    update_account_kyc(
        &form.slug,
        &form.customer_id,
        form.fields.clone(),
        files
    ).await
}

pub async fn delete_sep12_kyc(
    data: Json<Sep12DeleteKycForm>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(delete_account_kyc(
        &data.slug, 
        &data.account, 
        data.memo.as_deref()
    ).await?)
}
