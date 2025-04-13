use std::str::FromStr;
use anyhow::Error;
use reqwest::Response;
use stellar_base::{
    amount::Stroops, asset::CreditAsset, operations::{ChangeTrustOperationBuilder, PaymentOperationBuilder}, time_bounds::TimeBounds, transaction::TransactionBuilder, xdr::XDRSerialize, Asset, KeyPair, Network, Transaction
};
use stellar_sdk::{Keypair, Server};



/// A struct that handles the creation and issuance of custom assets on the Stellar blockchain
///
/// This struct manages the relationship between an asset issuer and receiver, including
/// creating trustlines and issuing assets.
pub struct AssetIssuer {
    client: Server,
    issuer_keypair: Option<Keypair>,
    receiver_keypair: Option<Keypair>,
    asset: Option<CreditAsset>,
    issuer_secret: String,
    receiver_secret: String,
    server_url: String,
}

impl AssetIssuer {
    /// Creates a new AssetIssuer instance
    ///
    /// # Arguments
    /// * `server_url` - The URL of the Stellar server to connect to
    /// * `issuer_secret` - The secret key of the asset issuer
    /// * `receiver_secret` - The secret key of the asset receiver
    /// * `asset_code` - The code/name of the asset to be created
    pub fn new(
        server_url: String,
        issuer_secret: String,
        receiver_secret: String,
        asset_code: String,
    ) -> Self {
        let mut asset_issuer = Self {
            client: Server::new(server_url.clone(), None).unwrap(),
            issuer_keypair: None,
            receiver_keypair: None,
            asset: None,
            issuer_secret: issuer_secret.clone(),
            receiver_secret: receiver_secret.clone(),
            server_url: server_url,
        };

        asset_issuer
            .set_keypair(issuer_secret, receiver_secret)
            .unwrap();
        asset_issuer.define_asset(asset_code).unwrap();
        asset_issuer
    }


    /// Sets the keypairs for both the issuer and receiver
    ///
    /// # Arguments
    /// * `issuer_secret` - The secret key of the asset issuer
    /// * `receiver_secret` - The secret key of the asset receiver
    ///
    /// # Returns
    /// * `Result<(), Error>` - Ok if keypairs are set successfully, Error otherwise
    fn set_keypair(&mut self, issuer_secret: String, receiver_secret: String) -> Result<(), Error> {
        let issuer_keypair = Keypair::from_secret_key(&issuer_secret)?;
        let receiver_keypair = Keypair::from_secret_key(&receiver_secret)?;

        self.issuer_keypair = Some(issuer_keypair);
        self.receiver_keypair = Some(receiver_keypair);

        Ok(())
    }

    /// Defines a new custom asset with the given code
    ///
    /// # Arguments
    /// * `asset_code` - The code/name of the asset to be created
    ///
    /// # Returns
    /// * `Result<(), Error>` - Ok if asset is defined successfully, Error otherwise
    fn define_asset(&mut self, asset_code: String) -> Result<(), Error> {
        let issuer_key = stellar_base::PublicKey::from_account_id(
            self.issuer_keypair.as_ref().unwrap().public_key().as_str(),
        )?;
        let new_asset = CreditAsset::new(asset_code, issuer_key).unwrap();
        self.asset = Some(new_asset);
        Ok(())
    }

    /// Creates a trustline between the issuer and receiver for the defined asset
    ///
    /// This operation allows the receiver to hold the custom asset by establishing
    /// trust with the issuer.
    ///
    /// # Returns
    /// * `Result<(), Error>` - Ok if trustline is created successfully, Error otherwise
    pub async fn create_trustline(&self) -> Result<Response, Error> {

        let receiver_account = stellar_base::PublicKey::from_account_id(
            self.receiver_keypair
                .as_ref()
                .unwrap()
                .public_key()
                .as_str(),
        )?;
    
        let time_bounds = TimeBounds::always_valid();

        let trust_operation_builder = ChangeTrustOperationBuilder::new();

        let trust_operation = trust_operation_builder
            .with_source_account(receiver_account.clone())
            .with_asset(Asset::Credit(self.asset.as_ref().unwrap().clone()))
            .with_limit(Some(Stroops::new(10000000)))?
            .build()?;

        // Fetch the current account details to get the sequence number
        let receiver_account_details = self.client
            .load_account(&self.receiver_keypair.as_ref().unwrap().public_key())?;

        let mut trust_transaction = Transaction::builder(
                receiver_account, 
                receiver_account_details.sequence_number().parse::<i64>()? + 1, // Convert to i64
                Stroops::new(100)
            )
            .add_operation(trust_operation)
            .with_time_bounds(time_bounds)
            .into_transaction()?;

        let receiver_key = KeyPair::from_str(&self.receiver_secret)?;
        trust_transaction.sign(&receiver_key, &Network::new_test())?;

        let base64_transaction = trust_transaction.into_envelope().xdr_base64()?;

        // Submit the transaction to Horizon testnet using reqwest
        let client = reqwest::Client::new();
        let response = client
            .post(self.server_url.clone() + "/transactions")
            .header("Accept", "application/json")
            .form(&[("tx", base64_transaction)])
            .send()
            .await?;

        Ok(response)
    }

