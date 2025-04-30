use rocket::{form::Form, http::Status, post, response::status, serde::json::Json, get, State};
use std::sync::Arc;
use controllers::{
    auth::form::form::{ChallengeRequestForm,TokenRequestForm},
    api::api::{failure, success, ApiResponse},
};
use services::sep10auth::StellarAuth;



#[get("/auth", data = "<form>")]
pub async fn get_challenge<'r>(
    form: Form<ChallengeRequestForm<'r>>,
    auth: &State<Arc<StellarAuth>>,
) -> Result<status::Custom<Json<ApiResponse<String>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::auth::get_challenge_controller(form, auth)
        .await
        .map(|response| success("Challenge generated", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to generate challenge: {}", e), Status::BadRequest))
}

#[post("/auth/token", data = "<form>")]
pub async fn get_jwt_token<'r>(
    form: Form<TokenRequestForm<'r>>,
    auth: &State<Arc<StellarAuth>>,
) -> Result<status::Custom<Json<ApiResponse<String>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::auth::get_jwt_token_controller(form, auth)
        .await
        .map(|response| success("Token generated", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to generate token: {}", e), Status::BadRequest))
}
