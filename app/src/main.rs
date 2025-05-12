#[macro_use]
extern crate rocket;
use app::routes::auth;
use app::routes::info;
use app::routes::kyc;
use app::routes::withdraw;

#[launch]
async fn rocket() -> _ {
    // Load env
    dotenv::dotenv().ok();

    rocket::build()
        .mount(
            "/v1/auth",
            routes![auth::get_challenge, auth::get_jwt_token,],
        )
        .mount(
            "/v1/info",
            routes![
                info::get_info,
                info::get_prices,
                info::get_price,
                info::create_quote,
                info::get_quote,
            ],
        )
        .mount(
            "/v1/kyc",
            routes![
                kyc::get_customer,
                kyc::put_customer,
                kyc::set_callback,
                kyc::submit_verification,
                kyc::delete_customer,
                kyc::upload_file,
                kyc::list_files,
            ],
        )
        .mount(
            "/v1/withdraw",
            routes![
                withdraw::withdraw,
                withdraw::withdraw_exchange,
                withdraw::get_transactions,
                withdraw::get_transaction,
            ],
        )
}
