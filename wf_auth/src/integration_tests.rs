use crate::*;
use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::Utc;
use std::collections::HashMap;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_whiteflag_jwt_workflow() {
        // Generate a signer
        let signer = WhiteflagSigner::generate().unwrap();

        // Create JWT auth instance
        let jwt_auth = WhiteflagJwtAuth::new(
            signer.clone(),
            "fennel-validator-001".to_string(),
            "whiteflag-mainnet".to_string(),
        );

        // Create a Method 1 authentication token
        let mut additional_claims = HashMap::new();
        additional_claims.insert("validator_type".to_string(), serde_json::json!("fennel"));
        additional_claims.insert("network".to_string(), serde_json::json!("polkadot"));

        let auth_token = jwt_auth
            .create_method1_token(
                "https://fennel.network/whiteflag/auth",
                Some(additional_claims),
            )
            .unwrap();

        // Verify it's a JWT token
        assert!(auth_token.is_jwt_token());
        assert!(matches!(
            auth_token.as_ref(),
            &AuthenticationMethod::PresharedToken
        ));

        // Extract and verify the JWS
        let jws_token = auth_token.get_jws_token().unwrap().unwrap();
        let compact_jwt = jws_token.compact();

        // Should be valid JWT format
        assert_eq!(compact_jwt.matches('.').count(), 2);

        // Verify the token
        let claims = jwt_auth.verify_token(&compact_jwt).unwrap();
        assert_eq!(claims.sub, "fennel-validator-001");
        assert_eq!(claims.aud, "whiteflag-mainnet");
        assert!(
            claims
                .whiteflag_claims
                .get("method")
                .unwrap()
                .as_i64()
                .unwrap()
                == 1
        );
        assert_eq!(
            claims
                .whiteflag_claims
                .get("resource")
                .unwrap()
                .as_str()
                .unwrap(),
            "https://fennel.network/whiteflag/auth"
        );

        println!("✅ Complete JWT workflow test passed");
        println!("Generated JWT: {}", compact_jwt);
    }

    #[test]
    fn test_dual_algorithm_compatibility() {
        // Create deterministic signer for testing
        let seed = [0x42u8; 32];
        let signer = WhiteflagSigner::from_seed(&seed).unwrap();

        let test_data = b"Whiteflag authentication test message";

        // Test sr25519 signing (for Substrate compatibility)
        let sr25519_sig = signer.sign_sr25519(test_data);
        assert!(WhiteflagSigner::verify_sr25519(
            &signer.sr25519_public_key(),
            &sr25519_sig,
            test_data
        ));

        // Test ECDSA signing (for JWT compatibility)
        let ecdsa_sig = signer.sign_ecdsa(test_data).unwrap();
        assert!(
            WhiteflagSigner::verify_ecdsa(&signer.ecdsa_public_key(), &ecdsa_sig, test_data)
                .unwrap()
        );

        // Test hybrid authentication
        let hybrid_auth = signer.create_hybrid_auth(test_data).unwrap();
        assert!(hybrid_auth.verify().unwrap());

        // Test serialization
        let compact = hybrid_auth.to_compact().unwrap();
        assert!(!compact.is_empty());

        println!("✅ Dual algorithm compatibility test passed");
        println!(
            "SR25519 signature length: {} bytes",
            sr25519_sig.to_bytes().len()
        );
        println!(
            "ECDSA signature length: {} bytes",
            ecdsa_sig.to_bytes().len()
        );
    }

    #[test]
    fn test_jwks_generation() {
        let signer = WhiteflagSigner::generate().unwrap();
        let jwt_auth = WhiteflagJwtAuth::new(
            signer,
            "test-validator".to_string(),
            "test-network".to_string(),
        );

        let jwks = jwt_auth.create_jwks().unwrap();

        // Verify JWKS structure
        assert!(jwks.get("keys").is_some());
        let keys = jwks.get("keys").unwrap().as_array().unwrap();
        assert_eq!(keys.len(), 1);

        let key = &keys[0];
        assert_eq!(key.get("kty").unwrap().as_str().unwrap(), "EC");
        assert_eq!(key.get("crv").unwrap().as_str().unwrap(), "P-256");
        assert_eq!(key.get("alg").unwrap().as_str().unwrap(), "ES256");
        assert_eq!(key.get("use").unwrap().as_str().unwrap(), "sig");
        assert!(key.get("kid").is_some());
        assert!(key.get("x").is_some());
        assert!(key.get("y").is_some());

        println!("✅ JWKS generation test passed");
        println!("JWKS: {}", serde_json::to_string_pretty(&jwks).unwrap());
    }

    #[test]
    fn test_extended_auth_token_functionality() {
        let signer = WhiteflagSigner::generate().unwrap();

        // Test the extended WhiteflagAuthToken functionality
        let auth_token = WhiteflagAuthToken::create_method1_jwt(
            &signer,
            "fennel-validator",
            "whiteflag-network",
            "https://api.fennel.network/auth",
            7200, // 2 hours
        )
        .unwrap();

        // Test token properties
        assert!(auth_token.is_jwt_token());

        let jws = auth_token.get_jws_token().unwrap().unwrap();
        let compact = jws.compact();

        // Verify JWT structure
        let parts: Vec<&str> = compact.split('.').collect();
        assert_eq!(parts.len(), 3);

        // Decode and verify header
        let header_bytes = Base64UrlUnpadded::decode_vec(parts[0]).unwrap();
        let header: JwtHeader = serde_json::from_slice(&header_bytes).unwrap();
        assert_eq!(header.alg, "ES256");
        assert_eq!(header.typ, "JWT");

        // Decode and verify payload
        let payload_bytes = Base64UrlUnpadded::decode_vec(parts[1]).unwrap();
        let payload: WhiteflagAuthPayload = serde_json::from_slice(&payload_bytes).unwrap();
        assert_eq!(payload.sub, "fennel-validator");
        assert_eq!(payload.aud, "whiteflag-network");
        assert!(
            payload
                .whiteflag_claims
                .get("method")
                .unwrap()
                .as_i64()
                .unwrap()
                == 1
        );
        assert_eq!(
            payload
                .whiteflag_claims
                .get("resource")
                .unwrap()
                .as_str()
                .unwrap(),
            "https://api.fennel.network/auth"
        );

        println!("✅ Extended auth token functionality test passed");
    }

    #[test]
    fn test_compatibility_with_existing_auth() {
        // Test that existing WhiteflagAuthToken functionality still works
        let secret = b"test_secret_key_for_preshared_auth";
        let auth_token = WhiteflagAuthToken::new(secret.to_vec());

        // Should work with existing methods
        assert!(matches!(
            auth_token.as_ref(),
            &AuthenticationMethod::PresharedToken
        ));

        let context = b"test_context";
        let verification_data = auth_token.get_verification_data(context).unwrap();
        assert!(!verification_data.is_empty());
        assert_eq!(verification_data.len(), 32); // HKDF output length

        // Should not be a JWT token
        assert!(!auth_token.is_jwt_token());

        println!("✅ Backward compatibility test passed");
    }

    #[test]
    fn test_deterministic_key_generation() {
        let seed1 = [0x01u8; 32];
        let seed2 = [0x01u8; 32];
        let seed3 = [0x02u8; 32];

        let signer1 = WhiteflagSigner::from_seed(&seed1).unwrap();
        let signer2 = WhiteflagSigner::from_seed(&seed2).unwrap();
        let signer3 = WhiteflagSigner::from_seed(&seed3).unwrap();

        // Same seed should produce same keys
        assert_eq!(
            signer1.sr25519_public_key().to_bytes(),
            signer2.sr25519_public_key().to_bytes()
        );
        assert_eq!(
            signer1.ecdsa_public_key().to_encoded_point(false),
            signer2.ecdsa_public_key().to_encoded_point(false)
        );

        // Different seed should produce different keys
        assert_ne!(
            signer1.sr25519_public_key().to_bytes(),
            signer3.sr25519_public_key().to_bytes()
        );
        assert_ne!(
            signer1.ecdsa_public_key().to_encoded_point(false),
            signer3.ecdsa_public_key().to_encoded_point(false)
        );

        println!("✅ Deterministic key generation test passed");
    }

    #[test]
    fn test_token_expiration_handling() {
        let signer = WhiteflagSigner::generate().unwrap();
        let mut jwt_auth = WhiteflagJwtAuth::new(
            signer,
            "test-validator".to_string(),
            "test-network".to_string(),
        );

        // Set very short validity
        jwt_auth.set_default_validity(1); // 1 second

        let auth_token = jwt_auth
            .create_method1_token("https://example.com/auth", None)
            .unwrap();

        let jws = auth_token.get_jws_token().unwrap().unwrap();
        let compact = jws.compact();

        // Token should be valid immediately
        let claims = jwt_auth.verify_token(&compact).unwrap();
        assert!(claims.exp > Utc::now().timestamp());

        // Wait for expiration (in real test, you might mock time instead)
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Token should now be expired
        match jwt_auth.verify_token(&compact) {
            Err(WhiteflagJwtError::TokenExpired) => {
                println!("✅ Token expiration correctly detected");
            }
            _ => panic!("Token should have been expired"),
        }
    }
}

