use ed25519_dalek::{SigningKey, VerifyingKey, PUBLIC_KEY_LENGTH};
use k256::ecdsa::{SigningKey as K256SigningKey, VerifyingKey as K256VerifyingKey};
use thiserror::Error;
use k256::elliptic_curve::rand_core::OsRng;
use k256::elliptic_curve::rand_core::RngCore;
use sha3::Keccak256;
use ripemd::Ripemd160;
use sha2::Sha256;
use sha3::digest::Digest;
use base58::ToBase58;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Failed to generate keypair: {0}")]
    KeypairGeneration(String),
    #[error("Failed to derive address: {0}")]
    AddressDerivation(String),
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
}

pub type Result<T> = std::result::Result<T, WalletError>;

/// Represents a wallet keypair with its associated address
#[derive(Debug, Clone)]
pub struct Wallet {
    pub address: String,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Trait for wallet operations
pub trait WalletOperations {
    fn generate_keypair() -> Result<Wallet>;
    fn derive_address(public_key: &[u8]) -> Result<String>;
}

/// BSC wallet implementation using secp256k1
pub struct BscWallet;

impl WalletOperations for BscWallet {
    fn generate_keypair() -> Result<Wallet> {
        let signing_key = K256SigningKey::random(&mut OsRng);
        let verifying_key = K256VerifyingKey::from(&signing_key);
        
        let private_key = signing_key.to_bytes().to_vec();
        let public_key = verifying_key.to_encoded_point(false).as_bytes().to_vec();
        
        let address = Self::derive_address(&public_key)?;
        
        Ok(Wallet {
            address,
            private_key,
            public_key,
        })
    }

    fn derive_address(public_key: &[u8]) -> Result<String> {
        if public_key.len() != 65 {
            return Err(WalletError::InvalidKeyFormat("BSC public key must be 65 bytes (uncompressed)".to_string()));
        }
        // Remove the first byte (0x04) which is the prefix for uncompressed public key
        let public_key = &public_key[1..];
        
        // Calculate Keccak-256 hash of the public key
        let mut hasher = Keccak256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        
        // Take the last 20 bytes of the hash
        let address_bytes = &hash[hash.len() - 20..];
        
        // Add 0x prefix and convert to hex string
        Ok(format!("0x{}", hex::encode(address_bytes)))
    }
}

/// Pactus wallet implementation using ed25519
pub struct PactusWallet;

impl WalletOperations for PactusWallet {
    fn generate_keypair() -> Result<Wallet> {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let signing_key = SigningKey::from_bytes(&bytes);
        let verifying_key = VerifyingKey::from(&signing_key);
        let private_key = signing_key.to_bytes().to_vec();
        let public_key = verifying_key.to_bytes().to_vec();
        let address = Self::derive_address(&public_key)?;
        Ok(Wallet {
            address,
            private_key,
            public_key,
        })
    }

    fn derive_address(public_key: &[u8]) -> Result<String> {
        if public_key.len() != PUBLIC_KEY_LENGTH {
            return Err(WalletError::InvalidKeyFormat("Pactus public key must be 32 bytes".to_string()));
        }
        // Calculate RIPEMD160 hash of the public key
        let mut hasher = Ripemd160::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        
        // Add version byte (0x01 for Pactus)
        let mut address_bytes = vec![0x01];
        address_bytes.extend_from_slice(&hash);
        
        // Calculate checksum (first 4 bytes of double SHA256)
        let mut hasher = Sha256::new();
        hasher.update(&address_bytes);
        let hash = hasher.finalize();
        let mut hasher = Sha256::new();
        hasher.update(&hash);
        let hash = hasher.finalize();
        let checksum = &hash[..4];
        
        // Combine address bytes and checksum
        address_bytes.extend_from_slice(checksum);
        
        // Encode to Base58
        Ok(address_bytes.to_base58())
    }
} 