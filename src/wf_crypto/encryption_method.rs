/*
 * Whiteflag Java Library
 */
package org.whiteflagprotocol.java.crypto;

/* Static import of cryptographic utility functions */
import static org.whiteflagprotocol.java.crypto.WfCryptoUtil.convertToByteArray;

/**
 * Whiteflag encryption parameters enum class
 *
 * <p> This is a non-instantiatable enum class that holds all
 * encryption parameters in accordance with the Whiteflag specification.
 * No implementation specific properties and methods are defined by this class.
 * 
 * <p> The names of cryptographic algorithms, modes, schemes used by this class
 * are in accordance with the
 * <a href="https://docs.oracle.com/en/java/javase/17/docs/specs/security/standard-names.html">Java Security Standard Algorithm Names</a>.
 * 
 * @wfver v1-draft.6
 * @wfref 5.2.3 Key and Token Derivation
 * 
 * @since 1.1
 */
@SuppressWarnings("java:S1192")
public enum WfEncryptionMethod {
    /**
     * Encryption Method 0: no encryption
     */
    NO_ENCRYPTION("0", "NONE", "NONE", "NoPadding", 0, ""),

    /**
     * Encryption Method 1: AES-256-CTR with negotiated key
     */
    AES_256_CTR_ECDH("1", "AES", "CTR", "NoPadding", 32, "8ddb03085a2c15e69c35c224bce2952dca7878770724741cbce5a135328be0c0"),

    /**
     * Encryption Method : AES-256-CTR with pre-shared key
     */
    AES_256_CTR_PSK("2", "AES", "CTR", "NoPadding", 32, "c4d028bd45c876135e80ef7889835822a6f19a31835557d5854d1334e8497b56");

    /* PROPERTIES */

    /* The valid regex charset of an unencoded field value */
    /**
     * The value used in a Whiteflag message to indicate the encryption method
     */
    public final String fieldValue;
    /**
     * The name of the algorithm for this encryption method, i.a.w. Java Security Standard Algorithm Names
     */
    protected final String algorithmName;
    /**
     * The mode of operation for this encryption method, i.a.w. Java Security Standard Algorithm Names
     */
    protected final String operationMode;
    /**
     * The padding scheme for this encryption method, i.a.w. Java Security Standard Algorithm Names
     */
    protected final String paddingScheme;
    /**
     * The cipher name for this encryption method i.a.w. Java Security Standard Algorithm Names
     */
    public final String cipherName;
    /**
     * The byte length of the encryption key for this encryption method
     */
    protected final int keyLength;
    /**
     * The salt used by this encryption method in the HKDF function to derive the encryption key
     */
    protected final byte[] hkdfSalt;

    /* METHODS */

    /* Constructor */
    /**
     * Sets the properties of the encryption methods
     * @param fieldValue the value used in the EncryptionIndicator message field to indicate the encryption method
     * @param algorithmName the name of the encryption algorithm, i.a.w. Java Security Standard Algorithm Names
     * @param operationMode the encryption mode of operation, i.a.w. Java Security Standard Algorithm Names
     * @param paddingScheme the padding scheme, i.a.w. Java Security Standard Algorithm Names
     * @param keyLength the length of the encryption key in bytes
     * @param hkdfSalt the salt used in the HKDF function to derive the encryption key
     */
    private WfEncryptionMethod(
        final String fieldValue,
        final String algorithmName,
        final String operationMode,
        final String paddingScheme,
        final int keyLength,
        final String hkdfSalt
    ) {
        this.fieldValue = fieldValue;
        this.algorithmName = algorithmName;
        this.operationMode = operationMode;
        this.paddingScheme = paddingScheme;
        this.cipherName = algorithmName + "/" + operationMode + "/" + paddingScheme;
        this.keyLength = keyLength;
        this.hkdfSalt = convertToByteArray(hkdfSalt);
    }

    /* PUBLIC STATIC METHODS */

    /**
     * Returns the encryption method from the indicator value
     * @since 1.1
     * @param fieldValue the value used in the EncryptionIndicator message field to indicate the encryption method
     * @return the requested encryption method
     * @throws WfCryptoException if the encryption indicator is invalid
     */
    public static final WfEncryptionMethod fromFieldValue(final String fieldValue) throws WfCryptoException {
        if (fieldValue == null || fieldValue.isEmpty()) {
            throw new IllegalArgumentException("Field value is null or empty");
        }
        for (WfEncryptionMethod method : values()) {
            if (method.fieldValue.equalsIgnoreCase(fieldValue)) return method;
        }
        throw new WfCryptoException("Invalid encryption method: " + fieldValue, null);
    }
}
