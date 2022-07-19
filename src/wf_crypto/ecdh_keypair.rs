/**
 * Whiteflag ECDH Key Pair class
 *
 * This class represents an Elleptic Curve Diffie-Hellmann key pair
 * used by Whiteflag for cryptographic key negotiation. The elliptic curve
 * parameters that must be used for Whiteflag are defined by the
 * brainpoolP256r1 curve as specified in RFC 5639. Public keys are shared
 * as raw 264-bit compressed public ECDH keys.
 * 
 * @wfver v1-draft.6
 * Whiteflag Specification 5.2.2 Key Agreement

 */
public final class WfECDHKeyPair implements Destroyable {
    /**
     * The name of the elleptic curve used by Whiteflag for ECDH hey negotiation
     * Whiteflag Specification 5.2.2 Key Agreement
     */
    public static final String CURVENAME = "brainpoolP256R1";

    /* Cryptographic parameters */
    private static final String ALGORITHM = "ECDH";
    private static final String PROVIDER = "BC";
    private static final int PUBKEYLENGTH = 33;

    /* Status of the instance */
    private boolean destroyed = false;

    /* Main key pair properties */
    private KeyPair keypair;
}

impl WhiteflagECDHKeyPair {
    /**
     * Constructs a new Whiteflag ECDH key pair
     * @throws WfCryptoException if the key pair could not be created
     */
    public WfECDHKeyPair() throws WfCryptoException {
        this.keypair = createKeyPair();
    }

    /**
     * Constructs a new Whiteflag ECDH key pair from an existing private key
     * @param ecPrivateKey the private key object
     * @throws WfCryptoException if the private key is invalid or the key pair could not be created
     */
    public WfECDHKeyPair(final ECPrivateKey ecPrivateKey) throws WfCryptoException {
        this.keypair = createKeyPair(ecPrivateKey);
    }

    /**
     * Destroys this Whiteflag ECDH key pair by clearing the private key
     * @throws DestroyFailedException if the destroy operation fails
     * @throws IllegalStateException if the encryption key has already been destroyed
     */
    @Override
    public final void destroy() throws DestroyFailedException {
        keypair.getPrivate().destroy();    // Destroy derived key; throws exceptions
        this.destroyed = true;
    }

    /**
     * Determine if this Whiteflag cipher has been destroyed.
     * @return TRUE if destroyed, else FALSE
     */
    @Override
    public final boolean isDestroyed() {
        return destroyed;
    }

    /**
     * Returns the public key of this key pair
     * @return a public key object
     * @throws IllegalStateException if the key pair has been destroyed
     */
    public final ECPublicKey getPublicKey() {
        checkDestroyed();
        return (ECPublicKey) keypair.getPublic();
    }

    /**
     * Returns the raw public key of the ECDH key pair
     * @return a byte array with the raw 264-bit compressed public ECDH key
     * @throws IllegalStateException if the key pair has been destroyed
     */
    public final byte[] getRawPublicKey() {
        checkDestroyed();
        return compressPublicKey(getPublicKey());
    }

    /**
     * Calculates the negotiated shared key with an originator
     * @param rawPublicKey the originator's raw 264-bit compressed public ECDH key
     * @return a byte array with the negotiated secret key
     * @throws WfCryptoException if the raw key or any of the parameters is invalid
     * @throws IllegalStateException if the key pair has been destroyed
     */
    public final byte[] negotiateKey(final byte[] rawPublicKey) throws WfCryptoException {
        return negotiateKey(createPublicKey(rawPublicKey));
    }

    /**
     * Calculates the negotiated shared key with an originator
     * @param ecPublicKey the originator's ECDH public key
     * @return a byte array with the negotiated secret key
     * @throws WfCryptoException if the raw key or any of the parameters is invalid
     * @throws IllegalStateException if the key pair has been destroyed
     */
    public final byte[] negotiateKey(final ECPublicKey ecPublicKey) throws WfCryptoException {
        checkDestroyed();
        try {
            KeyAgreement ka = KeyAgreement.getInstance(ALGORITHM, PROVIDER);
            ka.init(keypair.getPrivate());
            ka.doPhase(ecPublicKey, true);
            return ka.generateSecret();
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not generate negotiated key from ECDH public key", e);
        }
    }

    /**
     * Creates a new random ECDH key pair with the curve specified for Whiteflag key negotiation
     * @return a key pair object
     * @throws WfCryptoException if the new ECDH key pair could not be created
     */
    public static final KeyPair createKeyPair() throws WfCryptoException {
        try {
            KeyPairGenerator kpg = KeyPairGenerator.getInstance(ALGORITHM, PROVIDER);
            kpg.initialize(ecParamSpec);
            return kpg.generateKeyPair();
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not generate new ECDH key pair", e);
        }
    }

