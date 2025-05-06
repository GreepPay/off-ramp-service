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
                routes::sep38::sep38::get_sep38_info_route,
                routes::sep38::sep38::get_sep38_price_route,
                routes::sep38::sep38::create_sep38_quote_route,
                routes::sep38::sep38::get_sep38_quote_route,
            ],
        )
        .mount(
            "/v1/withdrawl",
            routes![ routes::sep6::sep6::withdraw,
                routes::sep6::sep6::withdraw_exchange,
                routes::sep6::sep6::transactions,
                routes::sep6::sep6::transaction,],
        )
    
        .mount(
            "/v1/kyc",
            routes![ 
                routes::sep6::sep6::get_kyc_status,
                routes::sep6::sep6::create_kyc,
                routes::sep6::sep6::update_kyc,
                routes::sep6::sep6::delete_customer,],
        )
    
}
