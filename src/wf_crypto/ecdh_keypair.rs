use fennel_lib::dh_tools::{get_session_public_key, get_session_secret, get_shared_secret};
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};
/// Whiteflag ECDH Key Pair class
///
/// This class represents an Elleptic Curve Diffie-Hellmann key pair
/// used by Whiteflag for cryptographic key negotiation.
///
/// The current Fennel implemenation uses curve-x25519 to verify feature completenesss.
///
/// Whiteflag Specification 5.2.2 Key Agreement

pub trait WfECDHKeyPair {
    /// Returns the public key of this key pair
    fn get_public_key(&self) -> PublicKey;
    fn get_shared_secret(&self) -> Option<&SharedSecret>;

    /// Returns the raw public key of the ECDH key pair
    fn get_raw_public_key(&self) -> [u8; 32];

    /// Calculates the negotiated shared key with an originator
    fn negotiate_key(&mut self, public_key: [u8; 32]);

    /// Creates a new random ECDH key with the curve specified for Whiteflag key negotiation
    fn create_keypair() -> WhiteflagECDHKeyPair;

    /// Creates an ECDH key pair from an existing private key with the curve specified for Whiteflag key negotiation
    fn create_keypair_from_secret(secret: StaticSecret) -> WhiteflagECDHKeyPair;

    /// Private helper function to prevent rewriting.
    fn _get_keypair_with_secret(secret: StaticSecret) -> WhiteflagECDHKeyPair;

    /// Creates an ECDH public key object from a byte array
    fn create_public_key_from_raw(raw_public_key: [u8; 32]) -> PublicKey;

    /// Creates an ECDH private key object from a byte array
    fn create_private_key_from_raw(raw_private_key: [u8; 32]) -> StaticSecret;
}

pub struct WhiteflagECDHKeyPair {
    /// Main key pair properties
    session_secret: StaticSecret,
    public_key: PublicKey,
    shared_secret: Option<SharedSecret>,
}

/// Constructs a new Whiteflag ECDH key pair
pub fn generate_wfkeypair() -> WhiteflagECDHKeyPair {
    WhiteflagECDHKeyPair::create_keypair()
}

/// Constructs a new Whiteflag ECDH key pair from an existing private key
pub fn generate_wfkeypair_from_key(private_key: StaticSecret) -> WhiteflagECDHKeyPair {
    WhiteflagECDHKeyPair::create_keypair_from_secret(private_key)
}

impl WfECDHKeyPair for WhiteflagECDHKeyPair {
    /// Returns the public key of this key pair
    fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    fn get_shared_secret(&self) -> Option<&SharedSecret> {
        self.shared_secret.as_ref()
    }

    /// Returns the raw public key of the ECDH key pair
    fn get_raw_public_key(&self) -> [u8; 32] {
        *self.get_public_key().as_bytes()
    }

    /// Calculates the negotiated shared key with an originator
    fn negotiate_key(&mut self, public_key: [u8; 32]) {
        let secret = get_shared_secret(self.session_secret.clone(), &PublicKey::from(public_key));
        self.shared_secret = Some(secret);
    }

    /// Creates a new random ECDH key with the curve specified for Whiteflag key negotiation
    fn create_keypair() -> WhiteflagECDHKeyPair {
        let secret: StaticSecret = get_session_secret();
        let pair = Self::_get_keypair_with_secret(secret);
        pair
    }

    /// Creates an ECDH key pair from an existing private key with the curve specified for Whiteflag key negotiation
    fn create_keypair_from_secret(secret: StaticSecret) -> WhiteflagECDHKeyPair {
        let pair = Self::_get_keypair_with_secret(secret);
        pair
    }

    fn _get_keypair_with_secret(secret: StaticSecret) -> WhiteflagECDHKeyPair {
        let public_key = get_session_public_key(&secret);
        let session_secret = secret;
        WhiteflagECDHKeyPair {
            session_secret,
            public_key,
            shared_secret: None,
        }
    }

    /// Creates an ECDH public key object from a byte array
    fn create_public_key_from_raw(raw_public_key: [u8; 32]) -> PublicKey {
        PublicKey::from(raw_public_key)
    }

    /// Creates an ECDH private key object from a byte array
    fn create_private_key_from_raw(raw_private_key: [u8; 32]) -> StaticSecret {
        StaticSecret::from(raw_private_key)
    }
}
