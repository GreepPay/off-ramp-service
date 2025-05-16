pub mod sep12 {
    use controllers::{
        api::api::{failure, success, ApiResponse},
        sep12::form::form::{
            Sep12KycStatusForm, Sep12CreateKycForm, Sep12UpdateKycForm, Sep12DeleteKycForm,Sep12FieldsAndFiles

        },
        sep12::{
            create_sep12_kyc, delete_sep12_kyc, get_sep12_kyc_status, 
            update_sep12_kyc,
        },
    };
    use services::sep12::sep12::Customer;
    use rocket::form::Form;
    use rocket::{delete, get, http::Status, post, put, response::status, serde::json::Json};


    #[get("/customer", data = "<form>")]
    pub async fn get_kyc_status(
        form: Json<Sep12KycStatusForm>,
    ) -> Result<status::Custom<Json<ApiResponse<Customer>>>, status::Custom<Json<ApiResponse<()>>>> {
        let customer = get_sep12_kyc_status(form)
            .await
            .map_err(|e| {
                eprintln!("Error getting SEP-12 KYC status: {:?}", e);
                failure("Failed to get KYC status", Status::InternalServerError)
            })?;

        Ok(success(
            "Customer KYC status retrieved successfully",
            customer,
            Status::Ok,
        ))
    }

    #[put("/customer")]
    pub async fn create_kyc(
        data: Json<Sep12CreateKycForm>,
        form: Form<Sep12FieldsAndFiles<'_>>,
    ) -> Result<status::Custom<Json<ApiResponse<Customer>>>, status::Custom<Json<ApiResponse<()>>>> {
        let data_tuple = Json((data.into_inner(), form.into_inner()));
        let customer = create_sep12_kyc(data_tuple)
            .await
            .map_err(|e| {
                eprintln!("Error creating SEP-12 KYC: {:?}", e);
                failure("Failed to create KYC", Status::InternalServerError)
            })?;

        Ok(success(
            "Customer KYC created successfully",
            customer,
            Status::Accepted,
        ))
    }


    #[post("/customer")]
    pub async fn update_kyc(
        data: Json<Sep12UpdateKycForm>,
        form: Form<Sep12FieldsAndFiles<'_>>,
    ) -> Result<status::Custom<Json<ApiResponse<Customer>>>, status::Custom<Json<ApiResponse<()>>>> {
        let data_tuple = Json((data.into_inner(), form.into_inner()));
        let customer = update_sep12_kyc(data_tuple)
            .await
            .map_err(|e| {
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
    pub async fn set_callback(
    ) -> Result<status::Custom<Json<ApiResponse<()>>>, status::Custom<Json<ApiResponse<()>>>> {
        Ok(success(
            "Callback set successfully", 
            (), 
            Status::Ok
        ))
    }

    #[delete("/customer", data = "<form>")]
    pub async fn delete_customer(
        form: Json<Sep12DeleteKycForm>,
    ) -> Result<status::Custom<Json<ApiResponse<()>>>, status::Custom<Json<ApiResponse<()>>>> {
        delete_sep12_kyc(form)
            .await
            .map_err(|e| {
                eprintln!("Error deleting SEP-12 KYC: {:?}", e);
                failure("Failed to delete KYC", Status::InternalServerError)
            })?;

        Ok(success(
            "Customer deleted successfully",
            (),
            Status::Ok,
        ))
    }

}