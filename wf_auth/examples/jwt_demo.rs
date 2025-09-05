//! # Whiteflag JWT Authentication Example
//! 
//! This example demonstrates how to use the extended Whiteflag authentication system
//! with JWT/JWS support for sr25519 and ECDSA dual-algorithm signing.

use wf_auth::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Whiteflag JWT Authentication Example");
    println!("========================================\n");

    // Example 1: Generate a new dual-algorithm signer
    println!("1. Generating dual-algorithm signer...");
    let signer = WhiteflagSigner::generate()?;
    println!("   ✅ Generated sr25519 and ECDSA P-256 keypairs");
    
    // Display public keys
    let sr25519_pk = signer.sr25519_public_key();
    let ecdsa_pk = signer.ecdsa_public_key();
    println!("   📋 SR25519 Public Key: {}", hex::encode(sr25519_pk.to_bytes()));
    println!("   📋 ECDSA Public Key: {}", hex::encode(ecdsa_pk.to_encoded_point(false)));

    // Example 2: Create JWT authentication instance
    println!("\n2. Creating JWT authentication instance...");
    let jwt_auth = WhiteflagJwtAuth::new(
        signer.clone(),
        "fennel-validator-001".to_string(),
        "whiteflag-polkadot".to_string(),
    );
    println!("   ✅ JWT auth configured for subject: {}", jwt_auth.subject());
    println!("   ✅ JWT auth configured for audience: {}", jwt_auth.audience());

    // Example 3: Create Whiteflag Method 1 authentication token
    println!("\n3. Creating Method 1 authentication token...");
    let mut additional_claims = HashMap::new();
    additional_claims.insert("validator_type".to_string(), serde_json::json!("fennel"));
    additional_claims.insert("network".to_string(), serde_json::json!("polkadot"));
    additional_claims.insert("chain_id".to_string(), serde_json::json!("polkadot"));
    
    let auth_token = jwt_auth.create_method1_token(
        "https://fennel.network/whiteflag/auth/v1",
        Some(additional_claims),
    )?;
    
    println!("   ✅ Created Whiteflag authentication token");
    println!("   📋 Token type: {:?}", auth_token.as_ref());
    println!("   📋 Is JWT token: {}", auth_token.is_jwt_token());

    // Example 4: Extract and display the JWT
    println!("\n4. Extracting JWT/JWS token...");
    let jws_token = auth_token.get_jws_token()?.unwrap();
    let compact_jwt = jws_token.compact();
    
    println!("   ✅ Extracted JWS compact serialization");
    println!("   📋 JWT Length: {} characters", compact_jwt.len());
    println!("   📋 JWT: {}", compact_jwt);
    
    // Show JWT structure
    let parts: Vec<&str> = compact_jwt.split('.').collect();
    println!("   📋 Header: {}", parts[0]);
    println!("   📋 Payload: {}", parts[1]);
    println!("   📋 Signature: {}...", &parts[2][..20]);

    // Example 5: Verify the token
    println!("\n5. Verifying the JWT token...");
    let claims = jwt_auth.verify_token(&compact_jwt)?;
    println!("   ✅ Token verification successful");
    println!("   📋 Subject: {}", claims.sub);
    println!("   📋 Audience: {}", claims.aud);
    println!("   📋 Issued at: {}", chrono::DateTime::from_timestamp(claims.iat, 0).unwrap());
    println!("   📋 Expires at: {}", chrono::DateTime::from_timestamp(claims.exp, 0).unwrap());
    println!("   📋 Token ID: {}", claims.jti);
    println!("   📋 Whiteflag method: {}", claims.whiteflag_claims.get("method").unwrap());
    println!("   📋 Internet resource: {}", claims.whiteflag_claims.get("resource").unwrap());

    // Example 6: Generate JWKS (JSON Web Key Set)
    println!("\n6. Generating JWKS for public key distribution...");
    let jwks = jwt_auth.create_jwks()?;
    println!("   ✅ Generated JWKS");
    println!("   📋 JWKS:\n{}", serde_json::to_string_pretty(&jwks)?);

    // Example 7: Demonstrate dual-algorithm signing
    println!("\n7. Demonstrating dual-algorithm signing...");
    let test_message = b"Whiteflag authentication message for dual signing demo";
    
    // Sign with sr25519 (for Substrate/Polkadot compatibility)
    let sr25519_sig = signer.sign_sr25519(test_message);
    println!("   ✅ SR25519 signature created");
    println!("   📋 SR25519 sig length: {} bytes", sr25519_sig.to_bytes().len());
    
    // Sign with ECDSA (for JWT/standard compatibility)
    let ecdsa_sig = signer.sign_ecdsa(test_message)?;
    println!("   ✅ ECDSA signature created");
    println!("   📋 ECDSA sig length: {} bytes", ecdsa_sig.to_bytes().len());
    
    // Verify both signatures
    let sr25519_valid = WhiteflagSigner::verify_sr25519(
        &signer.sr25519_public_key(),
        &sr25519_sig,
        test_message,
    );
    let ecdsa_valid = WhiteflagSigner::verify_ecdsa(
        &signer.ecdsa_public_key(),
        &ecdsa_sig,
        test_message,
    )?;
    
    println!("   ✅ SR25519 verification: {}", sr25519_valid);
    println!("   ✅ ECDSA verification: {}", ecdsa_valid);

    // Example 8: Create hybrid authentication
    println!("\n8. Creating hybrid authentication...");
    let hybrid_auth = signer.create_hybrid_auth(test_message)?;
    let hybrid_valid = hybrid_auth.verify()?;
    println!("   ✅ Hybrid authentication created and verified: {}", hybrid_valid);
    
    let compact_hybrid = hybrid_auth.to_compact()?;
    println!("   📋 Compact hybrid auth length: {} characters", compact_hybrid.len());

    // Example 9: Deterministic signer from seed
    println!("\n9. Demonstrating deterministic key generation...");
    let seed = [0x42u8; 32]; // Example seed
    let deterministic_signer = WhiteflagSigner::from_seed(&seed)?;
    println!("   ✅ Generated deterministic signer from seed");
    println!("   📋 Seed: {}", hex::encode(seed));
    println!("   📋 Deterministic SR25519 PK: {}", hex::encode(deterministic_signer.sr25519_public_key().to_bytes()));

    // Example 10: Backward compatibility
    println!("\n10. Demonstrating backward compatibility...");
    let legacy_secret = b"legacy_preshared_secret_for_method_2";
    let legacy_token = WhiteflagAuthToken::new(legacy_secret.to_vec());
    println!("   ✅ Created legacy authentication token");
    println!("   📋 Is JWT token: {}", legacy_token.is_jwt_token());
    
    let context = b"test_verification_context";
    let verification_data = legacy_token.get_verification_data(context)?;
    println!("   ✅ Generated verification data: {} bytes", verification_data.len());

    println!("\n🎉 All examples completed successfully!");
    println!("\nIntegration Notes:");
    println!("- Use the JWT tokens for Whiteflag API integration");
    println!("- Use sr25519 signatures for Substrate/Polkadot compatibility");
    println!("- JWKS can be published for public key verification");
    println!("- Hybrid auth provides both signature types when needed");
    println!("- Existing Method 2 authentication remains unchanged");

    Ok(())
}
