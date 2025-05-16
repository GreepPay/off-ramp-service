pub mod form {
    use rocket::serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep31InfoForm {
        pub slug: String,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep31TransactionRequestForm {
        pub slug: String,
        pub account: String,
        pub amount: String,
        pub asset_code: String,
        #[serde(default)]
        pub asset_issuer: Option<String>,
        #[serde(default)]
        pub destination_asset: Option<String>,
        #[serde(default)]
        pub quote_id: Option<String>,
        pub sender_id: String,
        pub receiver_id: String,
        #[serde(default)]
        pub lang: Option<String>,
        #[serde(default)]
        pub refund_memo: Option<String>,
        #[serde(default)]
        pub refund_memo_type: Option<String>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep31GetTransactionForm {
        pub slug: String,
        pub account: String,
        pub transaction_id: String,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep31UpdateTransactionForm {
        pub slug: String,
        pub account: String,
        pub transaction_id: String,
        pub fields: String, // Expecting a JSON string here
    }

    #[derive(Deserialize, Serialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Sep31SetTransactionCallbackForm {
        pub slug: String,
        pub account: String,
        pub transaction_id: String,
        pub callback_url: String,
    }
}