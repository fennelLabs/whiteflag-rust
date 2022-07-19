/**
 * Whiteflag cryptographic utility class
 *
 * <p> This is a non-instantiatable utility class that performs
 * cryptographic support functions. No implementation specific
 * properties and methods are defined by this class.
 * 
 * @since 1.1
 */
struct CryptoUtil {

    /* PROPERTIES */

    /* Constants */
    /**
     * The hash algorithm for the HKDF function
     */
    pub HKDF_HASHALG: String,
    /**
     * The regex pattern describing a valid hexadecimnal string
     */
    public static final Pattern HEXPATTERN = Pattern.compile("^[a-fA-F0-9]*$");
    /**
     * The "0x" prefix of a hexadecimal string
     */
    pub HEXPREFIX: &str,
    /**
     * The radix of a hexadecimal digit
     */
    pub HEXRADIX: usize,
    /**
     * The bit size of a quadbit
     */
    pub QUADBITL: usize,
}

impl WfCryptoUtil for CryptoUtil {
    /** 
     * Prevents the utility class to be instantiated
     */
    private WfCryptoUtil() {
        throw new IllegalStateException("Cannot instantiate Whiteflag cryptographic utility class");
    }

    /* PUBLIC STATIC METHODS */

    /**
     * Zeroises a byte array
     * @param byteArray the byte array to be zeroised
     */
    public static final void zeroise(byte[] byteArray) {
        Arrays.fill(byteArray, (byte) 0xFF);    // 1111 1111
        Arrays.fill(byteArray, (byte) 0xAA);    // 1010 1010
        Arrays.fill(byteArray, (byte) 0x55);    // 0101 0101
        Arrays.fill(byteArray, (byte) 0x00);    // 0000 0000
    }

    /**
     * Converts a hexadecimal string to a byte array
     * @param hexstr the hexadecimal string
     * @return a byte array
     * @throws IllegalArgumentException if argument is not a hexadecimal string
     */
    public static final byte[] convertToByteArray(final String hexstr) {
        /* Prepare hexadecimal input string by removing prefix, checking characters and adding trailing 0 */
        if (hexstr == null) {
            throw new IllegalArgumentException("Null is not a valid hexadecimal string");
        }
        String str = removeStringPrefix(hexstr, HEXPREFIX);
        if (str.length() % 2 == 1) str = str + "0";
        if (!HEXPATTERN.matcher(str).matches()) {
            throw new IllegalArgumentException("Invalid hexadecimal string");
        }
        /* Loop through hexadecimal string and take two chars at a time*/
        final int strLength = str.length();
        byte[] byteArray = new byte[strLength / 2];
        for (int i = 0; i < strLength; i += 2) {
            byteArray[i / 2] = (byte) ((Character.digit(str.charAt(i), HEXRADIX) << QUADBIT)
                                      + Character.digit(str.charAt(i + 1), HEXRADIX));
        }
        return byteArray;
    }

    /**
     * Converts a byte array to a hexadecimal string
     * @param byteArray the byte array
     * @return a hexadecimal string
     */
    public static final String convertToHexString(final byte[] byteArray) {
        StringBuilder hexstr = new StringBuilder();
        for (int byteIndex = 0; byteIndex < byteArray.length; byteIndex++) {
            char[] hexDigits = new char[2];
            hexDigits[0] = Character.forDigit((byteArray[byteIndex] >> QUADBIT) & 0xF, HEXRADIX);
            hexDigits[1] = Character.forDigit((byteArray[byteIndex] & 0xF), HEXRADIX);
            hexstr.append(new String(hexDigits));
        }
        return hexstr.toString().toLowerCase();
    }

