pub mod sep6 {
    use controllers::{
        api::api::{failure, success, ApiResponse},
        sep6::{get_sep6_withdraw, get_sep6_withdraw_exchange, get_sep6_transactions, get_sep6_transaction},
        sep6::form::form::{Sep6TransactionForm, Sep6TransactionsForm, Sep6WithdrawExchangeForm, Sep6WithdrawForm},
    };
    use models::sep6::Sep6Transaction;
    use rocket::{
        form::Form, get, http::Status, response::status, serde::json::Json,
    };
    use services::sep6::{Sep6Service, WithdrawResponse};

    #[get("/withdraw", data = "<form>")]
    pub async fn withdraw<'r>(
        form: Form<Sep6WithdrawForm<'r>>,
        sep6_service: &rocket::State<Sep6Service>,
    ) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
        let response = get_sep6_withdraw(form, sep6_service)
            .await
            .map_err(|e| {
                eprintln!("Error getting withdrawal info: {:?}", e);
                failure("Failed to get withdrawal info", Status::InternalServerError)
            })?;

        Ok(success("Withdrawal info fetched successfully", response, Status::Ok))
    }

    #[get("/withdraw-exchange", data = "<form>")]
    pub async fn withdraw_exchange<'r>(
        form: Form<Sep6WithdrawExchangeForm<'r>>,
        sep6_service: &rocket::State<Sep6Service>,
    ) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
        let response = get_sep6_withdraw_exchange(form, sep6_service)
            .await
            .map_err(|e| {
                eprintln!("Error getting exchange withdrawal info: {:?}", e);
                failure("Failed to get exchange withdrawal info", Status::InternalServerError)
            })?;

        Ok(success("Exchange withdrawal info fetched successfully", response, Status::Ok))
    }

    #[get("/transactions", data = "<form>")]
    pub async fn transactions<'r>(
        form: Form<Sep6TransactionsForm<'r>>,
        sep6_service: &rocket::State<Sep6Service>,
    ) -> Result<status::Custom<Json<ApiResponse<Vec<Sep6Transaction>>>>, status::Custom<Json<ApiResponse<()>>>> {
        let kind = form.kind.as_ref().map(|k| k.split(',').collect::<Vec<&str>>());

        let transactions = get_sep6_transactions(form, kind, sep6_service)
            .await
            .map_err(|e| {
                eprintln!("Error fetching transactions: {:?}", e);
                failure("Failed to fetch transactions", Status::InternalServerError)
            })?;

        Ok(success("Transactions fetched successfully", transactions, Status::Ok))
    }

    #[get("/transaction", data = "<form>")]
    pub async fn transaction<'r>(
        form: Form<Sep6TransactionForm<'r>>,
        sep6_service: &rocket::State<Sep6Service>,
    ) -> Result<status::Custom<Json<ApiResponse<Sep6Transaction>>>, status::Custom<Json<ApiResponse<()>>>> {
        let transaction = get_sep6_transaction(form, sep6_service)
            .await
            .map_err(|e| {
                eprintln!("Error fetching transaction: {:?}", e);
                failure("Failed to fetch transaction", Status::InternalServerError)
            })?;

        Ok(success("Transaction fetched successfully", transaction, Status::Ok))
    }

    // Add additional routes for deposit, info, and other SEP-6 endpoints as needed
}