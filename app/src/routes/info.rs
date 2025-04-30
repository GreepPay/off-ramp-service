use rocket::{form::Form, http::Status, post, response::status, serde::json::Json};
use controllers::{
    info::form::form::{QuoteRequestForm,PricesRequestForm,PriceRequestForm,},
    api::api::{failure, success, ApiResponse},
};



// Info Routes
#[get("/info")]
pub async fn get_info(
    client: &State<Arc<Sep38Client>>,
) -> Result<status::Custom<Json<ApiResponse<Vec<AssetInfo>>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_info_controller(client)
        .await
        .map(|response| success("Info retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get info: {}", e), Status::BadRequest))
}

#[get("/prices", data = "<form>")]
pub async fn get_prices<'r>(
    form: Form<PricesRequestForm<'r>>,
    client: &State<Arc<Sep38Client>>,
) -> Result<status::Custom<Json<ApiResponse<Vec<PriceAsset>>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_prices_controller(form, client)
        .await
        .map(|response| success("Prices retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get prices: {}", e), Status::BadRequest))
}

#[get("/price", data = "<form>")]
pub async fn get_price<'r>(
    form: Form<PriceRequestForm<'r>>,
    client: &State<Arc<Sep38Client>>,
) -> Result<status::Custom<Json<ApiResponse<PriceResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_price_controller(form, client)
        .await
        .map(|response| success("Price retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get price: {}", e), Status::BadRequest))
}

#[post("/quote", data = "<form>")]
pub async fn create_quote<'r>(
    form: Form<QuoteRequestForm<'r>>,
    client: &State<Arc<Sep38Client>>,
    auth_token: &str,
    transaction_id: Option<Uuid>,
) -> Result<status::Custom<Json<ApiResponse<QuoteResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::create_quote_controller(form, client, auth_token, transaction_id)
        .await
        .map(|response| success("Quote created", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to create quote: {}", e), Status::BadRequest))
}

#[get("/quote/<id>")]
pub async fn get_quote(
    id: &str,
    client: &State<Arc<Sep38Client>>,
    auth_token: &str,
) -> Result<status::Custom<Json<ApiResponse<QuoteResponse>>>, status::Custom<Json<ApiResponse<()>>>> {
    controllers::info::get_quote_controller(id, client, auth_token)
        .await
        .map(|response| success("Quote retrieved", response, Status::Ok))
        .map_err(|e| failure(&format!("Failed to get quote: {}", e), Status::BadRequest))
}

