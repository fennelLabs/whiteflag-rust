use crate::dual_signer::{SignerError, WhiteflagSigner};
use crate::jws::{JwsError, WhiteflagJwsToken};
use crate::WhiteflagAuthToken;
use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::{Duration, Utc};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Whiteflag JWT authentication implementation
/// Extends the existing authentication system with JWT/JWS support for Method 1
pub struct WhiteflagJwtAuth {
    /// The dual-algorithm signer
    signer: WhiteflagSigner,
    /// Default token validity duration (in seconds)
    default_validity: i64,
    /// Subject identifier for this validator/node
    subject: String,
    /// Audience for Whiteflag network
    audience: String,
}

impl WhiteflagJwtAuth {
    /// Create a new JWT authentication instance
    pub fn new(signer: WhiteflagSigner, subject: String, audience: String) -> Self {
        Self {
            signer,
            default_validity: 3600, // 1 hour default
            subject,
            audience,
        }
    }

    /// Create from a seed (deterministic)
    pub fn from_seed(
        seed: &[u8; 32],
        subject: String,
        audience: String,
    ) -> Result<Self, WhiteflagJwtError> {
        let signer = WhiteflagSigner::from_seed(seed)?;
        Ok(Self::new(signer, subject, audience))
    }

    /// Set the default token validity duration
    pub fn set_default_validity(&mut self, seconds: i64) {
        self.default_validity = seconds;
    }

    /// Create a Whiteflag Authentication Method 1 JWT token
    pub fn create_method1_token(
        &self,
        internet_resource: &str,
        additional_claims: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<WhiteflagAuthToken, WhiteflagJwtError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.default_validity);
        let jti = uuid::Uuid::new_v4().to_string();

        // Build Whiteflag-specific claims
        let mut whiteflag_claims = json!({
            "method": 1,
            "resource": internet_resource,
            "whiteflag_version": "1.0",
        });

        // Add any additional claims
        if let Some(claims) = additional_claims {
            if let serde_json::Value::Object(ref mut map) = whiteflag_claims {
                for (key, value) in claims {
                    map.insert(key, value);
                }
            }
        }

        // Create the JWS token
        let jws = self.signer.create_whiteflag_jws(
            &self.subject,
            &self.audience,
            now.timestamp(),
            exp.timestamp(),
            &jti,
            whiteflag_claims,
        )?;

        // Convert to WhiteflagAuthToken
        let token_bytes = jws.compact().into_bytes();
        let mut auth_token = WhiteflagAuthToken::new(token_bytes);

        // Store the JWS for later retrieval
        auth_token.set_jws_token(jws);

        Ok(auth_token)
    }

    /// Verify a Whiteflag JWT token
    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, WhiteflagJwtError> {
        let jws = WhiteflagJwsToken::from_compact(token)?;

        // For now, we'll do basic parsing - full verification would require
        // the issuer's public key
        let payload_bytes = Base64UrlUnpadded::decode_vec(&jws.payload)
            .map_err(|e| WhiteflagJwtError::InvalidToken(format!("Base64 decode error: {}", e)))?;

        let claims: TokenClaims = serde_json::from_slice(&payload_bytes)?;

        // Basic validation
        let now = Utc::now().timestamp();
        if claims.exp < now {
            return Err(WhiteflagJwtError::TokenExpired);
        }

        if claims.iat > now {
            return Err(WhiteflagJwtError::InvalidToken(
                "Token issued in future".to_string(),
            ));
        }

        Ok(claims)
    }

    /// Get the signer for direct access
    pub fn signer(&self) -> &WhiteflagSigner {
        &self.signer
    }

    /// Get the subject identifier
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Get the audience
    pub fn audience(&self) -> &str {
        &self.audience
    }

    /// Create a verification key set (JWK Set) for this signer
    pub fn create_jwks(&self) -> Result<serde_json::Value, WhiteflagJwtError> {
        let ecdsa_pk = self.signer.ecdsa_public_key();
        let point = ecdsa_pk.to_encoded_point(false);

        // Extract coordinates from the uncompressed point
        let point_bytes = point.as_bytes();
        // Skip the first byte (0x04) which indicates uncompressed format
        let x_bytes = &point_bytes[1..33];
        let y_bytes = &point_bytes[33..65];

        let jwk = json!({
            "kty": "EC",
            "crv": "P-256",
            "x": Base64UrlUnpadded::encode_string(x_bytes),
            "y": Base64UrlUnpadded::encode_string(y_bytes),
            "use": "sig",
            "alg": "ES256",
            "kid": self.create_key_id()?,
        });

        Ok(json!({
            "keys": [jwk]
        }))
    }

    /// Create a unique key identifier
    fn create_key_id(&self) -> Result<String, WhiteflagJwtError> {
        let ecdsa_pk = self.signer.ecdsa_public_key();
        let point = ecdsa_pk.to_encoded_point(false);
        let hash = Sha256::digest(point.as_bytes());
        Ok(Base64UrlUnpadded::encode_string(&hash[..8]))
    }
}

/// Token claims structure
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
    #[serde(flatten)]
    pub whiteflag_claims: serde_json::Value,
}

