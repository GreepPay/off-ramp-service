#[macro_use]
extern crate rocket;
// use app::routes;

#[launch]
fn rocket() -> _ {
    // Load env
    dotenv::dotenv().ok();

    // Launch application
    rocket::build()
        // .mount(
        //     "/v1/users",
        //     routes![
        //         routes::auth::auth::get_users,
        //         routes::auth::auth::add_user,
        //         routes::auth::auth::update_user
        //     ],
        // )
        // .mount(
        //     "/v1/notifications",
        //     routes![routes::notification::notification::get_notifications],
        // )
    
        // .mount(
        //     "/v1/offramproutes",
        //     routes![
        //         routes::offramproutes::offramp_funds,
        //         routes::offramproutes::check_transaction_status,
        //         routes::offramproutes::get_transactions,
        //         routes::offramproutes::get_transaction,
        //         routes::offramproutes::get_asset_info,
        //     ],
        // )
    
}
