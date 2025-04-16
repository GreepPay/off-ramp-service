#[macro_use]
extern crate rocket;
use app::routes::offramproutes;
// use helpers::asset_issuer::AssetIssuer;
// use helpers::{asset_issuer::AssetIssuer, stellar_chain::StellarChain};
// use stellar_base::Network;
// use stellar_sdk::Keypair;

#[launch]
async fn rocket() -> _ {
    // Load env
    dotenv::dotenv().ok();

    // Initialize asset issuer Do this only once
    // let asset_issuer = AssetIssuer::new(
    //     "https://horizon-testnet.stellar.org".to_string(),
    //     std::env::var("ISSUER_SECRET_KEY").unwrap(),
    //     std::env::var("RECEIVER_SECRET_KEY").unwrap(),
    //     "GRP".to_string(),
    // );

    // Create trustline
    // asset_issuer.create_trustline().await.unwrap();

    // Issue asset
    // asset_issuer.issue_asset().await.unwrap();

    // Mint asset
    // asset_issuer
    //     .mint_asset(String::from("10000"))
    //     .await
    //     .unwrap();

    // Create and activate account
    // let stellar_chain = StellarChain::new("https://horizon-testnet.stellar.org".to_string(), Network::new_test());

    // let account = stellar_chain.create_new_account().unwrap();

    // let account_keypair = Keypair::from_secret_key(account.secret_key.as_str()).unwrap();

    // stellar_chain.activate_account(account_keypair).await.unwrap();

    // Generate encryption key and iv. Use when generating a new key.
    // helpers::common::generate_encryption_key_and_iv();

    // Launch application
    rocket::build()
        .mount(
            "/v1/offramproutes",
            routes![
                offramproutes::offramp_funds,
            ],
        )
}