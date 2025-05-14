pub mod sep6 {
    use controllers::{
        api::api::{ApiResponse, failure, success},
        sep6::form::form::{
            Sep6InfoForm, Sep6TransactionForm, Sep6TransactionsForm, Sep6WithdrawExchangeForm,
            Sep6WithdrawForm,
        },
        sep6::{
            get_sep6_info, get_sep6_transaction, get_sep6_transactions, get_sep6_withdraw,
            get_sep6_withdraw_exchange,
        },
    };
    use models::sep6::Sep6Transaction;
    use rocket::{http::Status, post, response::status, serde::json::Json};
    use services::sep6::sep6::{InfoResponse, WithdrawResponse};

    #[post("/info", data = "<form>")]
    pub async fn anchorinfo<'r>(
        form: Json<Sep6InfoForm<'r>>,
    ) -> Result<
        status::Custom<Json<ApiResponse<InfoResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let response = get_sep6_info(form).await.map_err(|e| {
            eprintln!("Error getting withdrawal info: {:?}", e);
            failure("Failed to get withdrawal info", Status::InternalServerError)
        })?;

        Ok(success(
            "Withdrawal info fetched successfully",
            response,
            Status::Ok,
        ))
    }

    #[post("/withdraw", data = "<form>")]
    pub async fn withdraw<'r>(
        form: Json<Sep6WithdrawForm<'r>>,
    ) -> Result<
        status::Custom<Json<ApiResponse<WithdrawResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let response = get_sep6_withdraw(form).await.map_err(|e| {
            eprintln!("Error getting withdrawal info: {:?}", e);
            failure("Failed to get withdrawal info", Status::InternalServerError)
        })?;

        Ok(success(
            "Withdrawal info fetched successfully",
            response,
            Status::Ok,
        ))
    }

    #[post("/withdraw-exchange", data = "<form>")]
    pub async fn withdraw_exchange<'r>(
        form: Json<Sep6WithdrawExchangeForm<'r>>,
    ) -> Result<
        status::Custom<Json<ApiResponse<WithdrawResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let response = get_sep6_withdraw_exchange(form).await.map_err(|e| {
            eprintln!("Error getting exchange withdrawal info: {:?}", e);
            failure(
                "Failed to get exchange withdrawal info",
                Status::InternalServerError,
            )
        })?;

        Ok(success(
            "Exchange withdrawal info fetched successfully",
            response,
            Status::Ok,
        ))
    }

    #[post("/transactions", data = "<form>")]
    pub async fn transactions<'r>(
        form: Json<Sep6TransactionsForm<'r>>,
    ) -> Result<
        status::Custom<Json<ApiResponse<Vec<Sep6Transaction>>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let kind = form
            .kind
            .as_ref()
            .map(|k| k.split(',').collect::<Vec<&str>>());

        let transactions = get_sep6_transactions(form, kind).await.map_err(|e| {
            eprintln!("Error fetching transactions: {:?}", e);
            failure("Failed to fetch transactions", Status::InternalServerError)
        })?;

        Ok(success(
            "Transactions fetched successfully",
            transactions,
            Status::Ok,
        ))
    }

    #[post("/transaction", data = "<form>")]
    pub async fn transaction<'r>(
        form: Json<Sep6TransactionForm<'r>>,
    ) -> Result<
        status::Custom<Json<ApiResponse<Sep6Transaction>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let transaction = get_sep6_transaction(form).await.map_err(|e| {
            eprintln!("Error fetching transaction: {:?}", e);
            failure("Failed to fetch transaction", Status::InternalServerError)
        })?;

        Ok(success(
            "Transaction fetched successfully",
            transaction,
            Status::Ok,
        ))
    }

    // Add additional routes for deposit, info, and other SEP-6 endpoints as needed
}
