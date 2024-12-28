pub mod notification {
    use controllers::api::api::{failure, success, ApiResponse};
    use models::notification::models::notification::notification::Notification;
    use rocket::{get, http::Status, response::status, serde::json::Json};

    #[get("/")]
    pub async fn get_notifications() -> Result<
        status::Custom<Json<ApiResponse<Vec<Notification>>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let results = controllers::notification::get_many_notifications()
            .await
            .map_err(|_| {
                failure(
                    "Failed to get many notifications",
                    Status::InternalServerError,
                )
            })?;

        Ok(success(
            "Notifications fetched successfully",
            results,
            Status::Ok,
        ))
    }
}
