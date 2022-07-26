struct WhiteflagCipher {}

trait WfCipher {}

impl WfCipher for WhiteflagCipher {
    pub IVBYTELENGTH: usize = 16;

    destroyed: boolean = false;

    Cipher cipher;
    WfEncryptionKey key; 
    SecretKey secretKey;
    IvParameterSpec iv;
    byte[] context;

    fn new(WfEncryptionKey key) {
        this.key = key;
        try {
            this.cipher = Cipher.getInstance(key.method.cipherName);
        } catch(Exception e) {
            throw new WfCryptoException("Could not instantiate cryptographic cipher: " + key.method.cipherName, e);
        }
    }

    fn WfCipher fromKey(WfEncryptionKey key) {
        if (Boolean.TRUE.equals(key.isDestroyed())) {
            throw new IllegalArgumentException("Cannot create Whiteflag cipher from a destroyed key");
        }
        return new WfCipher(key);
    }

    fn WfCipher setContext(String context) {
        return setContext(convertToByteArray(context));
    }

    fn WfCipher setContext(context: Vec<u8>) {
        this.context = context;
        this.secretKey = key.getSecretKey(context);
        return this;
    }

    fn setInitVector() -> Vec<u8> {
        byte[] initialisationVector = new byte[IVBYTELENGTH];
        try {
            SecureRandom.getInstanceStrong().nextBytes(initialisationVector);
            this.iv = new IvParameterSpec(initialisationVector);
        } catch (Exception e) {
            throw new WfCryptoException("Could not generate new random initialisation vector", e);
        }
        return initialisationVector;
    }

    fn WfCipher setInitVector(String initialisationVector) {
        return setInitVector(convertToByteArray(initialisationVector));
    }

    fn WfCipher setInitVector(initialisationVector: Vec<u8>) {
        checkDestroyed();
        this.iv = new IvParameterSpec(initialisationVector, 0, IVBYTELENGTH);
        return this;
    }

    fn getInitVector() -> Vec<u8> {
        if (iv == null) return new byte[0];
        return iv.getIV();
    }

    fn isSet() -> boolean {
        if (context == null || context.length == 0) return false;
        if (iv == null || iv.getIV().length != IVBYTELENGTH) return false;
        return !this.destroyed;
    }

    fn encrypt(data: String) -> String {
        return convertToHexString(encrypt(convertToByteArray(data)));
    }

    fn encrypt_bytes(data: Vec<u8>) -> Vec<u8> {
        checkSet();
        try {
            cipher.init(Cipher.ENCRYPT_MODE, secretKey, iv);
            return cipher.doFinal(data);
        } catch(Exception e) {
            throw new WfCryptoException("Could not encrypt data with " + key.method.cipherName + " cipher", e);
        }
    }

    fn decrypt(data: String) -> String {
        return convertToHexString(decrypt(convertToByteArray(data)));
    }

    fn decrypt_bytes(data: Vec<u8>) -> Vec<u8> {
        checkSet();
        try {
            cipher.init(Cipher.DECRYPT_MODE, secretKey, iv);
            return cipher.doFinal(data);
        } catch(Exception e) {
            throw new WfCryptoException("Could not decrypt data with " + key.method.cipherName + " cipher", e);
        }
    }

    fn getContext() -> Vec<u8> {
        return Arrays.copyOf(context, context.length);
    }

    fn checkSet() {
        if (Boolean.FALSE.equals(isSet())) {
            throw new IllegalStateException("Context and/or initialisation vector have not been set");
        }
    }
}