/// Extended WhiteflagAuthToken with JWT support
impl WhiteflagAuthToken {
    /// Set the associated JWS token
    pub fn set_jws_token(&mut self, jws: WhiteflagJwsToken) {
        // Store as JSON in token field for now
        // In a real implementation, you might want a separate field
        let jws_json = serde_json::json!({
            "jws": jws.compact(),
            "type": "jwt_method1"
        });
        self.token = jws_json.to_string().into_bytes();
    }

    /// Get the JWS token if present
    pub fn get_jws_token(&self) -> Result<Option<WhiteflagJwsToken>, JwsError> {
        let token_str = String::from_utf8_lossy(&self.token);

        // Try to parse as JSON first
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&token_str) {
            if let Some(jws_str) = json_val.get("jws").and_then(|v| v.as_str()) {
                return Ok(Some(WhiteflagJwsToken::from_compact(jws_str)?));
            }
        }

        // Try to parse as direct JWS
        if token_str.matches('.').count() == 2 {
            return Ok(Some(WhiteflagJwsToken::from_compact(&token_str)?));
        }

        Ok(None)
    }

    /// Check if this token supports JWT/JWS
    pub fn is_jwt_token(&self) -> bool {
        self.get_jws_token().unwrap_or(None).is_some()
    }

    /// Create a Method 1 JWT token using the provided signer
    pub fn create_method1_jwt(
        signer: &WhiteflagSigner,
        subject: &str,
        audience: &str,
        internet_resource: &str,
        validity_seconds: i64,
    ) -> Result<Self, WhiteflagJwtError> {
        let jwt_auth =
            WhiteflagJwtAuth::new(signer.clone(), subject.to_string(), audience.to_string());

        let mut claims = HashMap::new();
        claims.insert("validity".to_string(), json!(validity_seconds));

        jwt_auth.create_method1_token(internet_resource, Some(claims))
    }
}

/// JWT-specific errors
#[derive(Debug)]
pub enum WhiteflagJwtError {
    SignerError(SignerError),
    JwsError(JwsError),
    SerializationError(serde_json::Error),
    InvalidToken(String),
    TokenExpired,
    InvalidClaims(String),
}

impl std::fmt::Display for WhiteflagJwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhiteflagJwtError::SignerError(e) => write!(f, "Signer error: {}", e),
            WhiteflagJwtError::JwsError(e) => write!(f, "JWS error: {}", e),
            WhiteflagJwtError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            WhiteflagJwtError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            WhiteflagJwtError::TokenExpired => write!(f, "Token has expired"),
            WhiteflagJwtError::InvalidClaims(msg) => write!(f, "Invalid claims: {}", msg),
        }
    }
}

impl std::error::Error for WhiteflagJwtError {}

impl From<SignerError> for WhiteflagJwtError {
    fn from(err: SignerError) -> Self {
        WhiteflagJwtError::SignerError(err)
    }
}

impl From<JwsError> for WhiteflagJwtError {
    fn from(err: JwsError) -> Self {
        WhiteflagJwtError::JwsError(err)
    }
}

impl From<serde_json::Error> for WhiteflagJwtError {
    fn from(err: serde_json::Error) -> Self {
        WhiteflagJwtError::SerializationError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_auth_creation() {
        let signer = WhiteflagSigner::generate().unwrap();
        let jwt_auth = WhiteflagJwtAuth::new(
            signer,
            "test-validator".to_string(),
            "whiteflag-testnet".to_string(),
        );

        assert_eq!(jwt_auth.subject(), "test-validator");
        assert_eq!(jwt_auth.audience(), "whiteflag-testnet");
    }

    #[test]
    fn test_method1_token_creation() {
        let signer = WhiteflagSigner::generate().unwrap();
        let jwt_auth = WhiteflagJwtAuth::new(
            signer,
            "test-validator".to_string(),
            "whiteflag-testnet".to_string(),
        );

        let token = jwt_auth
            .create_method1_token("https://example.com/whiteflag/auth", None)
            .unwrap();

        assert!(token.is_jwt_token());
        assert!(matches!(
            token.as_ref(),
            &crate::AuthenticationMethod::PresharedToken
        ));
    }

    #[test]
    fn test_jwks_creation() {
        let signer = WhiteflagSigner::generate().unwrap();
        let jwt_auth = WhiteflagJwtAuth::new(
            signer,
            "test-validator".to_string(),
            "whiteflag-testnet".to_string(),
        );

        let jwks = jwt_auth.create_jwks().unwrap();
        assert!(jwks.get("keys").is_some());

        let keys = jwks.get("keys").unwrap().as_array().unwrap();
        assert_eq!(keys.len(), 1);

        let key = &keys[0];
        assert_eq!(key.get("kty").unwrap().as_str().unwrap(), "EC");
        assert_eq!(key.get("crv").unwrap().as_str().unwrap(), "P-256");
        assert_eq!(key.get("alg").unwrap().as_str().unwrap(), "ES256");
    }

    #[test]
    fn test_token_extension() {
        let signer = WhiteflagSigner::generate().unwrap();

        let token = WhiteflagAuthToken::create_method1_jwt(
            &signer,
            "test-validator",
            "whiteflag-testnet",
            "https://example.com/auth",
            3600,
        )
        .unwrap();

        assert!(token.is_jwt_token());
        let jws = token.get_jws_token().unwrap().unwrap();
        assert!(!jws.compact().is_empty());
    }
}
