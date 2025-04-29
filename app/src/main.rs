#[macro_use]
extern crate rocket;
use app::routes::offramproutes;


#[launch]
async fn rocket() -> _ {
    // Load env
    dotenv::dotenv().ok();

 
    rocket::build()
        .mount(
            "/v1/offramproutes",
            routes![
                offramproutes::offramp_funds,
            ],
        )
}