    /**
     * Creates an ECDH key pair from an existing private key with the curve specified for Whiteflag key negotiation
     * @param ecPrivateKey the ECDH private key object
     * @return a key pair object
     * @throws WfCryptoException if an ECDH key pair could not be generated from the provided private key
     */
    public static final KeyPair createKeyPair(final ECPrivateKey ecPrivateKey) throws WfCryptoException {
        try {
            KeyFactory kf = KeyFactory.getInstance(ALGORITHM, PROVIDER);
            ECPoint point = ecParamSpec.getG().multiply(ecPrivateKey.getS());
            ECPublicKeySpec ecPubkeySpec = getPublicKeySpec(point.getEncoded(false));
            return new KeyPair(kf.generatePublic(ecPubkeySpec), ecPrivateKey);
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not generate ECDH key pair from existing private key", e);
        }
    }

    /**
     * Creates an ECDH public key object from a byte array
     * @param rawPublicKey a string with the raw 264-bit compressed public ECDH key
     * @return an ECDH public key object
     * @throws WfCryptoException if the raw key or any of the curve parameters is invalid
     */
    public static final ECPublicKey createPublicKey(final String rawPublicKey) throws WfCryptoException {
        return createPublicKey(convertToByteArray(rawPublicKey));
    }

    /**
     * Creates an ECDH public key object from a byte array
     * @param rawPublicKey a byte array with the raw 264-bit compressed public ECDH key
     * @return an ECDH public key object
     * @throws WfCryptoException if the raw key or any of the curve parameters is invalid
     */
	public static final ECPublicKey createPublicKey(final byte[] rawPublicKey) throws WfCryptoException {
        try {
            KeyFactory kf = KeyFactory.getInstance(ALGORITHM, PROVIDER);
            ECPublicKeySpec ecPubkeySpec = getPublicKeySpec(rawPublicKey);
            return (ECPublicKey) kf.generatePublic(ecPubkeySpec);
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not create ECDH public key from raw public key", e);
        }
	}

    /**
     * Creates an ECDH private key object from a byte array
     * @param rawPrivateKey a byte array with the raw private ECDH key
     * @return an ECDH private key object
     * @throws WfCryptoException if the raw key or any of the curve parameters is invalid
     */
    public static final ECPrivateKey createPrivateKey(final byte[] rawPrivateKey) throws WfCryptoException {
        try {
            KeyFactory kf = KeyFactory.getInstance(ALGORITHM, PROVIDER);
            ECPrivateKeySpec ecPrivkeySpec = new ECPrivateKeySpec(new BigInteger(rawPrivateKey), ecParamSpec);
            return (ECPrivateKey) kf.generatePrivate(ecPrivkeySpec);
        } catch (GeneralSecurityException e) {
            throw new WfCryptoException("Could not create ECDH private key from raw private key", e);
        }
    }

    /**
     * Compresses an ECDH public key to a raw 264-bit compressed public ECDH key
     * @param ecPublicKey an ECDH public key object
     * @return a byte array with the raw 264-bit compressed public ECDH key
     */
    public static final byte[] compressPublicKey(final ECPublicKey ecPublicKey) {
        /* Get coordinates of public key */
        final BigInteger y = ecPublicKey.getW().getAffineY();
        final BigInteger x = ecPublicKey.getW().getAffineX();

        /* Copy x-coordinate into byte array */
        byte[] compressedPubkey = new byte[PUBKEYLENGTH];
        final byte[] xBytes = x.toByteArray();
        final int startByte = compressedPubkey.length - xBytes.length;
        System.arraycopy(xBytes, 0, compressedPubkey, startByte, xBytes.length);

        /* Set first byte of compressed key and return compressed key */
        if (y.testBit(0)) compressedPubkey[0] = 0x03;   // y is odd
            else compressedPubkey[0] = 0x02;            // y is even
        return compressedPubkey;
    }

    /**
     * Checks and throws exception if this key pair has been destroyed
     * @throws IllegalStateException if this key pair has been destroyed
     */
    private final void checkDestroyed() {
        if (destroyed) throw new IllegalStateException("Key pair has been destroyed");
    }

    /**
     * Calculates the point on the curve and returns public key specification
     * @param coordinates a byte array with the ASN.1 encoded coordinates
     * @return the public key specification
     */
    private static final ECPublicKeySpec getPublicKeySpec(final byte[] coordinates) {
        ECPoint point = curve.decodePoint(coordinates);
        return new ECPublicKeySpec(point, ecParamSpec);
    }
}
