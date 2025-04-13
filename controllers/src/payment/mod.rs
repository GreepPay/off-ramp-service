// use rocket::form::Form;
// use stellar_base::{Asset, KeyPair, Memo};
// use services::payment::PaymentService;
// use form::form::PaymentForm;
// use serde_json::Value;
// pub mod form;

// pub mod form;


// pub async fn send_payment_controller(
//     data: Form<form::PaymentForm<'_>>,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let payment_service = PaymentService::new(
//         crate::config::HORIZON_URL.to_string(),
//         crate::config::NETWORK_PASSPHRASE.to_string(),
//     );
    
//     let keypair = KeyPair::from_secret(data.secret_key)?;
    
//     let asset = if let Some(issuer) = data.asset_issuer {
//         Asset::new_credit(data.asset_code, issuer)?
//     } else {
//         Asset::native()
//     };

//     let memo = match data.memo_type {
//         Some("id") => data.memo.as_ref().and_then(|v| v.as_str()).map(|s| Memo::Id(s.parse()?)),
//         Some("text") => data.memo.as_ref().and_then(|v| v.as_str()).map(Memo::Text),
//         Some("hash") => data.memo.as_ref().and_then(|v| v.as_str()).map(|s| Memo::Hash(hex::decode(s)?.try_into()?)),
//         _ => None,
//     };

//     let tx_hash = payment_service.send_payment(
//         &keypair,
//         data.destination,
//         &asset,
//         data.amount,
//         memo,
//     ).await?;

//     Ok(tx_hash)
// }