    /**
     * Performs HKDF key and token derivation for Whiteflag
     * 
     * <p> The HKDF function as defined in RFC 5869 to derive the tokens and
     * encryption keys used for Whiteflag. This function performs the full
     * HKDF expand and extract.
     * 
     * @wfver v1-draft.6
     * @wfref 5.2.3 Key and Token Derivation
     * 
     * @param ikm byte array with the input key material
     * @param salt byte array with the cryptographic salt
     * @param info byte array with information to bind the derived key to an intended context
     * @param keyLength integer with the output key length in bytes
     * @return the output key material, i.e. the generated secret cryptographic key
     */
    public static final byte[] hkdf(final byte[] ikm, final byte[] salt, final byte[] info, final int keyLength) {
        /*
         * Step 1. HKDF-Extract(salt, IKM) -> PRK
         * Step 2. HKDF-Expand(PRK, info, L) -> OKM
         */
        return hkdfExpand(hkdfExtract(ikm, salt), info, keyLength);
    }

    /**
     * Performs HKDF key and token derivation for Whiteflag
     * 
     * <p> This is a wrapper for the HKDF function allowing to provide
     * the input as hexadecimal strings.
     * @param ikm hexadecimal string with the input key material
     * @param salt hexadecimal string the cryptographic salt
     * @param info hexadecimal string information to bind the derived key to an intended context
     * @param keyLength integer with the output key length in bytes
     * @return the output key material, i.e. the generated secret cryptographic key
     */
    public static final String hkdf(final String ikm, final String salt, final String info, final int keyLength) {
        /*
         * Step 1. HKDF-Extract(salt, IKM) -> PRK
         * Step 2. HKDF-Expand(PRK, info, L) -> OKM
         */
        return convertToHexString(hkdf(
            convertToByteArray(ikm),
            convertToByteArray(salt),
            convertToByteArray(info),
            keyLength
        ));
    }

    /* PROTECTED STATIC METHODS */

    /**
     * Performs RFC 5869 HKDF Step 1: extract
     * @param ikm the input key material
     * @param salt the cryptographic salt
     * @return an intermediate pseudo random key
     */
    protected static final byte[] hkdfExtract(final byte[] ikm, final byte[] salt) {
        return getHMAC(salt).doFinal(ikm);
    }

    /**
     * Performs RFC 5869 HKDF Step 2: expand
     * @param prk the intermediate pseudo random key
     * @param info information to bind the derived key to an intended context
     * @param keyLength the output key length in bytes
     * @return the output key material
     */
    protected static final byte[] hkdfExpand(final byte[] prk, final byte[] info, final int keyLength) {
        /* Prepare output */
        ByteBuffer okm = ByteBuffer.allocate(keyLength);
        int remainder = keyLength;

        /* Prepare hashing function */
        Mac hmac = getHMAC(prk);
        byte[] t = new byte[0];
        final int N = (int) Math.ceil((double) keyLength / (double) hmac.getMacLength());

        /* Interations to calculate okm */
        for (int i = 1; i <= N; i++) {
            /* Concatinate and hash previous hash T, info and counter i */
            hmac.update(t);
            hmac.update(info);
            hmac.update((byte) i);
            t = hmac.doFinal();

            /* Add hash to (remainder of) okm buffer */
            final int length = Math.min(remainder, t.length);
            okm.put(t, 0, length);
            remainder -= length;
        }
        return okm.array();
    }

    /* PRIVATE STATIC METHODS */

    /**
     * Creates a HMAC object initialised with the provide key
     * @param key the key to initialize the HMAC object
     * @return an initialised HMAC object
     */
    private static final Mac getHMAC(byte[] key) {
        Mac hmac;
        try {
            hmac = Mac.getInstance(HKDF_HASHALG);
            hmac.init(new SecretKeySpec(key, HKDF_HASHALG));
        } catch(NoSuchAlgorithmException e) {
            throw new IllegalArgumentException("Invalid hash algorithm " + HKDF_HASHALG + " for HMAC function: " + e.getMessage());
        } catch(InvalidKeyException e) {
            throw new IllegalArgumentException("Invalid keying material for HMAC function: " + e.getMessage());
        }
        return hmac;
    }

    /**
     * Checks for and removes prefix from string
     * @param str string to be checked
     * @param prefix the prefix to be checked for
     * @return a string without prefix
     */
    private static final String removeStringPrefix(final String str, final String prefix) {
        if (str.startsWith(prefix)) return str.substring(prefix.length());
        return str;
    }
}
