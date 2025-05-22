    use rocket::serde::json::Json;

    use services::sep24::sep24::{get_info, interactive_withdraw, get_transactions, get_transaction, InfoResponse, InteractiveResponse, TransactionsResponse, Transaction};
    use form::form::{ Sep24InfoForm, Sep24WithdrawForm, Sep24TransactionForm, Sep24TransactionsForm};
    pub mod form;

    pub async fn get_sep24_info(
        data: Json<Sep24InfoForm>,
    ) -> Result<InfoResponse, Box<dyn std::error::Error>> {
        Ok(get_info(
            data.slug.clone(),
            data.lang.clone()
        ).await?)
    }

    pub async fn interactive_sep24_withdraw(
        data: Json<Sep24WithdrawForm>,
    ) -> Result<InteractiveResponse, Box<dyn std::error::Error>> {
        Ok(interactive_withdraw(
            data.slug.clone(),
            data.account.clone(),
            data.asset_code.clone(),
            data.asset_issuer.clone(),
            data.amount.clone(),
            None,
            None,
            data.wallet_name.clone(),
            data.wallet_url.clone(),
            data.lang.clone(),
            data.refund_memo.clone(),
            data.refund_memo_type.clone(),
            data.quote_id.clone(),
        ).await?)
    }

    pub async fn get_sep24_transactions(
        data: Json<Sep24TransactionsForm>,
    ) -> Result<TransactionsResponse, Box<dyn std::error::Error>> {
        Ok(get_transactions(
            data.slug.clone(),
            data.account.clone(),
            data.asset_code.clone(),
            data.no_older_than.clone(),
            data.limit.clone(),
            data.kind.clone(),
            data.paging_id.clone(),
            data.lang.clone(),
        ).await?)
    }

    pub async fn get_sep24_transaction(
        data: Json<Sep24TransactionForm>,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        Ok(get_transaction(
            data.slug.clone(),
            data.account.clone(),
            data.id.clone(),
            data.stellar_transaction_id.clone(),
            data.external_transaction_id.clone(),
            data.lang.clone(),
        ).await?)
    }
