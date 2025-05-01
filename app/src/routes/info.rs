use rocket::form::Form;
use rocket::{get, post, response::status, http::Status};
use rocket::serde::json::Json;
use controllers::{
    info::form::form::{QuoteRequestForm, PricesRequestForm, PriceRequestForm},
    api::api::{failure, success, ApiResponse},
};
use services::info::{AssetInfo, PriceAsset, QuoteResponse};
use models::info::PriceResponse;

// Info Routes
#[get("/info")]
pub async fn get_info() -> Result<status::Custom<Json<ApiResponse<Vec<AssetInfo>>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_info_controller()
        .await
        .map(|response| success("Info retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get info: {}", e), Status::BadRequest))
}

#[get("/prices", data = "<form>")]
pub async fn get_prices<'r>(
    form: Form<PricesRequestForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<Vec<PriceAsset>>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_prices_controller(form)
        .await
        .map(|response| success("Prices retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get prices: {}", e), Status::BadRequest))
}

#[get("/price", data = "<form>")]
pub async fn get_price<'r>(
    form: Form<PriceRequestForm<'r>>,
) -> Result<status::Custom<Json<ApiResponse<PriceResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_price_controller(form)
        .await
        .map(|response| success("Price retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get price: {}", e), Status::BadRequest))
}

#[post("/quote", data = "<form>")]
pub async fn create_quote<'r>(
    form: Form<QuoteRequestForm<'r>>,

) -> Result<status::Custom<Json<ApiResponse<QuoteResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::create_quote_controller(form)
        .await
        .map(|response| success("Quote created", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to create quote: {}", e), Status::BadRequest))
}

#[get("/quote/<id>")]
pub async fn get_quote(
    id: &str,
) -> Result<status::Custom<Json<ApiResponse<QuoteResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    // Add the missing second parameter
    controllers::info::get_quote_controller(id)
        .await
        .map(|response| success("Quote retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get quote: {}", e), Status::BadRequest))
}