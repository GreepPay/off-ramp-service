use std::env;

use diesel::ConnectionError;
use diesel_async::{AsyncConnection, AsyncMysqlConnection};

pub async fn establish_connection(db_url: &str) -> Result<AsyncMysqlConnection, ConnectionError> {

    dotenv::dotenv().ok();

    match AsyncMysqlConnection::establish(&env::var(&db_url).unwrap()).await {
        Ok(connection) => Ok(connection),
        Err(error) => {
            eprintln!("Error establishing connection: {}", error);
            Err(error)
        }
    }
}