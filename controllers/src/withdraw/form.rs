pub mod form {
    use rocket::form::FromForm;
    use services::withdraw::WithdrawRequest; // Corrected import path

    #[derive(FromForm)]
    pub struct WithdrawRequestForm<'r> {
        #[field(name = "asset_code")]
        pub asset_code: &'r str,
        pub account: &'r str,
        pub memo: Option<&'r str>,
        #[field(name = "memo_type")]
        pub memo_type: Option<&'r str>,
        #[field(name = "quote_id")]
        pub quote_id: Option<&'r str>,
        pub amount: Option<&'r str>,
        #[field(name = "funding_method")]
        pub funding_method: Option<&'r str>,
        pub id: Option<&'r str>,
        #[field(name = "customer_type")]
        pub customer_type: Option<&'r str>,
    }

    // Implement From trait for conversion
    impl<'r> From<WithdrawRequestForm<'r>> for WithdrawRequest {
        fn from(form: WithdrawRequestForm<'r>) -> Self {
            Self {
                asset_code: form.asset_code.to_string(),
                account: form.account.to_string(),
                memo: form.memo.map(|s| s.to_string()),
                memo_type: form.memo_type.map(|s| s.to_string()),
                quote_id: form.quote_id.map(|s| s.to_string()),
                amount: form.amount.map(|s| s.to_string()),
                funding_method: form.funding_method.map(|s| s.to_string()),
                id: form.id.map(|s| s.to_string()),
                customer_type: form.customer_type.map(|s| s.to_string()),
            }
        }
    }

    #[derive(FromForm)]
    pub struct TransactionQueryForm<'r> {
        pub account: &'r str,
        #[field(name = "asset_code")]
        pub asset_code: Option<&'r str>,
        pub limit: Option<i64>,
    }
}