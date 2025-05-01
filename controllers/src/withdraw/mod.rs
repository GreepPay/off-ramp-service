use crate::withdraw::form::form::{TransactionQueryForm, WithdrawRequestForm};
use rocket::form::Form;
use services::withdraw::{Sep6Service, WithdrawResponse};
use models::withdrawal::Sep6Transaction;

pub mod form;

// GET /withdraw
pub async fn withdraw_controller<'r>(
    form: Form<WithdrawRequestForm<'r>>,
) -> Result<WithdrawResponse, String> {
    let request = form.into_inner().into();
    Sep6Service::global().withdraw(request)
        .await
        .map_err(|e| e.to_string())
}

// GET /withdraw-exchange
pub async fn withdraw_exchange_controller<'r>(
    form: Form<WithdrawRequestForm<'r>>,
) -> Result<WithdrawResponse, String> {
    let request = form.into_inner().into();
    Sep6Service::global().withdraw_exchange(request)
        .await
        .map_err(|e| e.to_string())
}

// GET /transactions
pub async fn get_transactions_controller<'r>(
    form: Form<TransactionQueryForm<'r>>,
) -> Result<Vec<Sep6Transaction>, String> {
    let query = form.into_inner();
    Sep6Service::global().get_transactions(
        query.account,
        query.asset_code,
        query.limit,
    )
    .await
    .map_err(|e| e.to_string())
}

// GET /transaction
pub async fn get_transaction_controller(
    id: &str,
) -> Result<Sep6Transaction, String> {
    Sep6Service::global().get_transaction(id)
        .await
        .map_err(|e| e.to_string())
}