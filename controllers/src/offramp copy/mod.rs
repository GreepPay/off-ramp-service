pub mod form;
use form::form::OfframpForm;
use services::offramp::offramp::OfframpService;
use stellar_sdk::Keypair;
use stellar_base::Network;
use rocket::form::Form;

// Offramp funds controller
pub async fn offramp_funds_controller(
    form: Form<OfframpForm<'_>>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let form_data = form.into_inner();
    
    let offramp_service = OfframpService::new(
        Network::new_public(),
        "your_jwt_secret".to_string(),
        "stellar:USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN".to_string(),
    );
    
    let keypair = Keypair::from_secret_key(&form_data.secret_key)?;
    
    offramp_service.offramp_funds(
        form_data.anchor_domain,
        form_data.account_id,
        &keypair,
        form_data.amount,
        form_data.dest_currency,
        Some(serde_json::from_str(&form_data.kyc_fields.unwrap_or_default())?),
    )
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync + 'static>)
}