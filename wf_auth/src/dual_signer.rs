use crate::jws::{JwtHeader, WhiteflagAuthPayload, WhiteflagJwsToken};
use base64ct::{Base64UrlUnpadded, Encoding};
use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{
    Signature as EcdsaSignature, SigningKey as EcdsaSigningKey, VerifyingKey as EcdsaVerifyingKey,
};
use schnorrkel::signing_context;
use schnorrkel::{
    Keypair as Sr25519Keypair, PublicKey as Sr25519PublicKey, Signature as Sr25519Signature,
};
use sha2::{Digest, Sha256};
use std::fmt;

/// Signing context for Whiteflag sr25519 signatures
const WHITEFLAG_SIGNING_CONTEXT: &[u8] = b"whiteflag-auth-v1";

/// Dual-algorithm signer supporting both sr25519 and ECDSA P-256
#[derive(Clone)]
pub struct WhiteflagSigner {
    /// SR25519 keypair for Substrate/Polkadot compatibility
    sr25519_keypair: Sr25519Keypair,
    /// ECDSA P-256 signing key for JWT/JWS compatibility
    ecdsa_key: EcdsaSigningKey,
}

impl WhiteflagSigner {
    /// Create a new signer with provided keys
    pub fn new(sr25519_keypair: Sr25519Keypair, ecdsa_key: EcdsaSigningKey) -> Self {
        Self {
            sr25519_keypair,
            ecdsa_key,
        }
    }

    /// Generate a new signer with random keys
    pub fn generate() -> Result<Self, SignerError> {
        // Generate sr25519 keypair
        let sr25519_keypair = Sr25519Keypair::generate();

        // Generate ECDSA P-256 keypair
        let ecdsa_key = EcdsaSigningKey::random(&mut rand::thread_rng());

        Ok(Self::new(sr25519_keypair, ecdsa_key))
    }

    /// Create a signer from seed (deterministic)
    pub fn from_seed(seed: &[u8; 32]) -> Result<Self, SignerError> {
        // Create sr25519 keypair from seed using mini_secret_key
        use schnorrkel::MiniSecretKey;
        let mini_secret = MiniSecretKey::from_bytes(seed).map_err(|e| {
            SignerError::KeyGeneration(format!("sr25519 mini secret from seed: {e}"))
        })?;
        let sr25519_keypair = mini_secret.expand_to_keypair(schnorrkel::ExpansionMode::Ed25519);

        // Create ECDSA key from seed (using seed as entropy)
        let ecdsa_key = EcdsaSigningKey::from_bytes(seed.into())
            .map_err(|e| SignerError::KeyGeneration(format!("ECDSA from seed: {e}")))?;

        Ok(Self::new(sr25519_keypair, ecdsa_key))
    }

    /// Get the sr25519 public key
    pub fn sr25519_public_key(&self) -> Sr25519PublicKey {
        self.sr25519_keypair.public
    }

    /// Get the ECDSA verifying key (public key)
    pub fn ecdsa_public_key(&self) -> EcdsaVerifyingKey {
        *self.ecdsa_key.verifying_key()
    }

    /// Sign data with sr25519
    pub fn sign_sr25519(&self, data: &[u8]) -> Sr25519Signature {
        let context = signing_context(WHITEFLAG_SIGNING_CONTEXT);
        self.sr25519_keypair.sign(context.bytes(data))
    }

    /// Sign data with ECDSA P-256
    pub fn sign_ecdsa(&self, data: &[u8]) -> Result<EcdsaSignature, SignerError> {
        // Hash the data first (ECDSA signs the hash, not raw data)
        let hash = Sha256::digest(data);
        let signature = self
            .ecdsa_key
            .try_sign(&hash)
            .map_err(|e| SignerError::SigningError(format!("ECDSA signing failed: {e}")))?;
        Ok(signature)
    }

    /// Create a JWS token for Whiteflag authentication
    pub fn create_whiteflag_jws(
        &self,
        subject: &str,
        audience: &str,
        issued_at: i64,
        expires_at: i64,
        token_id: &str,
        additional_claims: serde_json::Value,
    ) -> Result<WhiteflagJwsToken, SignerError> {
        // Create JWT header
        let header = JwtHeader::default();

        // Create payload
        let payload = WhiteflagAuthPayload {
            sub: subject.to_string(),
            aud: audience.to_string(),
            iat: issued_at,
            exp: expires_at,
            jti: token_id.to_string(),
            whiteflag_claims: additional_claims,
        };

        // Serialize header and payload for signing
        let header_json = serde_json::to_vec(&header)
            .map_err(|e| SignerError::SerializationError(e.to_string()))?;
        let payload_json = serde_json::to_vec(&payload)
            .map_err(|e| SignerError::SerializationError(e.to_string()))?;

        // Create signing input
        let header_b64 = Base64UrlUnpadded::encode_string(&header_json);
        let payload_b64 = Base64UrlUnpadded::encode_string(&payload_json);
        let signing_input = format!("{header_b64}.{payload_b64}");

        // Sign with sr25519 for Polkadot/Substrate compatibility
        let sr25519_signature = self.sign_sr25519(signing_input.as_bytes());
        let signature_bytes = sr25519_signature.to_bytes();

        // Create the JWS token
        WhiteflagJwsToken::new(&header, &payload, &signature_bytes)
            .map_err(|e| SignerError::TokenCreation(e.to_string()))
    }

    /// Create a hybrid authentication that includes both signatures
    pub fn create_hybrid_auth(&self, data: &[u8]) -> Result<HybridAuth, SignerError> {
        // Sign with both algorithms
        let sr25519_sig = self.sign_sr25519(data);
        let ecdsa_sig = self.sign_ecdsa(data)?;

        Ok(HybridAuth {
            data: data.to_vec(),
            sr25519_signature: sr25519_sig,
            ecdsa_signature: ecdsa_sig,
            sr25519_public_key: self.sr25519_public_key(),
            ecdsa_public_key: self.ecdsa_public_key(),
        })
    }

