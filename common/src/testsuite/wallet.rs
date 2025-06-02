#[allow(unused_imports)]
use crate::wallet::{BscWallet, PactusWallet};

#[test]
fn test_bsc_wallet_generation() {
    let wallet = BscWallet::generate_keypair().unwrap();
    
    // Verify BSC address format (should start with 0x and be 42 characters long)
    assert!(wallet.address.starts_with("0x"));
    assert_eq!(wallet.address.len(), 42);
    
    // Verify private key length (32 bytes)
    assert_eq!(wallet.private_key.len(), 32);
    
    // Verify public key length (65 bytes for uncompressed)
    assert_eq!(wallet.public_key.len(), 65);
}

#[test]
fn test_pactus_wallet_generation() {
    let wallet = PactusWallet::generate_keypair().unwrap();
    
    // Verify Pactus address format (should be a valid base58 string)
    assert!(!wallet.address.is_empty());
    
    // Verify private key length (32 bytes)
    assert_eq!(wallet.private_key.len(), 32);
    
    // Verify public key length (32 bytes)
    assert_eq!(wallet.public_key.len(), 32);
}

#[test]
fn test_multiple_wallet_generation() {
    // Generate multiple wallets and ensure they're all unique
    let wallets: Vec<_> = (0..5)
        .map(|_| BscWallet::generate_keypair().unwrap())
        .collect();
    
    // Check that all addresses are unique
    let addresses: Vec<_> = wallets.iter().map(|w| &w.address).collect();
    for i in 0..addresses.len() {
        for j in (i + 1)..addresses.len() {
            assert_ne!(addresses[i], addresses[j], "Generated duplicate addresses");
        }
    }
} 