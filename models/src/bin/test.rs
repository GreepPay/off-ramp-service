use diesel::prelude::*;
use models::offramp::{
    OfframpTransaction, NewOfframpTransaction,
    OfframpQuote, NewOfframpQuote
};
use models::common::establish_connection;
use models::schema::offramp_service::{offramp_transactions, offramp_quotes};
use bigdecimal::BigDecimal;
use chrono::Utc;
use uuid::Uuid;

fn fetch_offramp_transactions() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection()?;

    let results = offramp_transactions::table
        .filter(offramp_transactions::id.is_not_null())
        .load::<OfframpTransaction>(&mut connection)?;

    println!("Displaying {} offramp transactions", results.len());
    for tx in results {
        println!("Transaction ID: {}", tx.transaction_id);
        println!("Amount: {}", tx.amount);
        println!("Status: {}", tx.status);
        println!("Created: {}", tx.created_at);
        println!("-----------");
    }

    Ok(())
}

fn create_test_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection()?;

    let new_tx = NewOfframpTransaction {
        account_id: Uuid::new_v4(),
        transaction_id: "test_tx_123",
        amount: BigDecimal::from(100),
        dest_currency: "USD",
        status: "pending",
    };

    diesel::insert_into(offramp_transactions::table)
        .values(&new_tx)
        .execute(&mut connection)?;

    println!("Successfully created test transaction");
    Ok(())
}

fn fetch_offramp_quotes() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection()?;

    let results = offramp_quotes::table
        .filter(offramp_quotes::id.is_not_null())
        .load::<OfframpQuote>(&mut connection)?;

    println!("Displaying {} offramp quotes", results.len());
    for quote in results {
        println!("Quote ID: {}", quote.quote_id);
        println!("Sell Asset: {}", quote.sell_asset);
        println!("Buy Asset: {}", quote.buy_asset);
        println!("Price: {}", quote.price);
        println!("-----------");
    }

    Ok(())
}

fn create_test_quote() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = establish_connection()?;

    // First create a transaction
    let new_tx = NewOfframpTransaction {
        account_id: Uuid::new_v4(),
        transaction_id: "test_tx_for_quote_123",
        amount: BigDecimal::from(100),
        dest_currency: "USD",
        status: "pending",
    };

    // Insert and get the full transaction record
    let tx = diesel::insert_into(offramp_transactions::table)
        .values(&new_tx)
        .get_result::<OfframpTransaction>(&mut connection)?;

    // Now create the quote with the valid transaction_id
    let new_quote = NewOfframpQuote {
        transaction_id: tx.id,  // Use the actual ID from the transaction
        quote_id: "test_quote_456",
        sell_asset: "USDC",
        buy_asset: "USD",
        sell_amount: BigDecimal::from(100),
        buy_amount: BigDecimal::from(99),
        price: "0.99".parse::<BigDecimal>().unwrap(),
        expires_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(offramp_quotes::table)
        .values(&new_quote)
        .execute(&mut connection)?;

    println!("Successfully created test quote with valid transaction reference");
    Ok(())
}

fn main() {
    // Test transactions
    if let Err(e) = fetch_offramp_transactions() {
        eprintln!("Error fetching transactions: {}", e);
    }

    if let Err(e) = create_test_transaction() {
        eprintln!("Error creating test transaction: {}", e);
    }

    // Test quotes
    if let Err(e) = fetch_offramp_quotes() {
        eprintln!("Error fetching quotes: {}", e);
    }

    if let Err(e) = create_test_quote() {
        eprintln!("Error creating test quote: {}", e);
    }
}