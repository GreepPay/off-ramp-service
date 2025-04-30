use rocket::{form::Form, http::Status, post, response::status, serde::json::Json};
use controllers::{
    withdraw::form::form::{WithdrawRequestForm,TransactionQueryForm},
    api::api::{failure, success, ApiResponse},
};



// Withdraw Routes
#[get("/withdraw", data = "<form>")]
pub async fn withdraw<'r>(
    form: Form<WithdrawRequestForm<'r>>,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::withdraw_controller(form, withdraw_service, auth_token)
        .await
        .map(|response| success("Withdraw successful", response, Status::Ok))
        .map_err(|e| failure(&format!("Withdraw failed: {}", e), Status::BadRequest))
}

#[get("/withdraw-exchange", data = "<form>")]
pub async fn withdraw_exchange<'r>(
    form: Form<WithdrawRequestForm<'r>>,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::withdraw_exchange_controller(form, withdraw_service, auth_token)
        .await
        .map(|response| success("Exchange withdraw successful", response, Status::Ok))
        .map_err(|e| failure(&format!("Exchange withdraw failed: {}", e), Status::BadRequest))
}

#[get("/transactions", data = "<form>")]
pub async fn get_transactions<'r>(
    form: Form<TransactionQueryForm<'r>>,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<status::Custom<Json<ApiResponse<Vec<Sep6Transaction>>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::get_transactions_controller(form, withdraw_service, auth_token)
        .await
        .map(|response| success("Transactions retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get transactions: {}", e), Status::BadRequest))
}

#[get("/transaction/<id>")]
pub async fn get_transaction(
    id: &str,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<status::Custom<Json<ApiResponse<Sep6Transaction>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::get_transaction_controller(id, withdraw_service, auth_token)
        .await
        .map(|response| success("Transaction retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get transaction: {}", e), Status::BadRequest))
}
