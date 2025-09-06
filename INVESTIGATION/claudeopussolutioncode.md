// Cargo.toml additions
/*
[dependencies]
schnorrkel = "0.11"
jsonwebtoken = "9.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64ct = { version = "1.6", features = ["alloc"] }
p256 = { version = "0.13", features = ["ecdsa", "jwk"] }
chrono = "0.4"
sha2 = "0.10"
*/

use schnorrkel::{Keypair as Sr25519Keypair, Signature as Sr25519Signature};
use p256::ecdsa::{SigningKey, VerifyingKey, Signature as P256Signature};
use serde::{Deserialize, Serialize};
use base64ct::{Base64UrlUnpadded, Encoding};
use sha2::{Sha256, Digest};

/// JWT Header structure for both standard and custom algorithms
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

/// Whiteflag Authentication Payload (per spec)
#[derive(Debug, Serialize, Deserialize)]
pub struct WhiteflagAuthPayload {
    pub addr: String,      // Blockchain address
    pub orgname: String,   // Organization name
    pub url: String,       // Verification URL
    
    // Standard JWT claims
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,  // Issued at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,  // Expiration
    
    // Whiteflag specific claims
    #[serde(rename = "wf_method")]
    pub wf_method: String, // "1" for internet resource
}

/// Custom JWS Token supporting both sr25519 and ECDSA
pub struct WhiteflagJwsToken {
    pub protected: String,
    pub payload: String,
    pub signature: String,
}

impl WhiteflagJwsToken {
    /// Serialize to standard JWT compact format
    pub fn to_compact(&self) -> String {
        format!("{}.{}.{}", self.protected, self.payload, self.signature)
    }
    
    /// Parse from standard JWT compact format
    pub fn from_compact(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid JWT format".into());
        }
        
        Ok(WhiteflagJwsToken {
            protected: parts[0].to_string(),
            payload: parts[1].to_string(),
            signature: parts[2].to_string(),
        })
    }
}

/// Dual-key signer supporting both sr25519 and ECDSA
pub struct WhiteflagDualSigner {
    // Internal sr25519 for Substrate compatibility
    sr25519_keypair: Sr25519Keypair,
    // ECDSA for Whiteflag JWT compatibility
    ecdsa_keypair: SigningKey,
}

impl WhiteflagDualSigner {
    /// Create new dual signer from seed material
    pub fn new_from_seed(seed: &[u8; 32]) -> Result<Self, Box<dyn std::error::Error>> {
        // Derive sr25519 keypair
        let sr25519_keypair = Sr25519Keypair::from_bytes(seed)
            .map_err(|e| format!("Failed to create sr25519 keypair: {:?}", e))?;
        
        // Derive ECDSA keypair using deterministic derivation
        let ecdsa_seed = Self::derive_ecdsa_seed(seed);
        let ecdsa_keypair = SigningKey::from_bytes(&ecdsa_seed.into())
            .map_err(|e| format!("Failed to create ECDSA keypair: {:?}", e))?;
        
        Ok(Self {
            sr25519_keypair,
            ecdsa_keypair,
        })
    }
    
    /// Deterministic derivation of ECDSA seed from sr25519 seed
    fn derive_ecdsa_seed(sr25519_seed: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"whiteflag-ecdsa-derivation");
        hasher.update(sr25519_seed);
        let result = hasher.finalize();
        
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&result);
        seed
    }
    
    /// Create standard-compliant JWT for Whiteflag (using ECDSA)
    pub fn create_whiteflag_jwt(
        &self,
        payload: &WhiteflagAuthPayload,
    ) -> Result<WhiteflagJwsToken, Box<dyn std::error::Error>> {
        // Standard JWT header for ES256 (ECDSA with P-256)
        let header = JwtHeader {
            alg: "ES256".to_string(),
            typ: "JWT".to_string(),
        };
        
        // Encode header and payload
        let header_json = serde_json::to_vec(&header)?;
        let header_b64 = Base64UrlUnpadded::encode_string(&header_json);
        
        let payload_json = serde_json::to_vec(&payload)?;
        let payload_b64 = Base64UrlUnpadded::encode_string(&payload_json);
        
        // Create signing input
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        
        // Sign with ECDSA
        let signature: P256Signature = self.ecdsa_keypair.sign(signing_input.as_bytes());
        let signature_bytes = signature.to_bytes();
        let signature_b64 = Base64UrlUnpadded::encode_string(&signature_bytes);
        
        Ok(WhiteflagJwsToken {
            protected: header_b64,
            payload: payload_b64,
            signature: signature_b64,
        })
    }
    
    /// Create custom JWT with sr25519 (for internal use)
    pub fn create_sr25519_jwt(
        &self,
        payload: &WhiteflagAuthPayload,
    ) -> Result<WhiteflagJwsToken, Box<dyn std::error::Error>> {
        // Custom header for sr25519
        let header = JwtHeader {
            alg: "Sr25519".to_string(),  // Custom algorithm identifier
            typ: "JWT".to_string(),
        };
        
        // Encode header and payload
        let header_json = serde_json::to_vec(&header)?;
        let header_b64 = Base64UrlUnpadded::encode_string(&header_json);
        
        let payload_json = serde_json::to_vec(&payload)?;
        let payload_b64 = Base64UrlUnpadded::encode_string(&payload_json);
        
        // Create signing input
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        
        // Sign with sr25519
        let signature = self.sr25519_keypair.sign_simple(
            b"whiteflag-jwt",
            signing_input.as_bytes(),
        );
        let signature_b64 = Base64UrlUnpadded::encode_string(&signature.to_bytes());
        
        Ok(WhiteflagJwsToken {
            protected: header_b64,
            payload: payload_b64,
            signature: signature_b64,
        })
    }
    
    /// Get blockchain address (can be derived from either keypair)
    pub fn get_blockchain_address(&self) -> String {
        // Implementation depends on your blockchain
        // This is a placeholder
        format!("0x{}", hex::encode(&self.ecdsa_keypair.verifying_key().to_bytes()[1..21]))
    }
}

