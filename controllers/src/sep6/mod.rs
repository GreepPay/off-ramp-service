use form::form::{
    Sep6InfoForm, Sep6TransactionForm, Sep6TransactionsForm, Sep6WithdrawExchangeForm,
    Sep6WithdrawForm,
};
use rocket::serde::json::Json;
use services::sep6::sep6::{
    InfoResponse, WithdrawResponse, get_anchor_info, get_transaction, get_transactions,
    get_withdraw, get_withdraw_exchange,
};

pub mod form;

pub async fn get_sep6_info(
    data: Json<Sep6InfoForm<'_>>,
) -> Result<InfoResponse, Box<dyn std::error::Error>> {
    Ok(get_anchor_info(data.slug).await?)
}

pub async fn get_sep6_withdraw(
    data: Json<Sep6WithdrawForm<'_>>,
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
    )
    .await?)
}

pub async fn get_sep6_withdraw_exchange(
    data: Json<Sep6WithdrawExchangeForm<'_>>,
) -> Result<WithdrawResponse, Box<dyn std::error::Error>> {
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
    )
    .await?)
}

pub async fn get_sep6_transactions(
    data: Json<Sep6TransactionsForm<'_>>,
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
    )
    .await?)
}

pub async fn get_sep6_transaction(
    data: Json<Sep6TransactionForm<'_>>,
) -> Result<models::sep6::Sep6Transaction, Box<dyn std::error::Error>> {
    Ok(get_transaction(
        data.slug,
        data.account,
        data.id,
        data.stellar_transaction_id,
        data.external_transaction_id,
    )
    .await?)
}
