use fennel_lib::dh_tools::{get_session_public_key, get_session_secret, get_shared_secret};
use x25519_dalek::{PublicKey, StaticSecret};

use super::{
    cipher::{WfCipher, WhiteflagCipher},
    wf_encryption_key::WhiteflagEncryptionKey,
};

/// Whiteflag ECDH Key Pair class
///
/// This class represents an Elleptic Curve Diffie-Hellmann key pair
/// used by Whiteflag for cryptographic key negotiation.
///
/// The current Fennel implemenation uses curve-x25519 to verify feature completenesss.
///
/// Whiteflag Specification 5.2.2 Key Agreement
#[derive(Clone)]
pub struct WhiteflagECDHKeyPair {
    /// Main key pair properties
    session_secret: StaticSecret,
    public_key: PublicKey,
}

impl AsRef<PublicKey> for WhiteflagECDHKeyPair {
    fn as_ref(&self) -> &PublicKey {
        &self.public_key
    }
}

impl WhiteflagECDHKeyPair {
    /// Creates a new random ECDH key with the curve specified for Whiteflag key negotiation
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an ECDH key pair from an existing private key with the curve specified for Whiteflag key negotiation
    pub fn from_secret(secret: StaticSecret) -> Self {
        let public_key = get_session_public_key(&secret);
        WhiteflagECDHKeyPair {
            session_secret: secret,
            public_key,
        }
    }

    /// Calculates the negotiated shared key with an originator
    pub fn negotiate(&self, other: &PublicKey) -> Vec<u8> {
        let secret = get_shared_secret(self.session_secret.clone(), other);
        secret.to_bytes().to_vec()
    }

    pub fn create_cipher(&self, public_key: &PublicKey) -> WhiteflagCipher {
        let key = WhiteflagEncryptionKey::from_ecdh_key(public_key, &self);
        WhiteflagCipher::from_key(key)
    }
}

impl Default for WhiteflagECDHKeyPair {
    fn default() -> Self {
        let secret: StaticSecret = get_session_secret();
        let pair = Self::from_secret(secret);
        pair
    }
}
