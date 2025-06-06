pub mod routes {
    use controllers::api::api::{ApiResponse, failure, success};
    use controllers::sep38::form::form::{
        Sep38GetQuoteForm, Sep38InfoForm, Sep38PriceForm, Sep38QuoteForm,
    };
    use controllers::sep38::{
        create_sep38_quote, get_sep38_info, get_sep38_price, get_sep38_quote,
    };
    use rocket::{http::Status, post, response::status, serde::json::Json};
    use services::sep38::sep38::{AssetInfo, PriceResponse, QuoteResponse};

    #[post("/info", data = "<form>")]
    pub async fn get_sep38_info_route(
        form: Json<Sep38InfoForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<Vec<AssetInfo>>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let assets_info = get_sep38_info(form).await.map_err(|e| {
            eprintln!("Error getting SEP-38 info: {:?}", e);
            failure("Failed to get SEP-38 info", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-38 info retrieved successfully",
            assets_info,
            Status::Ok,
        ))
    }

    #[post("/price", data = "<form>")]
    pub async fn get_sep38_price_route<'r>(
        form: Json<Sep38PriceForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<PriceResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let price_response = get_sep38_price(form).await.map_err(|e| {
            eprintln!("Error getting SEP-38 price: {:?}", e);
            failure("Failed to get SEP-38 price", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-38 price retrieved successfully",
            price_response,
            Status::Ok,
        ))
    }

    #[post("/quote", data = "<form>")]
    pub async fn create_sep38_quote_route<'r>(
        form: Json<Sep38QuoteForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<QuoteResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let quote_response = create_sep38_quote(form).await.map_err(|e| {
            eprintln!("Error creating SEP-38 quote: {:?}", e);
            failure("Failed to create SEP-38 quote", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-38 quote created successfully",
            quote_response,
            Status::Created,
        ))
    }

    #[post("/get-qoute", data = "<form>")]
    pub async fn get_sep38_quote_route<'r>(
        form: Json<Sep38GetQuoteForm>,
    ) -> Result<
        status::Custom<Json<ApiResponse<QuoteResponse>>>,
        status::Custom<Json<ApiResponse<()>>>,
    > {
        let quote_response = get_sep38_quote(form).await.map_err(|e| {
            eprintln!("Error getting SEP-38 quote: {:?}", e);
            failure("Failed to get SEP-38 quote", Status::InternalServerError)
        })?;

        Ok(success(
            "SEP-38 quote retrieved successfully",
            quote_response,
            Status::Ok,
        ))
    }
}
