use super::ecdh_keypair::WhiteflagECDHKeyPair;
use super::encryption_method::WhiteflagEncryptionMethod;
use aes_tools::{AESCipher, AES256CTR};
use x25519_dalek::PublicKey;

///This class represents a Whiteflag encryption key. Instances of this
///class represent the raw key, either pre-shared or negotiated, from which
///the actual key material for encryption methods 1 and 2 is created.
///
///Whiteflag Specification 5.2.3 Key and Token Derivation
///Whiteflag Specification 5.2.4 Message Encryption
#[derive(Clone)]
pub struct WhiteflagEncryptionKey {
    /* The encryption method and keys */
    /**
    ///The encryption method for which this key is valid
     */
    method: WhiteflagEncryptionMethod,
    /* The raw key materials */
    secret_key: Vec<u8>,
}

impl WhiteflagEncryptionKey {
    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    pub fn from_ecdh_key(public_key: &PublicKey, ecdh_key_pair: &WhiteflagECDHKeyPair) -> Self {
        WhiteflagEncryptionKey {
            secret_key: ecdh_key_pair.negotiate(public_key),
            method: WhiteflagEncryptionMethod::from_number(1).unwrap(),
        }
    }

    /// Constructs a new Whiteflag encryption key from a raw pre-shared key
    /// @param raw_pre_shared_key a hexadecimal string with the raw pre-shared encryption key
    pub fn from_preshared_key(raw_pre_shared_key: &str) -> Self {
        let rawkey = hex::decode(raw_pre_shared_key).unwrap();
        WhiteflagEncryptionKey {
            secret_key: rawkey,
            method: WhiteflagEncryptionMethod::from_number(2).unwrap(),
        }
    }

    /// @param context is the originators address
    pub fn set_context(&mut self, context: &[u8]) {
        let method = self.method.alg();
        self.secret_key = method.derive_secret_key(&self.secret_key, context);
    }

    pub fn fixed_raw_secret(&self) -> [u8; 32] {
        let mut init: [u8; 32] = Default::default();
        init.copy_from_slice(&self.secret_key);
        init
    }

    pub fn aes_cipher(&self) -> AESCipher {
        AESCipher::new_from_shared_secret(&self.fixed_raw_secret())
    }

    pub fn aes_256_ctr_cipher<'a>(&'a self, iv: &'a [u8]) -> AES256CTR {
        AES256CTR::new(&self.secret_key, Some(iv))
    }
}

impl From<&WhiteflagEncryptionKey> for [u8; 32] {
    fn from(key: &WhiteflagEncryptionKey) -> Self {
        key.fixed_raw_secret()
    }
}

//TOFIX
/* fn new_key_from_raw_pre_shared_key_str(raw_pre_shared_key: String) -> Self {
    how to rustify? -> this(convertToByteArray(rawPreSharedKey));
}*/

//Constructs a new Whiteflag encryption key from a raw pre-shared key
//public WfEncryptionKey(final byte[] rawPreSharedKey) {
//this.rawkey = Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
//this.method = AES_256_CTR_PSK;
//this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
//}
//TOFIX fn new_key_from_raw_pre_shared_key_vec(raw_pre_shared_key: Vec<u8>) -> Self {
//TOFIX     Self {
//TOFIX         rawkey: , //Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
//TOFIX         method: AES_256_CTR_PSK,
//TOFIX         prk: WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt)//AES_256_CTR_PSK;
//TOFIX     }
//TOFIX }

//TOFIX fn new_key_from_ecdh_str(raw_public_key: Vec<u8>, ecdh_key_pair: WfECDHKeyPair) -> Self {
//this(convertToByteArray(rawPublicKey), ecdhKeyPair); //<- how to rustify?
//TOFIX }

//TOFIX fn new_key_from_ecdh_vec(raw_public_key: Vec<u8>, ecdh_key_pair: WfECDHKeyPair) -> Self {
//TOFIX     Self {
//TOFIX         rawkey: , //ecdhKeyPair.negotiateKey(rawPublicKey);
//TOFIX         method:  AES_256_CTR_ECDH,
//TOFIX         prk: //WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
//TOFIX     }
//TOFIX }

//Constructs a new Whiteflag encryption key through ECDH key negotiation
//public WfEncryptionKey(final ECPublicKey ecPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
//    this.rawkey = ecdhKeyPair.negotiateKey(ecPublicKey);
//    this.method = AES_256_CTR_ECDH;
//    this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
//}

//TOFIX pub fn new_key_from_ecdh_key() -> Self {
//TOFIX     Self {
//TOFIX         rawkey: ,// need to rustify -> ecdhKeyPair.negotiateKey(ecPublicKey),
//TOFIX         method: WhiteflagEncryptionMethod.AES_256_CTR_ECDH,
//TOFIX         prk = // need to rustify -> WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
//TOFIX     }
//TOFIX }

//Returns the encryption method
//public final WfEncryptionMethod getEncryptionMethod() {
//    return method;
//}
//TOFIX fn get_encryption_method() -> WhiteflagEncryptionMethod {
//    self.method
//}
