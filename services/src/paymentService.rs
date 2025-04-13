// use stellar_base::{KeyPair, Memo, Asset};
// use stellar_horizon::client::HorizonClient;
// use anyhow::{Result, anyhow};
// use bigdecimal::BigDecimal;



// pub struct PaymentService {
//     horizon_url: String,
//     network_passphrase: String,
// }

// impl PaymentService {
//     pub fn new(horizon_url: String, network_passphrase: String) -> Self {
//         Self {
//             horizon_url,
//             network_passphrase,
//         }
//     }

//     pub async fn send_payment(
//         &self,
//         source_keypair: &KeyPair,
//         destination: &str,
//         asset: &Asset,
//         amount: &str,
//         memo: Option<Memo>,
//     ) -> Result<String> {
//         let client = HorizonClient::new(&self.horizon_url);
        
//         let source_account = client.load_account(&source_keypair.public_key()).await?;
        
//         let operation = stellar_base::Operation::new_payment()
//             .to(destination)
//             .with_asset(asset.clone())
//             .with_amount(amount)
//             .build();
            
//         let mut tx_builder = stellar_base::TransactionBuilder::new(
//             source_account,
//             stellar_base::TransactionBuilderOptions::default()
//                 .with_time_bounds(Some(300))
//             .add_operation(operation);
            
//         if let Some(m) = memo {
//             tx_builder = tx_builder.with_memo(m);
//         }
        
//         let tx = tx_builder.build()?;
//         let signed_tx = tx.sign(&source_keypair, &self.network_passphrase)?;
        
//         let response = client.submit_transaction(&signed_tx).await?;
//         Ok(response.hash)
//     }
// }