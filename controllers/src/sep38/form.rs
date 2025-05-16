
    pub mod form {
        use rocket::serde::{Deserialize, Serialize};
    
        #[derive(Deserialize, Serialize)]
        #[serde(crate = "rocket::serde")]
        pub struct Sep38PriceForm {
            pub slug: String,
            pub sell_asset: String,
            pub buy_asset: String,
            pub sell_amount: String,
            pub buy_amount: String,
            #[serde(default)]
            pub sell_delivery_method: Option<String>,
            #[serde(default)]
            pub buy_delivery_method: Option<String>,
            #[serde(default)]
            pub country_code: Option<String>,
            pub context: String,
        }
    
        #[derive(Deserialize, Serialize)]
        #[serde(crate = "rocket::serde")]
        pub struct Sep38QuoteForm {
            pub slug: String,
            pub account: String,
            pub sell_asset: String,
            pub buy_asset: String,
            #[serde(default)]
            pub sell_amount: Option<String>,
            #[serde(default)]
            pub buy_amount: Option<String>,
            #[serde(default)]
            pub expire_after: Option<String>,
            #[serde(default)]
            pub sell_delivery_method: Option<String>,
            #[serde(default)]
            pub buy_delivery_method: Option<String>,
            #[serde(default)]
            pub country_code: Option<String>,
            pub context: String,
        }
        
        #[derive(Deserialize, Serialize)]
        #[serde(crate = "rocket::serde")]
        pub struct Sep38GetQuoteForm {
            pub slug: String,
            pub account: String,
            pub quote_id: String,
        }
    
        #[derive(Deserialize, Serialize)]
        #[serde(crate = "rocket::serde")]
        pub struct Sep38InfoForm {
            pub slug: String,
        }

}