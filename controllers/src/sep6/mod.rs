use form::form::{ Sep6WithdrawForm, Sep6WithdrawExchangeForm, Sep6TransactionsForm, Sep6TransactionForm, Sep6InfoForm};
use rocket::form::Form;
use services::sep6::sep6::{get_anchor_info,get_transactions,get_transaction,get_withdraw_exchange, get_withdraw, WithdrawResponse};




pub mod form;

pub async fn get_sep6_withdraw(
    data: Form<Sep6WithdrawForm<'_>>,
) -> Result<WithdrawResponse, Box<dyn std::error::Error>> {
    Ok(get_withdraw(
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
) -> Result<WithdrawResponse,Box<dyn std::error::Error>> {
    Ok(get_withdraw_exchange(
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
) -> Result<Vec<models::sep6::Sep6Transaction>, Box<dyn std::error::Error>> {
    Ok(get_transactions(
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
) -> Result<models::sep6::Sep6Transaction, Box<dyn std::error::Error>> {
    Ok(get_transaction(
        data.slug,
        data.account,
        data.id,
        data.stellar_transaction_id,
        data.external_transaction_id,
    ).await?)
}