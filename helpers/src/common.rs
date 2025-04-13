use openssl::symm::{decrypt, encrypt, Cipher};
use rand::rngs::OsRng;
use rand::RngCore;

/// Generates a random encryption key and initialization vector (IV) for AES-256-CBC encryption.
///
/// The function generates:
/// - A 32-byte (256-bit) random key suitable for AES-256
/// - A 16-byte (128-bit) random initialization vector
///
/// Prints both values to stdout for debugging/demonstration purposes.
pub fn generate_encryption_key_and_iv() {
    let mut key = [0u8; 32]; // 32 bytes for AES-256
    let mut iv = [0u8; 16]; // 16 bytes for AES block size
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);

    println!("Encryption Key: {:?}", hex::encode(key));
    println!("Initialization Vector (IV): {:?}", hex::encode(iv))
}

/// Encrypts a private key using AES-256-CBC encryption.
///
/// # Arguments
/// - `key`: The private key data to encrypt as a byte slice.
///
/// # Returns
/// - `Ok(Vec<u8>)`: The encrypted private key data.
/// - `Err(openssl::error::ErrorStack)`: If encryption fails.
///
/// # Note
/// Currently uses hardcoded encryption key and IV - should be updated for production use.
pub fn encrypt_private_key(key: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = Cipher::aes_256_cbc();
    let encryption_key = hex::decode(std::env::var("ENCRYPTION_KEY").unwrap()).unwrap();
    let iv = hex::decode(std::env::var("ENCRYPTION_IV").unwrap()).unwrap();

    encrypt(cipher, &encryption_key, Some(&iv), key)
}

/// Decrypts an encrypted private key using AES-256-CBC.
///
/// # Arguments
/// - `encrypted_data`: The encrypted private key as a byte slice.
/// - `key`: The 256-bit encryption key (32 bytes).
/// - `iv`: The initialization vector (16 bytes).
///
/// # Returns
/// - `Ok(Vec<u8>)`: The decrypted private key.
/// - `Err(openssl::error::ErrorStack)`: If decryption fails.
pub fn decrypt_private_key(encrypted_data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = Cipher::aes_256_cbc();
    let encryption_key = hex::decode(std::env::var("ENCRYPTION_KEY").unwrap()).unwrap();
    let iv = hex::decode(std::env::var("ENCRYPTION_IV").unwrap()).unwrap();
    decrypt(cipher, &encryption_key, Some(&iv), encrypted_data)
}
