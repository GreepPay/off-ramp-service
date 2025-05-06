
use form::form::{ Sep6WithdrawForm,Sep6WithdrawExchangeForm, Sep6TransactionsForm,Sep6TransactionForm};
use rocket::form::Form;
use services::sep6::Sep6Service;



pub mod form;

pub async fn get_sep6_withdraw(
    data: Form<Sep6WithdrawForm<'_>>,
    sep6_service: &Sep6Service,
) -> Result<services::sep6::WithdrawResponse, Box<dyn std::error::Error>> {
    Ok(sep6_service.get_withdraw(
        data.slug,
        data.account,
        data.asset_code,
        data.funding_method,
        data.memo,
        data.on_change_callback,
        data.amount,
        data.country_code,
        data.refund_memo,
        data.refund_memo_type,
    ).await?)
}

pub async fn get_sep6_withdraw_exchange(
    data: Form<Sep6WithdrawExchangeForm<'_>>,
    sep6_service: &Sep6Service,
) -> Result<services::sep6::WithdrawResponse, Box<dyn std::error::Error>> {
    Ok(sep6_service.get_withdraw_exchange(
        data.slug,
        data.account,
        data.source_asset,
        data.destination_asset,
        data.amount,
        data.quote_id,
        data.funding_method,
        data.memo,
        data.on_change_callback,
        data.country_code,
        data.refund_memo,
        data.refund_memo_type,
    ).await?)
}

pub async fn get_sep6_transactions(
    data: Form<Sep6TransactionsForm<'_>>,
    kind: Option<Vec<&str>>,
    sep6_service: &Sep6Service,
) -> Result<Vec<models::sep6::Sep6Transaction>, Box<dyn std::error::Error>> {
    Ok(sep6_service.get_transactions(
        data.slug,
        data.account,
        data.asset_code,
        data.no_older_than,
        data.limit,
        kind,
        data.paging_id,
    ).await?)
}

pub async fn get_sep6_transaction(
    data: Form<Sep6TransactionForm<'_>>,
    sep6_service: &Sep6Service,
) -> Result<models::sep6::Sep6Transaction, Box<dyn std::error::Error>> {
    Ok(sep6_service.get_transaction(
        data.slug,
        data.account,
        data.id,
        data.stellar_transaction_id,
        data.external_transaction_id,
    ).await?)
}