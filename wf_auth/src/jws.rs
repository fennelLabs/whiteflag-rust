use base64ct::{Base64UrlUnpadded, Encoding};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

/// JWT Header for Whiteflag authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    /// Algorithm identifier (always "ES256" for Whiteflag compatibility)
    pub alg: String,
    /// Token type (always "JWT")
    pub typ: String,
}

impl Default for JwtHeader {
    fn default() -> Self {
        Self {
            alg: "ES256".to_string(),
            typ: "JWT".to_string(),
        }
    }
}

/// Whiteflag Authentication Payload for JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteflagAuthPayload {
    /// Authentication method (1 for internet resource)
    pub sub: String,
    /// Audience - typically the Whiteflag network or domain
    pub aud: String,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
    /// Unique identifier for this token
    pub jti: String,
    /// Additional Whiteflag-specific claims
    #[serde(flatten)]
    pub whiteflag_claims: serde_json::Value,
}

/// Complete JWS token structure for Whiteflag
#[derive(Debug, Clone)]
pub struct WhiteflagJwsToken {
    /// Base64url-encoded header
    pub header: String,
    /// Base64url-encoded payload
    pub payload: String,
    /// Base64url-encoded signature
    pub signature: String,
}

impl WhiteflagJwsToken {
    /// Create a new JWS token from components
    pub fn new(
        header: &JwtHeader,
        payload: &WhiteflagAuthPayload,
        signature: &[u8],
    ) -> Result<Self, JwsError> {
        let header_json = serde_json::to_vec(header)?;
        let payload_json = serde_json::to_vec(payload)?;

        Ok(Self {
            header: Base64UrlUnpadded::encode_string(&header_json),
            payload: Base64UrlUnpadded::encode_string(&payload_json),
            signature: Base64UrlUnpadded::encode_string(signature),
        })
    }

    /// Get the signing input (header.payload)
    pub fn signing_input(&self) -> String {
        format!("{}.{}", self.header, self.payload)
    }

    /// Get the complete JWS compact serialization
    pub fn compact(&self) -> String {
        format!("{}.{}.{}", self.header, self.payload, self.signature)
    }

    /// Parse a JWS token from compact serialization
    pub fn from_compact(token: &str) -> Result<Self, JwsError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwsError::InvalidFormat(
                "JWS must have exactly 3 parts".to_string(),
            ));
        }

        Ok(Self {
            header: parts[0].to_string(),
            payload: parts[1].to_string(),
            signature: parts[2].to_string(),
        })
    }

    /// Verify the signature using provided public key
    pub fn verify_signature(&self, _public_key: &[u8]) -> Result<bool, JwsError> {
        // This is a placeholder - actual verification depends on the signature algorithm
        // For ECDSA P-256, we would use the p256 crate
        // For sr25519, we would use schnorrkel
        Ok(true) // Simplified for now
    }

    /// Create a hash of the token for sr25519 compatibility
    pub fn token_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.compact().as_bytes());
        hasher.finalize().into()
    }
}

impl fmt::Display for WhiteflagJwsToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.compact())
    }
}

/// JWS-related errors
#[derive(Debug)]
pub enum JwsError {
    InvalidFormat(String),
    SerializationError(serde_json::Error),
    SignatureError(String),
    ValidationError(String),
}

impl fmt::Display for JwsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwsError::InvalidFormat(msg) => write!(f, "Invalid JWS format: {msg}"),
            JwsError::SerializationError(err) => write!(f, "Serialization error: {err}"),
            JwsError::SignatureError(msg) => write!(f, "Signature error: {msg}"),
            JwsError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for JwsError {}

impl From<serde_json::Error> for JwsError {
    fn from(err: serde_json::Error) -> Self {
        JwsError::SerializationError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_jwt_header_default() {
        let header = JwtHeader::default();
        assert_eq!(header.alg, "ES256");
        assert_eq!(header.typ, "JWT");
    }

    #[test]
    fn test_jws_token_creation() {
        let header = JwtHeader::default();
        let payload = WhiteflagAuthPayload {
            sub: "whiteflag-validator".to_string(),
            aud: "whiteflag-network".to_string(),
            iat: Utc::now().timestamp(),
            exp: Utc::now().timestamp() + 3600,
            jti: "unique-token-id".to_string(),
            whiteflag_claims: serde_json::json!({"method": 1}),
        };
        let signature = b"dummy_signature";

        let token = WhiteflagJwsToken::new(&header, &payload, signature).unwrap();
        assert!(!token.header.is_empty());
        assert!(!token.payload.is_empty());
        assert!(!token.signature.is_empty());
    }

    #[test]
    fn test_jws_compact_format() {
        let header = JwtHeader::default();
        let payload = WhiteflagAuthPayload {
            sub: "test".to_string(),
            aud: "test".to_string(),
            iat: 1234567890,
            exp: 1234571490,
            jti: "test-id".to_string(),
            whiteflag_claims: serde_json::json!({}),
        };
        let signature = b"sig";

        let token = WhiteflagJwsToken::new(&header, &payload, signature).unwrap();
        let compact = token.compact();

        // Should have 3 parts separated by dots
        assert_eq!(compact.matches('.').count(), 2);

        // Should be able to parse it back
        let parsed = WhiteflagJwsToken::from_compact(&compact).unwrap();
        assert_eq!(parsed.header, token.header);
        assert_eq!(parsed.payload, token.payload);
        assert_eq!(parsed.signature, token.signature);
    }
}
