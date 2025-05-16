pub mod routes {
    use controllers::api::api::{ApiResponse, failure, success};
    use controllers::sep31::{
        get_sep31_info, create_sep31_transaction, get_sep31_transaction, update_sep31_transaction, set_sep31_transaction_callback,
    };
    use controllers::sep31::form::form::{
        Sep31InfoForm, Sep31TransactionRequestForm, Sep31GetTransactionForm, Sep31UpdateTransactionForm, Sep31SetTransactionCallbackForm
    };
    use rocket::{http::Status, post, response::status, serde::json::Json};
    use services::sep31::sep31::{InfoResponse, TransactionResponse, Transaction};



    #[post("/info", data = "<form>")]
    pub async fn get_sep31_info_route(
        form: Json<Sep31InfoForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<InfoResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let info_response = get_sep31_info(form).await.map_err(|e| {
            eprintln!("Error getting SEP-31 info: {:?}", e);
            failure("Failed to get SEP-31 info", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-31 info retrieved successfully",
            info_response,
            Status::Ok,
        ))
    }

    #[post("/transaction", data = "<form>")]
    pub async fn create_sep31_transaction_route<'r>(
        form: Json<Sep31TransactionRequestForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<TransactionResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let transaction_response = create_sep31_transaction(form).await.map_err(|e| {
            eprintln!("Error creating SEP-31 transaction: {:?}", e);
            failure("Failed to create SEP-31 transaction", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-31 transaction created successfully",
            transaction_response,
            Status::Created,
        ))
    }

    #[post("/get-transaction", data = "<form>")]
    pub async fn get_sep31_transaction_route<'r>(
        form: Json<Sep31GetTransactionForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<Transaction>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let transaction_response = get_sep31_transaction(form).await.map_err(|e| {
            eprintln!("Error getting SEP-31 transaction: {:?}", e);
            failure("Failed to get SEP-31 transaction", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-31 transaction retrieved successfully",
            transaction_response,
            Status::Ok,
        ))
    }

    #[post("/update-transaction", data = "<form>")]
    pub async fn update_sep31_transaction_route<'r>(
        form: Json<Sep31UpdateTransactionForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<Transaction>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let transaction_response = update_sep31_transaction(form).await.map_err(|e| {
            eprintln!("Error updating SEP-31 transaction: {:?}", e);
            failure("Failed to update SEP-31 transaction", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-31 transaction updated successfully",
            transaction_response,
            Status::Ok,
        ))
    }

    #[post("/set-callback", data = "<form>")]
    pub async fn set_sep31_transaction_callback_route<'r>(
        form: Json<Sep31SetTransactionCallbackForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<()>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        set_sep31_transaction_callback(form).await.map_err(|e| {
            eprintln!("Error setting SEP-31 transaction callback: {:?}", e);
            failure("Failed to set SEP-31 transaction callback", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-31 transaction callback set successfully",
            (),
            Status::Ok,
        ))
    }
}