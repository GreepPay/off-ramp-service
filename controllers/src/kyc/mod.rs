use rocket::form::Form;
use form::form::{
    CustomerQueryForm,
    CustomerRequestForm,
    CallbackRequestForm,
    VerificationRequestForm,
    FileUploadForm,
    FileQueryForm,
};
use services::kyc::{Sep12Service, CustomerResponse, CallbackResponse, FileResponse, FileListResponse};

pub mod form;

// GET /customer
pub async fn get_customer_controller<'r>(
    form: Form<CustomerQueryForm<'r>>,
) -> Result<CustomerResponse, String> {
    let kyc = Sep12Service::global();
    let query = form.into_inner();
    kyc.get_customer(query.into())
        .await
        .map_err(|e| e.to_string())
}

pub async fn put_customer_controller<'r>(
    form: Form<CustomerRequestForm<'r>>,
) -> Result<CustomerResponse, String> {
    let kyc = Sep12Service::global();
    let request = form.into_inner();
    kyc.put_customer(request.into())
        .await
        .map_err(|e| e.to_string())
}

pub async fn set_callback_controller<'r>(
    form: Form<CallbackRequestForm<'r>>,
) -> Result<CallbackResponse, String> {
    let kyc = Sep12Service::global();
    let request = form.into_inner();
    kyc.set_callback(request.into())
        .await
        .map_err(|e| e.to_string())
}

pub async fn submit_verification_controller<'r>(
    form: Form<VerificationRequestForm<'r>>,
) -> Result<CustomerResponse, String> {
    let kyc = Sep12Service::global();
    let request = form.into_inner();
    kyc.submit_verification(request.into())
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_customer_controller(
    account: &str,
    memo: Option<&str>,
) -> Result<(), String> {
    let kyc = Sep12Service::global();
    kyc.delete_customer(account, memo)
        .await
        .map_err(|e| e.to_string())
}

pub async fn upload_file_controller<'r>(
    form: Form<FileUploadForm<'r>>,
) -> Result<FileResponse, String> {
    let kyc = Sep12Service::global();
    let upload = form.into_inner();
    kyc.upload_file(upload.into())
        .await
        .map_err(|e| e.to_string())
}

pub async fn list_files_controller<'r>(
    form: Form<FileQueryForm<'r>>,
) -> Result<FileListResponse, String> {
    let kyc = Sep12Service::global();
    let query = form.into_inner();
    kyc.list_files(query.into())
        .await
        .map_err(|e| e.to_string())
}
