use super::ecdh_keypair::WhiteflagECDHKeyPair;

#[test]
fn test_generate_random_keypair() {
    let keypair = WhiteflagECDHKeyPair::new();
    let public_key = keypair.public_key_compressed();
    
    // brainpoolP256r1 compressed public key should be 33 bytes (0x02/0x03 prefix + 32 bytes)
    assert_eq!(public_key.len(), 33);
    
    // First byte should be 0x02 or 0x03 (SEC1 compressed format)
    assert!(public_key[0] == 0x02 || public_key[0] == 0x03);
}

#[test]
fn test_generate_keypair_from_bytes() {
    let secret_bytes = [0x42; 32]; // Fixed 32-byte secret
    let keypair = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    let public_key = keypair.public_key_compressed();
    
    // Should produce deterministic public key from same secret
    assert_eq!(public_key.len(), 33);
    assert!(public_key[0] == 0x02 || public_key[0] == 0x03);
    
    // Generate again from same bytes - should be identical
    let keypair2 = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    assert_eq!(public_key, keypair2.public_key_compressed());
}

#[test]
fn test_ecdh_key_agreement() {
    // Generate two keypairs
    let keypair1 = WhiteflagECDHKeyPair::new();
    let keypair2 = WhiteflagECDHKeyPair::new();
    
    // Each side computes shared secret using other's public key
    let shared_secret1 = keypair1.negotiate(&keypair2.public_key_compressed());
    let shared_secret2 = keypair2.negotiate(&keypair1.public_key_compressed());
    
    // Shared secrets should match
    assert_eq!(shared_secret1, shared_secret2);
    
    // ECDH shared secret should be 32 bytes (x-coordinate of point)
    assert_eq!(shared_secret1.len(), 32);
}

#[test]
fn test_public_key_hex_encoding() {
    let secret_bytes = [0x42; 32];
    let keypair = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    let hex_string = keypair.public_key_hex();
    
    // Hex string should be 66 characters (33 bytes * 2)
    assert_eq!(hex_string.len(), 66);
    
    // Should be valid hex
    assert!(hex_string.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_deterministic_key_generation() {
    let secret_bytes = [0x01; 32];
    
    // Generate multiple keypairs from same secret
    let keypair1 = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    let keypair2 = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    let keypair3 = WhiteflagECDHKeyPair::from_bytes(&secret_bytes);
    
    // All should produce identical public keys
    assert_eq!(keypair1.public_key_compressed(), keypair2.public_key_compressed());
    assert_eq!(keypair2.public_key_compressed(), keypair3.public_key_compressed());
}

#[test]
fn test_different_secrets_produce_different_keys() {
    let secret1 = [0x01; 32];
    let secret2 = [0x02; 32];
    
    let keypair1 = WhiteflagECDHKeyPair::from_bytes(&secret1);
    let keypair2 = WhiteflagECDHKeyPair::from_bytes(&secret2);
    
    // Different secrets should produce different public keys
    assert_ne!(keypair1.public_key_compressed(), keypair2.public_key_compressed());
}

#[test]
fn test_negotiate_key_default_keypairs() {
    let keypair1 = WhiteflagECDHKeyPair::default();
    let keypair2 = WhiteflagECDHKeyPair::default();

    let shared_secret1 = keypair1.negotiate(&keypair2.public_key_compressed());
    let shared_secret2 = keypair2.negotiate(&keypair1.public_key_compressed());

    assert_eq!(shared_secret1, shared_secret2);
}
