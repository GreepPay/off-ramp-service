// use rocket::{form::Form, get, http::Status, post, response::status, serde::json::Json};
// use controllers::{
//     api::api::{failure, success, ApiResponse},
//     payment,
// };
// use services::paymentService::PaymentService;

// #[post("/payment/send", data = "<form>")]
// pub async fn send_payment(
//     form: Form<payment::form::PaymentForm<'_>>,
// ) -> Result<status::Custom<Json<ApiResponse<String>>>, status::Custom<Json<ApiResponse<()>>>> {
//     let tx_hash = payment::send_payment_controller(form)
//         .await
//         .map_err(|e| {
//             failure(
//                 format!("Failed to send payment: {}", e),
//                 Status::InternalServerError,
//             )
//         })?;

//     Ok(success(
//         "Payment sent successfully",
//         tx_hash,
//         Status::Ok,
//     ))
// }
