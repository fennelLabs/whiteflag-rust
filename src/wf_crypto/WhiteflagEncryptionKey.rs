use fennel_lib::aes_tools::AesCipher;

///This class represents a Whiteflag encryption key. Instances of this
///class represent the raw key, either pre-shared or negotiated, from which
///the actual key material for encryption methods 1 and 2 is created.
///
///Whiteflag Specification 5.2.3 Key and Token Derivation
///Whiteflag Specification 5.2.4 Message Encryption
struct WhiteflagEncryptionKey {
    /* Status of the instance */
    destroyed: bool,
    /* The encryption method and keys */
    /**
    ///The encryption method for which this key is valid
     */
    method: WhiteflagEncryptionMethod,
    /* The raw key materials */
    rawkey: T, //this may be String or Vec<u8>,
    prk: Vec<u8>,
}

trait WfEncryptionKey {
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

    pub fn new_key_from_raw_pre_shared_key_str(raw_pre_sharedKey: String) -> Self {
       //how to rustify? -> this(convertToByteArray(rawPreSharedKey)); 
    }

    ///Constructs a new Whiteflag encryption key from a raw pre-shared key
    //public WfEncryptionKey(final byte[] rawPreSharedKey) {
        //this.rawkey = Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
        //this.method = AES_256_CTR_PSK;
        //this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    //}
    pub fn new_key_from_raw_pre_shared_key_vec(raw_pre_sharedKey: Vec<u8>) -> Self {
        Self {
            rawkey: , //Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
            method: WhiteflagEncryptionMethod.AES_256_CTR_PSK, 
            prk: WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);//AES_256_CTR_PSK;
        }
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    public WfEncryptionKey(final String rawPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
        this(convertToByteArray(rawPublicKey), ecdhKeyPair);
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    //public WfEncryptionKey(final byte[] rawPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
    //    this.rawkey = ecdhKeyPair.negotiateKey(rawPublicKey);
    //    this.method = AES_256_CTR_ECDH;
    //    this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    //}
    pub fn new_key_from_ecdh(raw_public_key: Vec<u8>, WfECDHKeyPair ecdh_key_pair) -> {
        Self {
            rawkey: , //ecdhKeyPair.negotiateKey(rawPublicKey);
            method:  WhiteflagEncryptionMethod.AES_256_CTR_ECDH, //Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
            prk: //WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
        }
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    public WfEncryptionKey(final ECPublicKey ecPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
        this.rawkey = ecdhKeyPair.negotiateKey(ecPublicKey);
        this.method = AES_256_CTR_ECDH;
        this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    }
    ///Destroys this Whiteflag cipher by clearing the encryption key
    public final void destroy() {
        WfCryptoUtil.zeroise(rawkey);
        WfCryptoUtil.zeroise(prk);
        this.destroyed = true;
    }
    ///Determine if this Whiteflag cipher has been destroyed.
    //public final boolean isDestroyed() {
    //    return destroyed;
    //}
    pub fn isDestroyed(&self) -> boolean {
        self.destroyed
    }

    ///Returns the encryption method
    //public final WfEncryptionMethod getEncryptionMethod() {
    //    return method;
    //}
    fn get_encryption_method() -> WhiteflagEncryptionMethod {
        self.method
    }

    ///Derive the secret cryptographic key from this Whiteflag encryption key
    fn get_secret_key(context: &str) -> AesCipher  {
        checkDestroyed();
        AesCipher::create(
            WfCryptoUtil.hkdfExpand(prk, hex::decode(context).unwrap(), method.keyLength)
        )
    }
    ///Checks and throws exception if this encryption key has been destroyed
    private final void checkDestroyed() {
        if (destroyed) throw new IllegalStateException("Encryption key has been destroyed");
    }
}