/// Performance and stress tests
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_signing_performance() {
        let signer = WhiteflagSigner::generate().unwrap();
        let test_data = b"Performance test message for signing benchmarks";
        let iterations = 100;

        // Benchmark sr25519 signing
        let start = Instant::now();
        for _ in 0..iterations {
            let _sig = signer.sign_sr25519(test_data);
        }
        let sr25519_duration = start.elapsed();

        // Benchmark ECDSA signing
        let start = Instant::now();
        for _ in 0..iterations {
            let _sig = signer.sign_ecdsa(test_data).unwrap();
        }
        let ecdsa_duration = start.elapsed();

        println!("📊 Signing Performance (100 iterations):");
        println!(
            "  SR25519: {:?} ({:.2} ms/op)",
            sr25519_duration,
            sr25519_duration.as_millis() as f64 / iterations as f64
        );
        println!(
            "  ECDSA:   {:?} ({:.2} ms/op)",
            ecdsa_duration,
            ecdsa_duration.as_millis() as f64 / iterations as f64
        );

        // Both should complete reasonably quickly
        assert!(sr25519_duration.as_secs() < 5);
        assert!(ecdsa_duration.as_secs() < 5);
    }

    #[test]
    fn test_jwt_creation_performance() {
        let signer = WhiteflagSigner::generate().unwrap();
        let jwt_auth = WhiteflagJwtAuth::new(
            signer,
            "perf-validator".to_string(),
            "perf-network".to_string(),
        );

        let iterations = 50;
        let start = Instant::now();

        for i in 0..iterations {
            let _token = jwt_auth
                .create_method1_token(&format!("https://example.com/auth/{}", i), None)
                .unwrap();
        }

        let duration = start.elapsed();
        println!(
            "📊 JWT Creation Performance ({} iterations): {:?} ({:.2} ms/op)",
            iterations,
            duration,
            duration.as_millis() as f64 / iterations as f64
        );

        // Should complete reasonably quickly
        assert!(duration.as_secs() < 10);
    }
}
