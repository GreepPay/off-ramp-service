use rocket::{form::Form, http::Status, post, response::status, serde::json::Json};
use controllers::{
    offramp::form::form::OfframpForm,
    api::api::{failure, success, ApiResponse},
};



#[post("/offramp", data = "<form>")]
pub async fn offramp_funds<'r>(
    form: Form<OfframpForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<String>>>, status::Custom<Json<ApiResponse<()>>>> {
    let transaction_id = controllers::offramp::offramp_funds_controller(form)
        .await
        .map_err(|e| {
            failure(
                &format!("Failed to initiate offramp: {}", e),
                Status::InternalServerError,
            )
        })?;

    Ok(success(
        "Offramp initiated successfully",
        transaction_id,
        Status::Ok,
    ))
}

