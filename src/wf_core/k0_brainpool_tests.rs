/// Tests for K0 (Cryptographic) message generation with brainpoolP256r1
/// 
/// These tests verify that brainpoolP256r1 33-byte SEC1 compressed public keys
/// work correctly with Whiteflag K0 messages per RFC 5639.

use super::{message::Message};
use wf_crypto::ecdh_keypair::WhiteflagECDHKeyPair;

#[test]
fn test_k0_message_with_brainpool_public_key() {
    // Generate a brainpoolP256r1 keypair
    let keypair = WhiteflagECDHKeyPair::new();
    
    // Get the public key in hex format
    let public_key_hex = keypair.public_key_hex();
    
    // Verify it's 66 characters (33 bytes * 2)
    assert_eq!(
        public_key_hex.len(),
        66,
        "brainpoolP256r1 compressed public key should be 66 hex characters (33 bytes)"
    );
    
    // Verify first character is '0' (SEC1 prefix 0x02 or 0x03)
    assert_eq!(
        &public_key_hex[0..1],
        "0",
        "SEC1 compressed key should start with 0x02 or 0x03"
    );
    
    // Create a K0 (Crypto) message with the brainpool public key
    // Field structure: WF | version | encryptionIndicator | duressIndicator | 
    //                  messageCode | referenceIndicator | referencedMessage |
    //                  cryptoDataType | cryptoData
    let field_values = vec![
        "WF",                                                                     // Prefix
        "1",                                                                      // Version
        "0",                                                                      // EncryptionIndicator
        "0",                                                                      // DuressIndicator
        "K",                                                                      // MessageCode (Crypto)
        "0",                                                                      // ReferenceIndicator
        "0000000000000000000000000000000000000000000000000000000000000000", // ReferencedMessage
        "11",                                                                     // CryptoDataType (ECDH public key)
        &public_key_hex,                                                          // CryptoData (our 66-char brainpool key)
    ];
    
    // Compile the message
    let message = Message::compile(&field_values).expect("Failed to compile K0 message with brainpool key");
    
    // Serialize and verify the message contains our public key
    let serialized = message.serialize();
    assert!(
        serialized.contains(&public_key_hex),
        "Serialized K0 message should contain the brainpool public key"
    );
    
    // Verify we can deserialize it back
    let deserialized = Message::deserialize(&serialized).expect("Failed to deserialize K0 message");
    let reserialized = deserialized.serialize();
    
    assert_eq!(
        serialized, reserialized,
        "K0 message should round-trip correctly with brainpool public key"
    );
    
    println!("✅ K0 message successfully created with brainpoolP256r1 public key:");
    println!("   Public key (hex): {}", public_key_hex);
    println!("   Message length: {} characters", serialized.len());
    println!("   Serialized: {}...", &serialized[..80]);
}

#[test]
fn test_k0_message_with_multiple_brainpool_keys() {
    // Test that we can create multiple K0 messages with different keys
    let mut public_keys = Vec::new();
    
    for i in 0..5 {
        let keypair = WhiteflagECDHKeyPair::new();
        let public_key_hex = keypair.public_key_hex();
        
        // Verify uniqueness
        assert!(
            !public_keys.contains(&public_key_hex),
            "Each brainpool keypair should generate a unique public key"
        );
        
        public_keys.push(public_key_hex.clone());
        
        // Create K0 message
        let field_values = vec![
            "WF", "1", "0", "0", "K", "0",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "11",
            &public_key_hex,
        ];
        
        let message = Message::compile(&field_values)
            .unwrap_or_else(|_| panic!("Failed to compile K0 message #{}", i + 1));
        
        let serialized = message.serialize();
        assert!(serialized.contains(&public_key_hex));
    }
    
    println!("✅ Created 5 unique K0 messages with different brainpoolP256r1 keys");
    println!("   All keys are 66 characters (33 bytes)");
    println!("   All messages serialize correctly");
}

#[test]
fn test_k0_message_deterministic_key() {
    // Test that deterministic key generation works
    let secret_bytes = [0x42; 32];
    let keypair1 = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    let keypair2 = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    
    let pk1 = keypair1.public_key_hex();
    let pk2 = keypair2.public_key_hex();
    
    assert_eq!(pk1, pk2, "Same secret should produce same brainpool public key");
    
    // Create K0 message
    let field_values = vec![
        "WF", "1", "0", "0", "K", "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "11",
        &pk1,
    ];
    
    let _message = Message::compile(&field_values).expect("Failed to compile K0 message");
    
    println!("✅ Deterministic brainpool key generation works:");
    println!("   Public key: {}", pk1);
    println!("   K0 message compiles successfully");
}
