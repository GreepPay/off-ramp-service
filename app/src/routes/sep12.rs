pub mod sep12 {
    use controllers::{
        api::api::{ApiResponse, failure, success},
        sep12::form::form::{
            Sep12DeleteKycForm, Sep12FieldsAndFiles, Sep12KycStatusForm, Sep12UpdateKycForm,
        },
        sep12::{create_sep12_kyc, delete_sep12_kyc, get_sep12_kyc_status, update_sep12_kyc},
    };
    use rocket::form::Form;
    use rocket::{delete, http::Status, post, put, response::status, serde::json::Json};
    use services::sep12::sep12::Customer;

    #[post("/customer", data = "<form>")]
    pub async fn get_kyc_status(
        form: Json<Sep12KycStatusForm>,
    ) -> Result<status::Custom<Json<ApiResponse<Customer>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let customer = get_sep12_kyc_status(form).await.map_err(|e| {
            eprintln!("Error getting SEP-12 KYC status: {:?}", e);
            failure("Failed to get KYC status", Status::InternalServerError)
        })?;

        Ok(success(
            "Customer KYC status retrieved successfully",
            customer,
            Status::Ok,
        ))
    }

    #[put("/customer", data = "<data>", format = "multipart/form-data")]
    pub async fn create_kyc(
        data: Form<Sep12FieldsAndFiles<'_>>,
    ) -> Result<status::Custom<Json<ApiResponse<Customer>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let customer = create_sep12_kyc(data.into_inner()).await.map_err(|e| {
            eprintln!("Error creating SEP-12 KYC: {:?}", e);
            failure("Failed to create KYC", Status::InternalServerError)
        })?;

        Ok(success(
            "Customer KYC created successfully",
            customer,
            Status::Accepted,
        ))
    }

    #[post("/customer", data = "<data>", format = "multipart/form-data")]
    pub async fn update_kyc<'v>(
        data: Form<Sep12UpdateKycForm<'v>>,
    ) -> Result<status::Custom<Json<ApiResponse<Customer>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let customer = update_sep12_kyc(data.into_inner()).await.map_err(|e| {
            eprintln!("Error updating SEP-12 KYC: {:?}", e);
            failure("Failed to update KYC", Status::InternalServerError)
        })?;

        Ok(success(
            "Customer KYC updated successfully",
            customer,
            Status::Accepted,
        ))
    }

    #[put("/customer/callback")]
    pub async fn set_callback()
    -> Result<status::Custom<Json<ApiResponse<()>>>, status::Custom<Json<ApiResponse<()>>>> {
        Ok(success("Callback set successfully", (), Status::Ok))
    }

    #[delete("/customer", data = "<form>")]
    pub async fn delete_customer(
        form: Json<Sep12DeleteKycForm>,
    ) -> Result<status::Custom<Json<ApiResponse<()>>>, status::Custom<Json<ApiResponse<()>>>> {
        delete_sep12_kyc(form).await.map_err(|e| {
            eprintln!("Error deleting SEP-12 KYC: {:?}", e);
            failure("Failed to delete KYC", Status::InternalServerError)
        })?;

        Ok(success("Customer deleted successfully", (), Status::Ok))
    }
}
