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
            "/v1/users",
            routes![
                routes::auth::auth::get_users,
                routes::auth::auth::add_user,
                routes::auth::auth::update_user
            ],
        )
        .mount(
            "/v1/notifications",
            routes![routes::notification::notification::get_notifications],
        )
}
