use fennel_lib::dh_tools::{get_session_public_key, get_session_secret, get_shared_secret};
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};
/// Whiteflag ECDH Key Pair class
///
/// This class represents an Elleptic Curve Diffie-Hellmann key pair
/// used by Whiteflag for cryptographic key negotiation.
///
/// The current Fennel implemenation uses curve-x25519.
///
/// Whiteflag Specification 5.2.2 Key Agreement

struct WhiteflagECDHKeyPair {
    /// Status of the instance
    destroyed: bool,

    /// Main key pair properties
    session_secret: StaticSecret,
    public_key: PublicKey,
    shared_secret: SharedSecret,
}

/// Constructs a new Whiteflag ECDH key pair
fn generate_keypair() -> WhiteflagECDHKeyPair {
    WhiteflagECDHKeyPair::createKeyPair()
}

/// Constructs a new Whiteflag ECDH key pair from an existing private key
fn generate_keypair_from_key(ecPrivateKey: ECPrivateKey) -> WhiteflagECDHKeyPair {
    WhiteflagECDHKeyPair::createKeyPair(ecPrivateKey)
}

impl WfECDHKeyPair for WhiteflagECDHKeyPair {
    /// Returns the public key of this key pair
    fn getPublicKey(&self) -> PublicKey {
        &self.public_key
    }

    /// Returns the raw public key of the ECDH key pair
    fn getRawPublicKey() -> Vec<u8> {
        getPublicKey().as_bytes()
    }

    /// Calculates the negotiated shared key with an originator
    fn negotiateKey(&self, ecPublicKey: Vec<u8>) -> Vec<u8> {
        get_shared_secret(self.session_secret, ecPublicKey).as_bytes()
    }

    /// Creates a new random ECDH key with the curve specified for Whiteflag key negotiation
    fn createKeyPair() -> (PublicKey, StaticSecret) {
        let secret: StaticSecret = get_session_secret();
        (get_session_public_key(secret), secret)
    }

    /// Creates an ECDH key pair from an existing private key with the curve specified for Whiteflag key negotiation
    fn createKeyPair(secret: StaticSecret) -> (PublicKey, StaticSecret) {
        (get_session_public_key(secret), secret)
    }

    /// Creates an ECDH public key object from a byte array
    fn createPublicKey(rawPublicKey: &str) -> PublicKey {
        PublicKey::from(hex::decode(rawPublicKey).unwrap())
    }

    /// Creates an ECDH private key object from a byte array
    fn createPrivateKey(rawPrivateKey: Vec<u8>) -> StaticSecret {
        StaticSecret::from(rawPrivateKey)
    }
}
