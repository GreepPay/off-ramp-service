use stellar_base::KeyPair;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyPairError {
    #[error("Generation failed")]
    GenerationFailed,

    #[error("Invalid key")]
    InvalidKey,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid format")]
    InvalidFormat,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn generate_keypair(account_secret: &str) -> Result<KeyPair, KeyPairError> {
    KeyPair::from_secret_seed(account_secret).map_err(|_| KeyPairError::GenerationFailed)
}
