pub mod form {
    use rocket::form::FromForm;
    use services::info::QuoteRequest; 
    use chrono::{Utc, DateTime};

    #[derive(FromForm)]
    pub struct QuoteRequestForm<'r> {
        #[field(name = "sell_asset")]
        pub sell_asset: &'r str,
        #[field(name = "buy_asset")]
        pub buy_asset: &'r str,
        #[field(name = "sell_amount")]
        pub sell_amount: Option<&'r str>,
        #[field(name = "buy_amount")]
        pub buy_amount: Option<&'r str>,
        #[field(name = "expire_after")]
        pub expire_after: Option<String>,
        #[field(name = "sell_delivery_method")]
        pub sell_delivery_method: Option<&'r str>,
        #[field(name = "buy_delivery_method")]
        pub buy_delivery_method: Option<&'r str>,
        #[field(name = "country_code")]
        pub country_code: Option<&'r str>,
        pub context: &'r str,
    }

    // Implement From trait for conversion
    impl<'r> From<QuoteRequestForm<'r>> for QuoteRequest {
        fn from(form: QuoteRequestForm<'r>) -> Self {
            Self {
                sell_asset: form.sell_asset.to_string(),
                buy_asset: form.buy_asset.to_string(),
                sell_amount: form.sell_amount.map(|s| s.to_string()),
                buy_amount: form.buy_amount.map(|s| s.to_string()),
                expire_after: form.expire_after.and_then(|s| {
                              DateTime::parse_from_rfc3339(&s)
                                  .map(|dt| dt.with_timezone(&Utc))
                                  .ok()
                          }),
                sell_delivery_method: form.sell_delivery_method.map(|s| s.to_string()),
                buy_delivery_method: form.buy_delivery_method.map(|s| s.to_string()),
                country_code: form.country_code.map(|s| s.to_string()),
                context: form.context.to_string(),
            }
        }
    }

    // Rest of your form definitions...
    #[derive(FromForm)]
    pub struct PricesRequestForm<'r> {
        pub sell_asset: Option<&'r str>,
        pub buy_asset: Option<&'r str>,
        pub sell_amount: Option<&'r str>,
        pub buy_amount: Option<&'r str>,
        pub sell_delivery_method: Option<&'r str>,
        pub buy_delivery_method: Option<&'r str>,
        pub country_code: Option<&'r str>,
    }

    #[derive(FromForm)]
    pub struct PriceRequestForm<'r> {
        pub sell_asset: &'r str,
        pub buy_asset: &'r str,
        pub sell_amount: Option<&'r str>,
        pub buy_amount: Option<&'r str>,
        pub sell_delivery_method: Option<&'r str>,
        pub buy_delivery_method: Option<&'r str>,
        pub country_code: Option<&'r str>,
        pub context: &'r str,
    }

    #[derive(FromForm)]
    pub struct GetQuoteRequestForm<'r> {
        pub id: &'r str,
    }

}
