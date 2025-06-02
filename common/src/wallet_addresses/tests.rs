use crate::wallet_addresses::{BscWallet, PactusWallet, WalletOperations};

#[test]
fn test_bsc_wallet_generation() {
    let wallet = BscWallet::generate_keypair().unwrap();
    println!("BSC Wallet Address: {}", wallet.address);
    println!("BSC Private Key: {}", hex::encode(&wallet.private_key));
    println!("BSC Public Key: {}", hex::encode(&wallet.public_key));
    
    // Test BSC address format
    assert!(wallet.address.starts_with("0x"));
    assert_eq!(wallet.address.len(), 42); // 0x + 40 hex chars
    
    // Test key lengths
    assert_eq!(wallet.private_key.len(), 32); // secp256k1 private key is 32 bytes
    assert_eq!(wallet.public_key.len(), 65); // uncompressed public key is 65 bytes (0x04 + 64 bytes)
    
    // Test address derivation from public key
    let derived_address = BscWallet::derive_address(&wallet.public_key).unwrap();
    assert_eq!(derived_address, wallet.address);
}

#[test]
fn test_pactus_wallet_generation() {
    let wallet = PactusWallet::generate_keypair().unwrap();
    println!("Pactus Wallet Address: {}", wallet.address);
    println!("Pactus Private Key: {}", hex::encode(&wallet.private_key));
    println!("Pactus Public Key: {}", hex::encode(&wallet.public_key));
    
    // Test Pactus key lengths
    assert_eq!(wallet.private_key.len(), 32); // ed25519 private key is 32 bytes
    assert_eq!(wallet.public_key.len(), 32); // ed25519 public key is 32 bytes
    
    // Test address derivation from public key
    let derived_address = PactusWallet::derive_address(&wallet.public_key).unwrap();
    assert_eq!(derived_address, wallet.address);
    
    // Test that the address is valid Base58
    assert!(base58::decode(&wallet.address).is_ok());
}

#[test]
fn test_multiple_wallet_generation() {
    // Generate multiple wallets and ensure they're all unique
    let wallets: Vec<_> = (0..10)
        .map(|_| BscWallet::generate_keypair().unwrap())
        .collect();
    
    // Check that all addresses are unique
    let addresses: Vec<_> = wallets.iter().map(|w| &w.address).collect();
    for (i, addr1) in addresses.iter().enumerate() {
        for (j, addr2) in addresses.iter().enumerate() {
            if i != j {
                assert_ne!(addr1, addr2, "Generated duplicate addresses");
            }
        }
    }
}

#[test]
fn test_invalid_public_key() {
    // Test with invalid public key length
    let invalid_key = vec![0u8; 31]; // Too short for both BSC and Pactus
    assert!(BscWallet::derive_address(&invalid_key).is_err());
    assert!(PactusWallet::derive_address(&invalid_key).is_err());
} 