    /// Issues the defined asset from the issuer to the receiver
    ///
    /// This operation transfers the custom asset from the issuer's account
    /// to the receiver's account.
    ///
    /// # Returns
    /// * `Result<(), Error>` - Ok if asset is issued successfully, Error otherwise
    pub async fn issue_asset(&self) -> Result<Response, Error> {
        let issuer_account = stellar_base::PublicKey::from_account_id(
            self.issuer_keypair.as_ref().unwrap().public_key().as_str(),
        )?;

        let receiver_account = stellar_base::PublicKey::from_account_id(
            self.receiver_keypair.as_ref().unwrap().public_key().as_str(),
        )?;

        let issuer_account_details = self.client
            .load_account(&self.issuer_keypair.as_ref().unwrap().public_key())?;

        let payment_transaction = TransactionBuilder::new(issuer_account,issuer_account_details.sequence_number().parse::<i64>()? + 1, Stroops::new(100));

        let source_account = stellar_base::PublicKey::from_account_id(
            self.issuer_keypair.as_ref().unwrap().public_key().as_str(),
        )?;

        let payment_operation_builder = PaymentOperationBuilder::new();

        let payment_operation = payment_operation_builder
            .with_source_account(source_account)
            .with_asset(Asset::Credit(self.asset.as_ref().unwrap().clone()))
            .with_amount(Stroops::new(10))?
            .with_destination(receiver_account)
            .build()?;

        let mut transaction = payment_transaction
            .add_operation(payment_operation)
            .into_transaction()
            .unwrap();

        let issuer_key = KeyPair::from_str(&self.issuer_secret)?;
        transaction.sign(&issuer_key, &Network::new_test())?;

        let base64_transaction = transaction.into_envelope().xdr_base64()?;

         // Submit the transaction to Horizon testnet using reqwest
         let client = reqwest::Client::new();
         let response = client
             .post(self.server_url.clone() + "/transactions")
             .header("Accept", "application/json")
             .form(&[("tx", base64_transaction)])
             .send()
             .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    // Helper function to generate test keypairs
    fn generate_test_keypairs() -> (String, String) {
        let issuer = Keypair::random();
        let receiver = Keypair::random();
        (
            issuer.unwrap().secret_key().unwrap().to_string(),
            receiver.unwrap().secret_key().unwrap().to_string(),
        )
    }

    #[tokio::test]
    async fn test_new_asset_issuer() {
        let (issuer_secret, receiver_secret) = generate_test_keypairs();
        let server_url = "https://horizon-testnet.stellar.org".to_string();
        
        let asset_issuer = AssetIssuer::new(
            server_url.clone(),
            issuer_secret.clone(),
            receiver_secret.clone(),
            "TEST".to_string(),
        );

        assert!(asset_issuer.issuer_keypair.is_some());
        assert!(asset_issuer.receiver_keypair.is_some());
        assert!(asset_issuer.asset.is_some());
        assert_eq!(asset_issuer.server_url, server_url);
    }

    #[tokio::test]
    async fn test_create_trustline() {
        let (issuer_secret, receiver_secret) = generate_test_keypairs();
        let server_url = "https://horizon-testnet.stellar.org".to_string();
        
        let asset_issuer = AssetIssuer::new(
            server_url,
            issuer_secret,
            receiver_secret,
            "TEST".to_string(),
        );

        let result = asset_issuer.create_trustline().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_issue_asset() {
        let (issuer_secret, receiver_secret) = generate_test_keypairs();
        let server_url = "https://horizon-testnet.stellar.org".to_string();
        
        let asset_issuer = AssetIssuer::new(
            server_url,
            issuer_secret,
            receiver_secret,
            "TEST".to_string(),
        );

        // First create trustline
        let trustline_result = asset_issuer.create_trustline().await;
        assert!(trustline_result.is_ok());

        // Then issue asset
        let issue_result = asset_issuer.issue_asset().await;
        assert!(issue_result.is_ok());
    }
}
