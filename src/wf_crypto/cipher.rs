struct WhiteflagCipher {
    cipher: Cipher,
    key: WfEncryptionKey,
    secretKey: SecretKey,
    iv: Vec<u8>,
    context: Vec<u8>,
}

trait WfCipher {}

impl WfCipher for WhiteflagCipher {
    fn new(key: WfEncryptionKey) -> WfCipher {
        this.key = key;
        try {
            this.cipher = Cipher.getInstance(key.method.cipherName);
        } catch(Exception e) {
            throw new WfCryptoException("Could not instantiate cryptographic cipher: " + key.method.cipherName, e);
        }
    }

    fn fromKey(key: WfEncryptionKey) -> WfCipher {
        return WfCipher::new(key);
    }

    fn setContext(context: String) -> WfCipher {
        return setContext(hex::decode(context).unwrap());
    }

    fn setContext(context: Vec<u8>) -> WfCipher {
        this.context = context;
        this.secretKey = key.getSecretKey(context);
        return this;
    }

    fn setInitVector() -> Vec<u8> {
        let initialisationVector = [u8; 32];
        try {
            SecureRandom.getInstanceStrong().nextBytes(initialisationVector);
            this.iv = new IvParameterSpec(initialisationVector);
        } catch (Exception e) {
            throw new WfCryptoException("Could not generate new random initialisation vector", e);
        }
        return initialisationVector;
    }

    fn setInitVector(initialisation_vector: String) -> WfCipher {
        return setInitVector(convertToByteArray(initialisationVector));
    }

    fn setInitVector(initialisationVector: Vec<u8>) -> WfCipher {
        this.iv = new IvParameterSpec(initialisationVector, 0, IVBYTELENGTH);
        return this;
    }

    fn getInitVector() -> Vec<u8> {
        if (iv == null) return new byte[0];
        return iv.getIV();
    }

    fn is_set() -> boolean {
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

    fn checkSet(&self) {
        if (Boolean.FALSE.equals(self.is_set())) {
            throw new IllegalStateException("Context and/or initialisation vector have not been set");
        }
    }
}
