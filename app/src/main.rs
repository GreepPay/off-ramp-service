#[macro_use]
extern crate rocket;
use app::routes;

#[launch]
fn rocket() -> _ {
    // Load env
    dotenv::dotenv().ok();

    // Launch application
    rocket::build()
        .mount(
            "/v1/exchange",
            routes![
                routes::sep38::routes::get_sep38_info_route,
                routes::sep38::routes::get_sep38_price_route,
                routes::sep38::routes::create_sep38_quote_route,
                routes::sep38::routes::get_sep38_quote_route,
            ],
        )
        .mount(
            "/v1/withdrawl",
            routes![
                routes::sep6::sep6::withdraw,
                routes::sep6::sep6::withdraw_exchange,
                routes::sep6::sep6::anchorinfo,
                routes::sep6::sep6::transactions,
                routes::sep6::sep6::transaction,
            ],
        )
        .mount(
            "/v1/kyc",
            routes![
                routes::sep12::sep12::get_kyc_status,
                routes::sep12::sep12::create_kyc,
                routes::sep12::sep12::update_kyc,
                routes::sep12::sep12::delete_customer,
            ],
        )
    
        .mount(
            "/v1/crossborderpayment",
            routes![
                routes::sep31::routes::get_sep31_info_route,
                routes::sep31::routes::create_sep31_transaction_route,
                routes::sep31::routes::get_sep31_transaction_route,
                routes::sep31::routes::update_sep31_transaction_route,
                routes::sep31::routes::set_sep31_transaction_callback_route,
            ],
        )
    
        .mount(
            "/v1/sep24payment",
            routes![
                routes::sep24::routes::get_sep24_info_route,
                routes::sep24::routes::interactive_sep24_withdraw_route,
                routes::sep24::routes::get_sep24_transactions_route,
                routes::sep24::routes::get_sep24_transaction_route,
            ],
        )
}
