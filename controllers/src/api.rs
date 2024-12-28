pub mod api {
    use rocket::http::Status;
    use rocket::response::status;
    use rocket::serde::{json::Json, Serialize};

    #[derive(Serialize)]
    pub struct ApiResponse<T> {
        pub success: bool,
        pub message: Option<String>,
        pub data: Option<T>,
        pub errors: Option<Vec<String>>,
    }

    impl<T> ApiResponse<T> {
        pub fn success(message: String, data: T) -> Json<ApiResponse<T>> {
            Json(ApiResponse {
                success: true,
                message: Some(message),
                data: Some(data),
                errors: None,
            })
        }

        pub fn failure(message: String) -> Json<ApiResponse<T>> {
            Json(ApiResponse {
                success: false,
                message: Some(message),
                data: None,
                errors: None,
            })
        }

        pub fn validation_fail(errors: Vec<String>) -> Json<ApiResponse<T>> {
            Json(ApiResponse {
                success: false,
                message: None,
                data: None,
                errors: Some(errors),
            })
        }
    }

    pub fn success<T: Serialize>(
        message: &str,
        data: T,
        status: Status,
    ) -> status::Custom<Json<ApiResponse<T>>> {
        status::Custom(status, ApiResponse::success(message.to_string(), data))
    }

    pub fn failure(message: &str, status: Status) -> status::Custom<Json<ApiResponse<()>>> {
        status::Custom(status, ApiResponse::<()>::failure(message.to_string()))
    }

    pub fn validation_fail(
        errors: Vec<String>,
        status: Status,
    ) -> status::Custom<Json<ApiResponse<()>>> {
        status::Custom(status, ApiResponse::<()>::validation_fail(errors))
    }
}
