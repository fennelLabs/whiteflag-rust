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
    method: WfEncryptionMethod,

    /* The raw key materials */
    rawkey: Vec<u8>,
    prk: Vec<u8>,
}

//rust builder pattern is recommended for situations where one needs multiple constructors

//https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
//https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
//https://www.ameyalokare.com/rust/2017/11/02/rust-builder-pattern.html


#[derive(Default)]
struct WhiteflagEncryptionKeyBuilder {
    rawkey: Vec<u8> //This may be different types. how to handle that?
    method: WfEncryptionMethod,
    prk: Vec<u8>,  
    destroyed: bool
    //not sure how to handle the this() lines in Java
}

impl WhiteflagEncryptionKeyBuilder {
    pub fn new(rawkey: Vec<u8>, method: WfEncryptionMethod, prk: Vec<u8>) -> WhiteflagEncryptionKeyBuilder {
        WhiteflagEncryptionKeyBuilder {
            rawkey,
            method,
            prk,
            destroyed,
            //additional
        }
    }
    //fn with_additional_optional_argument_name(mut self, additional_optional_argument_name: type) -> Self {
    //self.additional_optional_argument_name = Some(additional_optional_argument_name);
    //self
    //}

    fn build(self) -> WhiteflagEncryptionKey {
        WhiteflagEncryptionKey {
            rawkey: self.rawkey,
            method: self.method,
            prk: self.prk,
            destroyed: self.destroyed
            //additional_optional_argument_name: self.additional_optional_argument_name
        }
    }
}

impl WfEncryptionKey for WhiteflagEncryptionKey {

    pub fn builder() -> WhiteflagEncryptionKeyBuilder {
        WhiteflagEncryptionKeyBuilder::default()
    }


    /**
    ///Constructs a new Whiteflag encryption key from a raw pre-shared key
    ///@param rawPreSharedKey a hexadecimal string with the raw pre-shared encryption key
     */
    public WfEncryptionKey(final String rawPreSharedKey) {
        this(convertToByteArray(rawPreSharedKey));
    }

    ///Constructs a new Whiteflag encryption key from a raw pre-shared key
    public WfEncryptionKey(final byte[] rawPreSharedKey) {
        this.rawkey = Arrays.copyOf(rawPreSharedKey, rawPreSharedKey.length);
        this.method = AES_256_CTR_PSK;
        this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    public WfEncryptionKey(final String rawPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
        this(convertToByteArray(rawPublicKey), ecdhKeyPair);
    }

    ///Constructs a new Whiteflag encryption key through ECDH key negotiation
    public WfEncryptionKey(final byte[] rawPublicKey, final WfECDHKeyPair ecdhKeyPair) throws WfCryptoException {
        this.rawkey = ecdhKeyPair.negotiateKey(rawPublicKey);
        this.method = AES_256_CTR_ECDH;
        this.prk = WfCryptoUtil.hkdfExtract(rawkey, method.hkdfSalt);
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
    public final boolean isDestroyed() {
        return destroyed;
    }

    ///Returns the encryption method
    //public final WfEncryptionMethod getEncryptionMethod() {
    //    return method;
    //}
    fn get_encryption_method() -> WfEncryptionMethod {
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
