
pub mod notification {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use models::notification::{models::notification::notification::Notification, schema::notifications};
    /// Let create CRUD Operations
    
    // Get many notifications
    pub async fn get_many_notifications() -> Result<Vec<Notification>, Box<dyn std::error::Error>>  {
        let mut connection = models::notification::establish_connection().await?;

        let results: Vec<Notification> = notifications::table
        .filter(notifications::id.gt(0))
        .select(Notification::as_select())
        .load(&mut connection)
        .await?;
        
        Ok(results)
    }
}