/// Custom JWT verifier supporting both algorithms
pub struct WhiteflagJwtVerifier;

impl WhiteflagJwtVerifier {
    /// Verify JWT with automatic algorithm detection
    pub fn verify_token(
        token: &str,
        public_key_bytes: &[u8],
    ) -> Result<WhiteflagAuthPayload, Box<dyn std::error::Error>> {
        let jwt = WhiteflagJwsToken::from_compact(token)?;
        
        // Decode header to determine algorithm
        let header_bytes = Base64UrlUnpadded::decode_vec(&jwt.protected)?;
        let header: JwtHeader = serde_json::from_slice(&header_bytes)?;
        
        // Decode payload
        let payload_bytes = Base64UrlUnpadded::decode_vec(&jwt.payload)?;
        let payload: WhiteflagAuthPayload = serde_json::from_slice(&payload_bytes)?;
        
        // Verify signature based on algorithm
        let signing_input = format!("{}.{}", jwt.protected, jwt.payload);
        let signature_bytes = Base64UrlUnpadded::decode_vec(&jwt.signature)?;
        
        let is_valid = match header.alg.as_str() {
            "ES256" => {
                // Standard ECDSA verification
                let verifying_key = VerifyingKey::from_sec1_bytes(public_key_bytes)?;
                let signature = P256Signature::from_bytes(&signature_bytes.into())?;
                verifying_key.verify(signing_input.as_bytes(), &signature).is_ok()
            }
            "Sr25519" => {
                // Custom sr25519 verification
                Self::verify_sr25519(&signing_input, &signature_bytes, public_key_bytes)?
            }
            _ => return Err(format!("Unsupported algorithm: {}", header.alg).into()),
        };
        
        if !is_valid {
            return Err("Invalid signature".into());
        }
        
        Ok(payload)
    }
    
    fn verify_sr25519(
        message: &str,
        signature_bytes: &[u8],
        public_key_bytes: &[u8],
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Implementation of sr25519 verification
        // This is a placeholder - implement actual verification
        Ok(true)
    }
}

/// Integration with existing WhiteflagAuthToken
pub trait WhiteflagAuthExtension {
    fn create_method1_jwt(
        &self,
        org_name: &str,
        verification_url: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

impl WhiteflagAuthExtension for WhiteflagAuthToken {
    fn create_method1_jwt(
        &self,
        org_name: &str,
        verification_url: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Extract seed from existing token
        let seed = self.get_seed_material()?;
        let signer = WhiteflagDualSigner::new_from_seed(&seed)?;
        
        let payload = WhiteflagAuthPayload {
            addr: signer.get_blockchain_address(),
            orgname: org_name.to_string(),
            url: verification_url.to_string(),
            wf_method: "1".to_string(),
            iat: Some(chrono::Utc::now().timestamp()),
            exp: Some(chrono::Utc::now().timestamp() + 3600), // 1 hour expiry
        };
        
        // Create standard-compliant JWT for Whiteflag
        let jwt = signer.create_whiteflag_jwt(&payload)?;
        Ok(jwt.to_compact())
    }
}

// Placeholder for existing WhiteflagAuthToken
struct WhiteflagAuthToken {
    token: Vec<u8>,
}

impl WhiteflagAuthToken {
    fn get_seed_material(&self) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // Extract seed from token - implementation specific
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&self.token[..32]);
        Ok(seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dual_signing() {
        let seed = [0x42u8; 32];
        let signer = WhiteflagDualSigner::new_from_seed(&seed).unwrap();
        
        let payload = WhiteflagAuthPayload {
            addr: signer.get_blockchain_address(),
            orgname: "Test Organization".to_string(),
            url: "https://example.org/whiteflag".to_string(),
            wf_method: "1".to_string(),
            iat: Some(1234567890),
            exp: Some(1234571490),
        };
        
        // Test standard ECDSA JWT
        let ecdsa_jwt = signer.create_whiteflag_jwt(&payload).unwrap();
        println!("ECDSA JWT: {}", ecdsa_jwt.to_compact());
        
        // Test custom sr25519 JWT
        let sr25519_jwt = signer.create_sr25519_jwt(&payload).unwrap();
        println!("Sr25519 JWT: {}", sr25519_jwt.to_compact());
    }
}