use rocket::form::Form;
use form::form::{
    CustomerQueryForm,
    CustomerRequestForm,
    CallbackRequestForm,
    VerificationRequestForm,
    FileUploadForm,
    FileQueryForm,
};
use rocket::State;
use std::sync::Arc;
use services::kyc::{Sep12Service, CustomerResponse, CallbackResponse, FileResponse, FileListResponse};

pub mod form;

// GET /customer
pub async fn get_customer_controller<'r>(
    form: Form<CustomerQueryForm<'r>>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<CustomerResponse, String> {
    let query = form.into_inner();
    kyc.get_customer(query.into())
        .await
        .map_err(|e| e.to_string())
}

// PUT /customer
pub async fn put_customer_controller<'r>(
    form: Form<CustomerRequestForm<'r>>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<CustomerResponse, String> {
    let request = form.into_inner();
    kyc.put_customer(request.into())
        .await
        .map_err(|e| e.to_string())
}

// PUT /customer/callback
pub async fn set_callback_controller<'r>(
    form: Form<CallbackRequestForm<'r>>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<CallbackResponse, String> {
    let request = form.into_inner();
    kyc.set_callback(request.into())
        .await
        .map_err(|e| e.to_string())
}

// PUT /customer/verification
pub async fn submit_verification_controller<'r>(
    form: Form<VerificationRequestForm<'r>>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<CustomerResponse, String> {
    let request = form.into_inner();
    kyc.submit_verification(request.into())
        .await
        .map_err(|e| e.to_string())
}

// DELETE /customer/[account]
pub async fn delete_customer_controller<'r>(
    account: &str,
    memo: Option<&str>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<(), String> {
    kyc.delete_customer(account, memo)
        .await
        .map_err(|e| e.to_string())
}

// POST /customer/files
pub async fn upload_file_controller<'r>(
    form: Form<FileUploadForm<'r>>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<FileResponse, String> {
    let upload = form.into_inner();
    kyc.upload_file(upload.into())
        .await
        .map_err(|e| e.to_string())
}

// GET /customer/files
pub async fn list_files_controller<'r>(
    form: Form<FileQueryForm<'r>>,
    kyc: &State<Arc<Sep12Service>>,
) -> Result<FileListResponse, String> {
    let query = form.into_inner();
    kyc.list_files(query.into())
        .await
        .map_err(|e| e.to_string())
}