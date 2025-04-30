use crate::withdraw::form::form::{TransactionQueryForm, WithdrawRequestForm};
use rocket::form::Form;
use rocket::State;
use std::sync::Arc;
use services::withdraw::{Sep6Service, WithdrawResponse};
use models::withdrawal::Sep6Transaction;

pub mod form;

// GET /withdraw
pub async fn withdraw_controller<'r>(
    form: Form<WithdrawRequestForm<'r>>,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<WithdrawResponse, String> {
    let request = form.into_inner().into();
    withdraw_service.withdraw(request, auth_token)
        .await
        .map_err(|e| e.to_string())
}

// GET /withdraw-exchange
pub async fn withdraw_exchange_controller<'r>(
    form: Form<WithdrawRequestForm<'r>>,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<WithdrawResponse, String> {
    let request = form.into_inner().into();
    withdraw_service.withdraw_exchange(request, auth_token)
        .await
        .map_err(|e| e.to_string())
}

// GET /transactions
pub async fn get_transactions_controller<'r>(
    form: Form<TransactionQueryForm<'r>>,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<Vec<Sep6Transaction>, String> {
    let query = form.into_inner();
    withdraw_service.get_transactions(
        query.account,
        auth_token,
        query.asset_code,
        query.limit,
    )
    .await
    .map_err(|e| e.to_string())
}

// GET /transaction
pub async fn get_transaction_controller(
    id: &str,
    withdraw_service: &State<Arc<Sep6Service>>,
    auth_token: &str,
) -> Result<Sep6Transaction, String> {
    withdraw_service.get_transaction(id, auth_token)
        .await
        .map_err(|e| e.to_string())
}