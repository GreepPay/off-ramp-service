use models::notification::models::notification::notification::Notification;

pub mod form;

pub async fn get_many_notifications() -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
    Ok(services::notification::notification::get_many_notifications().await?)
}
