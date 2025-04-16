pub mod form;
use form::form::OfframpForm;
use stellar_sdk::Keypair;
use rocket::form::Form;
use services::offramp::offramp::offramp_funds;




// Offramp funds
pub async fn offramp_funds_controller(
    form: Form<OfframpForm<'_>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let form_data = form.into_inner();
    let keypair = Keypair::from_secret_key(&form_data.secret_key)?;
    Ok(
        offramp_funds(
            form_data.anchor_domain,
            form_data.account_id,
            &keypair,
            form_data.status,
            form_data.amount,
            form_data.dest_currency,
            Some(serde_json::from_str(&form_data.kyc_fields.unwrap_or_default())?),
        )
        .await?,
    )
}

