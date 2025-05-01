use rocket::{get, post, put, delete, response::status, http::Status};
use rocket::form::Form;
use rocket::serde::json::Json;


use controllers::{
    kyc::form::form::{
        CustomerQueryForm,
        CustomerRequestForm,
        CallbackRequestForm,
        VerificationRequestForm,
        FileUploadForm,
        FileQueryForm,
    },
    api::api::{failure, success, ApiResponse},
};
use services::kyc::{CustomerResponse, CallbackResponse, FileResponse, FileListResponse};



#[get("/customer", data = "<form>")]
pub async fn get_customer<'r>(
    form: Form<CustomerQueryForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<CustomerResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
     controllers::kyc::get_customer_controller(form)
        .await
        .map(|response| success("Customer details retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to retrieve customer: {}", e), Status::BadRequest))
}

#[put("/customer", data = "<form>")]
pub async fn put_customer<'r>(
    form: Form<CustomerRequestForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<CustomerResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
       controllers::kyc::put_customer_controller(form)
        .await
        .map(|response| success("Customer updated", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to update customer: {}", e), Status::BadRequest))
}

#[put("/customer/callback", data = "<form>")]
pub async fn set_callback<'r>(
    form: Form<CallbackRequestForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<CallbackResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
       controllers::kyc::set_callback_controller(form)
        .await
        .map(|response| success("Callback set", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to set callback: {}", e), Status::BadRequest))
}

#[put("/customer/verification", data = "<form>")]
pub async fn submit_verification<'r>(
    form: Form<VerificationRequestForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<CustomerResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
       controllers::kyc::submit_verification_controller(form)
        .await
        .map(|response| success("Verification submitted", response, Status::Ok))
        .map_err(|e| failure(&format!("Verification failed: {}", e), Status::BadRequest))
}

#[delete("/customer/<account>?<memo>")]
pub async fn delete_customer(
    account: &str,
    memo: Option<&str>,
) -> Result<status::Custom<Json<ApiResponse<()>>>, status::Custom<Json<ApiResponse<()>>>> {
       controllers::kyc::delete_customer_controller(account, memo)
        .await
        .map(|_| success("Customer deleted", (), Status::Ok))
        .map_err(|e| failure(&format!("Failed to delete customer: {}", e), Status::BadRequest))
}

#[post("/customer/files", data = "<form>")]
pub async fn upload_file<'r>(
    form: Form<FileUploadForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<FileResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::kyc::upload_file_controller(form)
        .await
        .map(|response| success("File uploaded", response, Status::Ok))
        .map_err(|e| failure(&format!("File upload failed: {}", e), Status::BadRequest))
}

#[get("/customer/files", data = "<form>")]
pub async fn list_files<'r>(
    form: Form<FileQueryForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<FileListResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::kyc::list_files_controller(form)
        .await
        .map(|response| success("Files listed", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to list files: {}", e), Status::BadRequest))
}