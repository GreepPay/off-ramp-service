use diesel::ConnectionError;
use diesel_async::AsyncMysqlConnection;

pub mod models;
pub mod schema;

pub async fn establish_connection() -> Result<AsyncMysqlConnection, ConnectionError> {
    crate::common::establish_connection("AUTH_DATABASE_URL").await
}