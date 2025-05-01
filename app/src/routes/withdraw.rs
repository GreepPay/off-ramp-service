use rocket::{form::Form, http::Status, get, response::status, serde::json::Json};
use controllers::{
    withdraw::form::form::{TransactionQueryForm, WithdrawRequestForm},
    api::api::{failure, success, ApiResponse},
};
use models::withdrawal::Sep6Transaction;
use services::withdraw::WithdrawResponse;

#[get("/withdraw", data = "<form>")]
pub async fn withdraw<'r>(
    form: Form<WithdrawRequestForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::withdraw_controller(form)
        .await
        .map(|response| success("Withdrawal initiated", response, Status::Ok))
        .map_err(|e| failure(&format!("Withdrawal failed: {}", e), Status::BadRequest))
}

#[get("/withdraw-exchange", data = "<form>")]
pub async fn withdraw_exchange<'r>(
    form: Form<WithdrawRequestForm<'r>>,

) -> Result<status::Custom<Json<ApiResponse<WithdrawResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::withdraw_exchange_controller(form)
        .await
        .map(|response| success("Exchange withdrawal initiated", response, Status::Ok))
        .map_err(|e| failure(&format!("Exchange withdrawal failed: {}", e), Status::BadRequest))
}

#[get("/transactions", data = "<form>")]
pub async fn get_transactions<'r>(
    form: Form<TransactionQueryForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<Vec<Sep6Transaction>>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::get_transactions_controller(form)
        .await
        .map(|response| success("Transactions retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to retrieve transactions: {}", e), Status::BadRequest))
}

#[get("/transaction?<id>")]
pub async fn get_transaction(
    id: &str,
) -> Result<status::Custom<Json<ApiResponse<Sep6Transaction>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::withdraw::get_transaction_controller(id)
        .await
        .map(|response| success("Transaction details retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to retrieve transaction: {}", e), Status::BadRequest))
}