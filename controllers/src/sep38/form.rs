pub mod form {
    use rocket::form::FromForm;
    
    #[derive(FromForm)]
    pub struct Sep38PriceForm<'r> {
        pub slug: &'r str,
        pub sell_asset: String,
        pub buy_asset:String,
        pub sell_amount: String,
        pub buy_amount: String,
        pub sell_delivery_method: Option<String>,
        pub buy_delivery_method: Option<String>,
        pub country_code: Option<String>,
        pub context: String,
    }

    #[derive(FromForm)]
    pub struct Sep38QuoteForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub sell_asset: &'r str,
        pub buy_asset: &'r str,
        pub sell_amount: Option<&'r str>,
        pub buy_amount: Option<&'r str>,
        pub expire_after: Option<&'r str>,
        pub sell_delivery_method: Option<&'r str>,
        pub buy_delivery_method: Option<&'r str>,
        pub country_code: Option<&'r str>,
        pub context: &'r str,
    }
    
    #[derive(FromForm)]
    pub struct Sep38GetQuoteForm<'r> {
        pub slug: &'r str,
        pub account: &'r str,
        pub quote_id: &'r str,
    }
    #[derive(FromForm)]
    pub struct Sep38InfoForm<'r> {
        pub slug: &'r str,
    }
}