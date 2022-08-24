use fennel_lib::dh_tools::{get_session_public_key, get_session_secret, get_shared_secret};
use fennel_lib::AESCipher;
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

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
        let secret: StaticSecret = get_session_secret();
        let pair = Self::from_secret(secret);
        pair
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
        self.negotiate_as_shared_secret(other).to_bytes().to_vec()
    }

    /// Calculates the negotiated shared key with an originator
    pub fn negotiate_as_shared_secret(&self, other: &PublicKey) -> SharedSecret {
        let secret = get_shared_secret(self.session_secret.clone(), other);
        secret
    }

    pub fn create_aes_cipher(&self, public_key: &PublicKey) -> AESCipher {
        AESCipher::new_from_shared_secret(self.negotiate_as_shared_secret(public_key).as_bytes())
    }
}

impl Default for WhiteflagECDHKeyPair {
    fn default() -> Self {
        Self::new()
    }
}
