use fennel_lib::dh_tools::{get_shared_secret, get_session_secret, get_session_public_key};
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};
/// Whiteflag ECDH Key Pair class
/// 
/// This class represents an Elleptic Curve Diffie-Hellmann key pair
/// used by Whiteflag for cryptographic key negotiation. The elliptic curve
/// parameters that must be used for Whiteflag are defined by the
/// brainpoolP256r1 curve as specified in RFC 5639. Public keys are shared
/// as raw 264-bit compressed public ECDH keys.
/// 
/// @wfver v1-draft.6
/// Whiteflag Specification 5.2.2 Key Agreement

    /// The name of the elleptic curve used by Whiteflag for ECDH hey negotiation
    /// Whiteflag Specification 5.2.2 Key Agreement
    const CURVENAME: &str = "brainpoolP256R1";

    /// Cryptographic parameters
    const ALGORITHM: &str = "ECDH";
    const PROVIDER: &str = "BC";
    const PUBKEYLENGTH: usize = 33;

    struct WhiteflagECDHKeyPair {
    /// Status of the instance
   destroyed: bool,

    /// Main key pair properties
    session_secret: StaticSecret,
    public_key: PublicKey,
    shared_secret: SharedSecret,
    }

    /// Constructs a new Whiteflag ECDH key pair
    fn generate_keypair() {
        createKeyPair()
    }
    
    /// Constructs a new Whiteflag ECDH key pair from an existing private key
    fn generate_keypair_from_key(ecPrivateKey: ECPrivateKey) {
        createKeyPair(ecPrivateKey)
    }
    

impl WfECDHKeyPair for WhiteflagECDHKeyPair {
    /// Destroys this Whiteflag ECDH key pair by clearing the private key
    fn destroy(&self) {
        keypair.getPrivate().destroy();    // Destroy derived key; throws exceptions
        self.destroyed = true;
    }
    
    /// Determine if this Whiteflag cipher has been destroyed.
    fn is_destroyed(&self) -> bool {
        self.destroyed
    }
    
    /// Returns the public key of this key pair
    fn getPublicKey(&self) -> PublicKey {
        checkDestroyed();
        &self.public_key
    }
    
    /// Returns the raw public key of the ECDH key pair
    fn getRawPublicKey() -> Vec<u8> {
        checkDestroyed();
        return compressPublicKey(getPublicKey());
    }
    
    /// Calculates the negotiated shared key with an originator
    fn negotiateKey(ecPublicKey: Vec<u8>) -> Vec<u8> {
        checkDestroyed();
        try {
            KeyAgreement ka = KeyAgreement.getInstance(ALGORITHM, PROVIDER);
            ka.init(keypair.getPrivate());
            ka.doPhase(createPublicKey(rawPublicKey), true);
            return ka.generateSecret();
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not generate negotiated key from ECDH public key", e);
        }
    }
    
    /// Creates a new random ECDH key pair with the curve specified for Whiteflag key negotiation
    fn createKeyPair() -> KeyPair {
        try {
            KeyPairGenerator kpg = KeyPairGenerator.getInstance(ALGORITHM, PROVIDER);
            kpg.initialize(ecParamSpec);
            return kpg.generateKeyPair();
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not generate new ECDH key pair", e);
        }
    }
    
    /// Creates an ECDH key pair from an existing private key with the curve specified for Whiteflag key negotiation
    fn createKeyPair(ecPrivateKey: ECPrivateKey) -> KeyPair {
        try {
            KeyFactory kf = KeyFactory.getInstance(ALGORITHM, PROVIDER);
            ECPoint point = ecParamSpec.getG().multiply(ecPrivateKey.getS());
            ECPublicKeySpec ecPubkeySpec = getPublicKeySpec(point.getEncoded(false));
            return new KeyPair(kf.generatePublic(ecPubkeySpec), ecPrivateKey);
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not generate ECDH key pair from existing private key", e);
        }
    }
    
    /// Creates an ECDH public key object from a byte array
    fn createPublicKey(String rawPublicKey) -> ECPublicKey {
        return createPublicKey(convertToByteArray(rawPublicKey));
    }
    
    /// Creates an ECDH private key object from a byte array
    fn createPrivateKey(rawPrivateKey: Vec<u8>) -> ECPrivateKey {
        try {
            KeyFactory kf = KeyFactory.getInstance(ALGORITHM, PROVIDER);
            ECPrivateKeySpec ecPrivkeySpec = new ECPrivateKeySpec(new BigInteger(rawPrivateKey), ecParamSpec);
            return (ECPrivateKey) kf.generatePrivate(ecPrivkeySpec);
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not create ECDH private key from raw private key", e);
        }
    }
    
    /// Checks and throws exception if this key pair has been destroyed
    fn check_destroyed() -> WhiteflagCryptoResult<()> {
        return if (destroyed) {
            Err(WhiteflagCryptoError::KeypairDestroyed)
        } else {
            Ok()
        }
    }
}