    /// Verify sr25519 signature
    pub fn verify_sr25519(
        public_key: &Sr25519PublicKey,
        signature: &Sr25519Signature,
        data: &[u8],
    ) -> bool {
        let context = signing_context(WHITEFLAG_SIGNING_CONTEXT);
        public_key.verify(context.bytes(data), signature).is_ok()
    }

    /// Verify ECDSA signature
    pub fn verify_ecdsa(
        public_key: &EcdsaVerifyingKey,
        signature: &EcdsaSignature,
        data: &[u8],
    ) -> Result<bool, SignerError> {
        let hash = Sha256::digest(data);
        Ok(public_key.verify(&hash, signature).is_ok())
    }
}

/// Hybrid authentication containing both sr25519 and ECDSA signatures
#[derive(Debug, Clone)]
pub struct HybridAuth {
    pub data: Vec<u8>,
    pub sr25519_signature: Sr25519Signature,
    pub ecdsa_signature: EcdsaSignature,
    pub sr25519_public_key: Sr25519PublicKey,
    pub ecdsa_public_key: EcdsaVerifyingKey,
}

impl HybridAuth {
    /// Verify both signatures
    pub fn verify(&self) -> Result<bool, SignerError> {
        let sr25519_valid = WhiteflagSigner::verify_sr25519(
            &self.sr25519_public_key,
            &self.sr25519_signature,
            &self.data,
        );

        let ecdsa_valid = WhiteflagSigner::verify_ecdsa(
            &self.ecdsa_public_key,
            &self.ecdsa_signature,
            &self.data,
        )?;

        Ok(sr25519_valid && ecdsa_valid)
    }

    /// Get a compact representation for storage/transmission
    pub fn to_compact(&self) -> Result<String, SignerError> {
        let compact = serde_json::json!({
            "data": Base64UrlUnpadded::encode_string(&self.data),
            "sr25519_sig": hex::encode(self.sr25519_signature.to_bytes()),
            "ecdsa_sig": hex::encode(self.ecdsa_signature.to_bytes()),
            "sr25519_pk": hex::encode(self.sr25519_public_key.to_bytes()),
            "ecdsa_pk": hex::encode(self.ecdsa_public_key.to_encoded_point(false).as_bytes()),
        });

        serde_json::to_string(&compact).map_err(|e| SignerError::SerializationError(e.to_string()))
    }
}

/// Errors that can occur during signing operations
#[derive(Debug)]
pub enum SignerError {
    KeyGeneration(String),
    SigningError(String),
    SerializationError(String),
    TokenCreation(String),
    VerificationError(String),
}

impl fmt::Display for SignerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignerError::KeyGeneration(msg) => write!(f, "Key generation error: {msg}"),
            SignerError::SigningError(msg) => write!(f, "Signing error: {msg}"),
            SignerError::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
            SignerError::TokenCreation(msg) => write!(f, "Token creation error: {msg}"),
            SignerError::VerificationError(msg) => write!(f, "Verification error: {msg}"),
        }
    }
}

impl std::error::Error for SignerError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_signer_generation() {
        let signer = WhiteflagSigner::generate().unwrap();

        // Test that we can get public keys
        let _sr25519_pk = signer.sr25519_public_key();
        let _ecdsa_pk = signer.ecdsa_public_key();
    }

    #[test]
    fn test_deterministic_signer() {
        let seed = [42u8; 32];
        let signer1 = WhiteflagSigner::from_seed(&seed).unwrap();
        let signer2 = WhiteflagSigner::from_seed(&seed).unwrap();

        // Should generate same keys from same seed
        assert_eq!(
            signer1.sr25519_public_key().to_bytes(),
            signer2.sr25519_public_key().to_bytes()
        );
        assert_eq!(
            signer1.ecdsa_public_key().to_encoded_point(false),
            signer2.ecdsa_public_key().to_encoded_point(false)
        );
    }

    #[test]
    fn test_signing_and_verification() {
        let signer = WhiteflagSigner::generate().unwrap();
        let data = b"test message for signing";

        // Test sr25519
        let sr25519_sig = signer.sign_sr25519(data);
        assert!(WhiteflagSigner::verify_sr25519(
            &signer.sr25519_public_key(),
            &sr25519_sig,
            data
        ));

        // Test ECDSA
        let ecdsa_sig = signer.sign_ecdsa(data).unwrap();
        assert!(
            WhiteflagSigner::verify_ecdsa(&signer.ecdsa_public_key(), &ecdsa_sig, data).unwrap()
        );
    }

    #[test]
    fn test_whiteflag_jws_creation() {
        let signer = WhiteflagSigner::generate().unwrap();
        let now = Utc::now().timestamp();

        let jws = signer
            .create_whiteflag_jws(
                "whiteflag-validator",
                "whiteflag-network",
                now,
                now + 3600,
                "test-token-id",
                serde_json::json!({"method": 1, "validator": true}),
            )
            .unwrap();

        let compact = jws.compact();
        assert_eq!(compact.matches('.').count(), 2); // Should have 3 parts
        assert!(!compact.is_empty());
    }

    #[test]
    fn test_hybrid_auth() {
        let signer = WhiteflagSigner::generate().unwrap();
        let data = b"hybrid authentication test";

        let hybrid = signer.create_hybrid_auth(data).unwrap();
        assert!(hybrid.verify().unwrap());

        // Test compact representation
        let _compact = hybrid.to_compact().unwrap();
    }
}
