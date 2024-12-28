use diesel::prelude::*;
use models::{auth::{models::user::user::User, schema::users}, notification::{models::notification::notification::Notification, schema::notifications}};
use rocket::tokio;
use diesel_async::RunQueryDsl;


async fn fetch_users() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = models::auth::establish_connection().await?;

    let results: Vec<User> = users::table
    .filter(users::id.gt(0))
    .select(User::as_select())
    .load(&mut connection)
    .await?;

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("-----------\n");
        println!("{:#?}", user.first_name.unwrap_or_else(|| "No name".to_string()));
    }

    Ok(())
}


async fn fetch_notification() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = models::notification::establish_connection().await?;

    let results: Vec<Notification> = notifications::table
    .filter(notifications::id.gt(0))
    .select(Notification::as_select())
    .load(&mut connection)
    .await?;

    println!("Displaying {} notifications", results.len());
    for notification in results {
        println!("{}", notification.id);
        println!("-----------\n");
        println!("{:#?}", notification.body);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = fetch_users().await {
        eprintln!("Error fetching users: {}", e);
    }

    if let Err(e) = fetch_notification().await {
        eprintln!("Error fetching notification: {}", e);
    }
}
