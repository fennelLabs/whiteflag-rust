use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// Dependencies needed in Cargo.toml:
// schnorrkel = "0.11"
// k256 = { version = "0.13", features = ["ecdsa", "sha256"] }
// jsonwebtoken = "8.3"
// base64ct = { version = "1.6", features = ["alloc"] }
// serde_json = "1.0"
// This demonstrates how to integrate sr25519 with JWT/JWS structure

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sr25519JwsHeader {
    pub alg: String,  // "Sr25519"
    pub typ: String,  // "JWT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>, // Key ID for key rotation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteflagJwtClaims {
    // Standard JWT claims
    pub iss: String,        // Issuer (Whiteflag originator)
    pub sub: String,        // Subject (message identifier)
    pub aud: String,        // Audience (Whiteflag network)
    pub exp: u64,           // Expiration time
    pub iat: u64,           // Issued at
    pub nbf: u64,           // Not before
    
    // Whiteflag-specific claims
    pub wf_method: u8,      // Authentication method (1 = Internet Resource)
    pub wf_resource: String, // Internet resource URL
    pub wf_verification: String, // Verification data
}

pub struct Sr25519JwsToken {
    header: Sr25519JwsHeader,
    payload: WhiteflagJwtClaims,
    signature: Vec<u8>,
}

impl Sr25519JwsToken {
    pub fn new(claims: WhiteflagJwtClaims) -> Self {
        let header = Sr25519JwsHeader {
            alg: "Sr25519".to_string(),
            typ: "JWT".to_string(),
            kid: None,
        };
        
        Self {
            header,
            payload: claims,
            signature: Vec::new(),
        }
    }
    
    pub fn sign_with_sr25519(&mut self, private_key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Create signing input (header.payload)
        let header_b64 = self.encode_header()?;
        let payload_b64 = self.encode_payload()?;
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        
        // This would use schnorrkel/sr25519 for actual signing
        // For now, showing the structure
        self.signature = self.sr25519_sign(signing_input.as_bytes(), private_key)?;
        
        Ok(())
    }
    
    pub fn to_compact_serialization(&self) -> Result<String, Box<dyn std::error::Error>> {
        let header_b64 = self.encode_header()?;
        let payload_b64 = self.encode_payload()?;
        let signature_b64 = base64_url_encode(&self.signature);
        
        Ok(format!("{}.{}.{}", header_b64, payload_b64, signature_b64))
    }
    
    pub fn verify(&self, public_key: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        let header_b64 = self.encode_header()?;
        let payload_b64 = self.encode_payload()?;
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        
        self.sr25519_verify(signing_input.as_bytes(), &self.signature, public_key)
    }
    
    // Base64URL encoding helpers
    fn encode_header(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self.header)?;
        Ok(base64_url_encode(json.as_bytes()))
    }
    
    fn encode_payload(&self) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self.payload)?;
        Ok(base64_url_encode(json.as_bytes()))
    }
    
    // Sr25519 crypto operations (placeholder - implement with schnorrkel)
    fn sr25519_sign(&self, message: &[u8], private_key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // TODO: Implement with schnorrkel
        // let keypair = schnorrkel::Keypair::from_bytes(private_key)?;
        // let signature = keypair.sign(message);
        // Ok(signature.to_bytes().to_vec())
        
        // Placeholder
        Ok(vec![0u8; 64]) // sr25519 signatures are 64 bytes
    }
    
    fn sr25519_verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        // TODO: Implement with schnorrkel
        // let public = schnorrkel::PublicKey::from_bytes(public_key)?;
        // let sig = schnorrkel::Signature::from_bytes(signature)?;
        // Ok(public.verify(message, &sig).is_ok())
        
        // Placeholder
        Ok(true)
    }
}

// Utility function for base64url encoding (without padding)
fn base64_url_encode(input: &[u8]) -> String {
    base64::encode_config(input, base64::URL_SAFE_NO_PAD)
}

// Integration with existing Whiteflag auth
impl From<crate::WhiteflagAuthToken> for WhiteflagJwtClaims {
    fn from(auth_token: crate::WhiteflagAuthToken) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        WhiteflagJwtClaims {
            iss: "whiteflag-rust".to_string(),
            sub: "whiteflag-message".to_string(),
            aud: "whiteflag-network".to_string(),
            exp: now + 3600, // 1 hour expiry
            iat: now,
            nbf: now,
            wf_method: auth_token.as_ref().get_method_code(),
            wf_resource: "https://example.com/.well-known/whiteflag".to_string(),
            wf_verification: hex::encode(auth_token.get_verification_data("context").unwrap()),
        }
    }
}

// Example usage for Whiteflag Authentication Method 1
pub fn create_authentication_jwt(
    auth_token: crate::WhiteflagAuthToken,
    sr25519_private_key: &[u8]
) -> Result<String, Box<dyn std::error::Error>> {
    let claims = WhiteflagJwtClaims::from(auth_token);
    let mut jwt = Sr25519JwsToken::new(claims);
    jwt.sign_with_sr25519(sr25519_private_key)?;
    jwt.to_compact_serialization()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sr25519_jwt_structure() {
        let claims = WhiteflagJwtClaims {
            iss: "test-issuer".to_string(),
            sub: "test-subject".to_string(),
            aud: "whiteflag".to_string(),
            exp: 1234567890,
            iat: 1234567800,
            nbf: 1234567800,
            wf_method: 1,
            wf_resource: "https://example.com/.well-known/whiteflag".to_string(),
            wf_verification: "abc123".to_string(),
        };
        
        let jwt = Sr25519JwsToken::new(claims);
        
        assert_eq!(jwt.header.alg, "Sr25519");
        assert_eq!(jwt.header.typ, "JWT");
        assert_eq!(jwt.payload.wf_method, 1);
    }
    
    #[test]
    fn test_compact_serialization_format() {
        let claims = WhiteflagJwtClaims {
            iss: "test".to_string(),
            sub: "test".to_string(),
            aud: "test".to_string(),
            exp: 123,
            iat: 123,
            nbf: 123,
            wf_method: 1,
            wf_resource: "https://test.com".to_string(),
            wf_verification: "test".to_string(),
        };
        
        let jwt = Sr25519JwsToken::new(claims);
        let serialized = jwt.to_compact_serialization().unwrap();
        
        // Should have format: header.payload.signature
        let parts: Vec<&str> = serialized.split('.').collect();
        assert_eq!(parts.len(), 3);
    }
}
