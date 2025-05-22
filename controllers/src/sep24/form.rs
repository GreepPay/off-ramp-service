pub mod form {
    use rocket::serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep24InfoForm {
        pub slug: String,
        #[serde(default)]
        pub lang: Option<String>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep24WithdrawForm {
        pub slug: String,
        pub account: String,
        pub asset_code: String,
        #[serde(default)]
        pub asset_issuer: Option<String>,
        #[serde(default)]
        pub amount: Option<String>,
        #[serde(default)]
        pub wallet_name: Option<String>,
        #[serde(default)]
        pub wallet_url: Option<String>,
        #[serde(default)]
        pub lang: Option<String>,
        #[serde(default)]
        pub refund_memo: Option<String>,
        #[serde(default)]
        pub refund_memo_type: Option<String>,
        #[serde(default)]
        pub quote_id: Option<String>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep24TransactionForm {
        pub slug: String,
        pub account: String,
        #[serde(default)]
        pub id: Option<String>,
        #[serde(default)]
        pub stellar_transaction_id: Option<String>,
        #[serde(default)]
        pub external_transaction_id: Option<String>,
        #[serde(default)]
        pub lang: Option<String>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep24TransactionsForm {
        pub slug: String,
        pub account: String,
        #[serde(default)]
        pub asset_code: Option<String>,
        #[serde(default)]
        pub no_older_than: Option<String>,
        #[serde(default)]
        pub limit: Option<i32>,
        #[serde(default)]
        pub kind: Option<String>,
        #[serde(default)]
        pub paging_id: Option<String>,
        #[serde(default)]
        pub lang: Option<String>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep24InteractiveResponseForm {
        pub slug: String,
        pub account: String,
        pub transaction_id: String,
        #[serde(default)]
        pub lang: Option<String>,
    }
}