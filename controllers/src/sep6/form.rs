pub mod form {
    use rocket::form::FromForm;

    #[derive(FromForm)]
    pub struct Sep6WithdrawForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub asset_code: &'r str,
        pub funding_method: &'r str,
        pub memo: Option<&'r str>,
        pub on_change_callback: Option<&'r str>,
        pub amount: Option<&'r str>,
        pub country_code: Option<&'r str>,
        pub refund_memo: Option<&'r str>,
        pub refund_memo_type: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct Sep6WithdrawExchangeForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub source_asset: &'r str,
        pub destination_asset: &'r str,
        pub amount: &'r str,
        pub quote_id: Option<&'r str>,
        pub funding_method: &'r str,
        pub memo: Option<&'r str>,
        pub on_change_callback: Option<&'r str>,
        pub country_code: Option<&'r str>,
        pub refund_memo: Option<&'r str>,
        pub refund_memo_type: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct Sep6TransactionsForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub asset_code: Option<&'r str>,
        pub no_older_than: Option<&'r str>,
        pub limit: Option<i32>,
        #[field(name = "kind")]
        pub kind: Option<&'r str>, 
        pub paging_id: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct Sep6TransactionForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub id: Option<&'r str>,
        pub stellar_transaction_id: Option<&'r str>,
        pub external_transaction_id: Option<&'r str>,
    }
}