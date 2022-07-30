use fennel_lib::aes_tools::AESCipher;
//use super::encryption_method::{WhiteflagEncryptionMethod::*};
use super::ecdh_keypair::{
    generate_wfkeypair, generate_wfkeypair_from_key, WfECDHKeyPair, WhiteflagECDHKeyPair,
};
use super::encryption_method::{encryption_method_from_field_value, WhiteflagEncryptionMethod};

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
    rawkey: Vec<u8>,
}

pub trait WfEncryptionKey {
    fn new(raw_pre_shared_key: String) -> Self;
    fn new_key_from_ecdh_key(raw_public_key: String, ecdh_key_pair: WhiteflagECDHKeyPair) -> Self;

    //TOFIX fn new_key_from_raw_pre_shared_key_str(raw_pre_shared_key: String) -> WhiteflagEncryptionKey;

    //TOFIX fn new_key_from_raw_pre_shared_key_vec(raw_pre_shared_key: Vec<u8>) -> WhiteflagEncryptionKey;

    //TOFIX fn new_key_from_ecdh_vec(raw_public_key: Vec<u8>, ecdh_key_pair: WhiteflagECDHKeyPair) -> WhiteflagEncryptionKey;

    //TOFIX fn new_key_from_ecdh_str(raw_public_key: Vec<u8>, ecdh_key_pair: WhiteflagECDHKeyPair) -> WhiteflagEncryptionKey;

    //TOFIX fn get_encryption_method() -> WhiteflagEncryptionMethod;

    fn get_secret_key(&self) -> Vec<u8>;
}

impl WfEncryptionKey for WhiteflagEncryptionKey {
    /// Constructs a new Whiteflag encryption key from a raw pre-shared key
    /// @param rawPreSharedKey a hexadecimal string with the raw pre-shared encryption key
    fn new(raw_pre_shared_key: String) -> Self {
        WhiteflagEncryptionKey {
            rawkey: hex::decode(raw_pre_shared_key).unwrap(),
            method: encryption_method_from_field_value("4".to_string()).unwrap(),
        }
    }

    //TOFIX
    /* fn new_key_from_raw_pre_shared_key_str(raw_pre_shared_key: String) -> Self {
       how to rustify? -> this(convertToByteArray(rawPreSharedKey));
    }*/

    ///Constructs a new Whiteflag encryption key from a raw pre-shared key
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

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    fn new_key_from_ecdh_key(
        raw_public_key: String,
        mut ecdh_key_pair: WhiteflagECDHKeyPair,
    ) -> Self {
        WhiteflagEncryptionKey {
            rawkey: ecdh_key_pair
                .negotiate_key_from_bytes(hex::decode(raw_public_key).unwrap().try_into().unwrap()),
            method: encryption_method_from_field_value("3".to_string()).unwrap(),
        }
    }
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

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
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

    ///Returns the encryption method
    //public final WfEncryptionMethod getEncryptionMethod() {
    //    return method;
    //}
    //TOFIX fn get_encryption_method() -> WhiteflagEncryptionMethod {
    //    self.method
    //}

    ///Derive the secret cryptographic key from this Whiteflag encryption key
    fn get_secret_key(&self) -> Vec<u8> {
        self.rawkey.clone()
    }
}
