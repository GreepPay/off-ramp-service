use form::form::{ChallengeRequestForm, TokenRequestForm};
use rocket::form::Form;
use services::sep10auth::StellarAuth;
pub mod form;


pub async fn get_challenge_controller(
    form: Form<ChallengeRequestForm<'_>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let auth = StellarAuth::global(); 
    auth.init().await.map_err(|e| Box::new(e))?;
    Ok(auth.get_challenge(form.account, form.client_domain).await?)
}


pub async fn get_jwt_token_controller(
    request: Form<TokenRequestForm<'_>>,
) -> Result<String, String> {
    let auth = StellarAuth::global(); 
    let token = auth.get_jwt_token(&request.transaction).await
        .map_err(|e| e.to_string())?;
    Ok(token)
}