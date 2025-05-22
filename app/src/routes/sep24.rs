pub mod routes {
    use controllers::api::api::{ApiResponse, failure, success};
    use controllers::sep24::{
        get_sep24_info, interactive_sep24_withdraw, get_sep24_transactions, get_sep24_transaction
    };
    use controllers::sep24::form::form::{
        Sep24InfoForm, Sep24WithdrawForm, Sep24TransactionForm, Sep24TransactionsForm
    };
    use rocket::{http::Status, post, response::status, serde::json::Json};
    use services::sep24::sep24::{InfoResponse, InteractiveResponse, TransactionsResponse, Transaction};

    #[post("/info", data = "<form>")]
    pub async fn get_sep24_info_route(
        form: Json<Sep24InfoForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<InfoResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let info_response = get_sep24_info(form).await.map_err(|e| {
            eprintln!("Error getting SEP-24 info: {:?}", e);
            failure("Failed to get SEP-24 info", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-24 info retrieved successfully",
            info_response,
            Status::Ok,
        ))
    }

    #[post("/withdraw", data = "<form>")]
    pub async fn interactive_sep24_withdraw_route(
        form: Json<Sep24WithdrawForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<InteractiveResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let withdraw_response = interactive_sep24_withdraw(form).await.map_err(|e| {
            eprintln!("Error processing SEP-24 withdraw: {:?}", e);
            failure("Failed to process SEP-24 withdraw", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-24 withdraw initiated successfully",
            withdraw_response,
            Status::Created,
        ))
    }

    #[post("/transactions", data = "<form>")]
    pub async fn get_sep24_transactions_route(
        form: Json<Sep24TransactionsForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<TransactionsResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let transactions_response = get_sep24_transactions(form).await.map_err(|e| {
            eprintln!("Error getting SEP-24 transactions: {:?}", e);
            failure("Failed to get SEP-24 transactions", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-24 transactions retrieved successfully",
            transactions_response,
            Status::Ok,
        ))
    }

    #[post("/transaction", data = "<form>")]
    pub async fn get_sep24_transaction_route(
        form: Json<Sep24TransactionForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<Transaction>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let transaction_response = get_sep24_transaction(form).await.map_err(|e| {
            eprintln!("Error getting SEP-24 transaction: {:?}", e);
            failure("Failed to get SEP-24 transaction", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-24 transaction retrieved successfully",
            transaction_response,
            Status::Ok,
        ))
    }
}