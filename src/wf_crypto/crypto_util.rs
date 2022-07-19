use regex::Regex;

    /// The hash algorithm for the HKDF function
    const HKDF_HASHALG: String = "HMACSHA256";
    
    /// The regex pattern describing a valid hexadecimnal string
    const HEXPATTERN: Regex = Regex::new("^[a-fA-F0-9]*$").unwrap();
    
    /// The "0x" prefix of a hexadecimal string
    const HEXPREFIX: &str = "0x";
    
    /// The radix of a hexadecimal digit
    const HEXRADIX: usize = 16;
    
    /// The bit size of a quadbit
    const QUADBITL: usize = 4;

    /// Zeroises a byte array
    pub fn zeroise(byteArray: Vec<u8>) {
        for elem in byteArray.iter_mut() { *elem = 0xFF; } // All ones.
        for elem in byteArray.iter_mut() { *elem = 0xAA; } // Alternating.
        for elem in byteArray.iter_mut() { *elem = 0x55; } // Alternating the other way.
        for elem in byteArray.iter_mut() { *elem = 0x00; } // Zero it all out.
    }
    
    /// Performs HKDF key and token derivation for Whiteflag
    /// 
    /// The HKDF function as defined in RFC 5869 to derive the tokens and
    /// encryption keys used for Whiteflag. This function performs the full
    /// HKDF expand and extract.
    /// 
    /// Whiteflag Specification 5.2.3 Key and Token Derivation
    pub fn hkdf(byte[] ikm, byte[] salt, byte[] info, int keyLength) {
        /// Step 1. HKDF-Extract(salt, IKM) -> PRK
        /// Step 2. HKDF-Expand(PRK, info, L) -> OKM
        return hkdfExpand(hkdfExtract(ikm, salt), info, keyLength);
    }

    
    /// Performs HKDF key and token derivation for Whiteflag
    /// 
    /// This is a wrapper for the HKDF function allowing to provide
    /// the input as hexadecimal strings.
    pub fn hkdf(String ikm, String salt, String info, int keyLength) {
        /// Step 1. HKDF-Extract(salt, IKM) -> PRK
        /// Step 2. HKDF-Expand(PRK, info, L) -> OKM
        return convertToHexString(hkdf(
            convertToByteArray(ikm),
            convertToByteArray(salt),
            convertToByteArray(info),
            keyLength
        ));
    }
    
    /// Performs RFC 5869 HKDF Step 1: extract
    fn hkdfExtract(byte[] ikm, byte[] salt) {
        return getHMAC(salt).doFinal(ikm);
    }
    
    /// Performs RFC 5869 HKDF Step 2: expand
    fn hkdfExpand(byte[] prk, byte[] info, int keyLength) {
        ByteBuffer okm = ByteBuffer.allocate(keyLength);
        int remainder = keyLength;

        Mac hmac = getHMAC(prk);
        byte[] t = new byte[0];
        int N = (int) Math.ceil((double) keyLength / (double) hmac.getMacLength());

        for (int i = 1; i <= N; i++) {
            hmac.update(t);
            hmac.update(info);
            hmac.update((byte) i);
            t = hmac.doFinal();

            int length = Math.min(remainder, t.length);
            okm.put(t, 0, length);
            remainder -= length;
        }
        return okm.array();
    }

    /// Creates a HMAC object initialised with the provide key
    private fn getHMAC(byte[] key) {
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

    
    /// Checks for and removes prefix from string
    private fn removeStringPrefix(String str, String prefix) {
        if (str.startsWith(prefix)) return str.substring(prefix.length());
        return str;
    }
}
