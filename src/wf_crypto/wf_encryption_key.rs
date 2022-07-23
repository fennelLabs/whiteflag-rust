use fennel_lib::aes_tools::AESCipher;
use super::encryption_method::{WhiteflagEncryptionMethod};
use super::ecdh_keypair::{
    generate_wfkeypair, generate_wfkeypair_from_key, WfECDHKeyPair, WhiteflagECDHKeyPair,
};

///This class represents a Whiteflag encryption key. Instances of this
///class represent the raw key, either pre-shared or negotiated, from which
///the actual key material for encryption methods 1 and 2 is created.
///
///Whiteflag Specification 5.2.3 Key and Token Derivation
///Whiteflag Specification 5.2.4 Message Encryption
pub struct WhiteflagEncryptionKey {
    /* The encryption method and keys */
    /**
    ///The encryption method for which this key is valid
     */
    method: WhiteflagEncryptionMethod,
    /* The raw key materials */
    rawkey: <T>, //this may be String or Vec<u8>,
    prk: Vec<u8>,
}

pub trait WfEncryptionKey {

    fn new_key_from_raw_pre_shared_key_str(raw_pre_shared_key: String) -> WhiteflagEncryptionKey;

    fn new_key_from_raw_pre_shared_key_vec(raw_pre_shared_key: Vec<u8>) -> WhiteflagEncryptionKey;

    fn new_key_from_ecdh_vec(raw_public_key: Vec<u8>, ecdh_key_pair: WhiteflagECDHKeyPair) -> WhiteflagEncryptionKey;

    fn new_key_from_ecdh_str(raw_public_key: Vec<u8>, ecdh_key_pair: WhiteflagECDHKeyPair) -> WhiteflagEncryptionKey;

    fn get_encryption_method() -> WhiteflagEncryptionMethod;

    fn get_secret_key(context: &str) -> WhiteflagEncryptionMethod;

}

impl WfEncryptionKey for WhiteflagEncryptionKey {
    /**
    ///Constructs a new Whiteflag encryption key from a raw pre-shared key
    ///@param rawPreSharedKey a hexadecimal string with the raw pre-shared encryption key
     */
    //public WfEncryptionKey(final String rawPreSharedKey) {
        //this(convertToByteArray(rawPreSharedKey));
    //}

    fn new_key_from_raw_pre_shared_key_str(raw_pre_shared_key: String) -> Self {
       //how to rustify? -> this(convertToByteArray(rawPreSharedKey)); 
    }

    ///Constructs a new Whiteflag encryption key from a raw pre-shared key
    //public WfEncryptionKey(final byte[] rawPreSharedKey) {
        //this.rawkey = Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
        //this.method = AES_256_CTR_PSK;
        //this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    //}
    fn new_key_from_raw_pre_shared_key_vec(raw_pre_shared_key: Vec<u8>) -> Self {
        Self {
            rawkey: , //Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
            method: WhiteflagEncryptionMethod.AES_256_CTR_PSK, 
            prk: WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt)//AES_256_CTR_PSK;
        }
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    /*public WfEncryptionKey(final String rawPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
        this(convertToByteArray(rawPublicKey), ecdhKeyPair); //<- how to rustify?
    }*/
    fn new_key_from_ecdh_str(raw_public_key: Vec<u8>, ecdh_key_pair: WfECDHKeyPair) -> Self {
        //this(convertToByteArray(rawPublicKey), ecdhKeyPair); //<- how to rustify?
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    //public WfEncryptionKey(final byte[] rawPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
    //    this.rawkey = ecdhKeyPair.negotiateKey(rawPublicKey);
    //    this.method = AES_256_CTR_ECDH;
    //    this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    //}
    fn new_key_from_ecdh_vec(raw_public_key: Vec<u8>, ecdh_key_pair: WfECDHKeyPair) -> Self {
        Self {
            rawkey: , //ecdhKeyPair.negotiateKey(rawPublicKey);
            method:  WhiteflagEncryptionMethod.AES_256_CTR_ECDH, 
            prk: //WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
        }
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    //public WfEncryptionKey(final ECPublicKey ecPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
    //    this.rawkey = ecdhKeyPair.negotiateKey(ecPublicKey);
    //    this.method = AES_256_CTR_ECDH;
    //    this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    //}

    pub fn new_key_from_ecdh_key() -> Self {
        Self {
            rawkey: ,// need to rustify -> ecdhKeyPair.negotiateKey(ecPublicKey),
            method: WhiteflagEncryptionMethod.AES_256_CTR_ECDH,
            prk = // need to rustify -> WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
        }
    }

    ///Returns the encryption method
    //public final WfEncryptionMethod getEncryptionMethod() {
    //    return method;
    //}
    fn get_encryption_method() -> WhiteflagEncryptionMethod {
        self.method
    }

    ///Derive the secret cryptographic key from this Whiteflag encryption key
    fn get_secret_key(context: &str) -> AESCipher  {
        AESCipher::create(
            WfCryptoUtil.hkdfExpand(prk, hex::decode(context).unwrap(), method.keyLength)
        )
    }

}