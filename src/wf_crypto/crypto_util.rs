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
    pub fn hkdf(ikm: Vec<u8>, salt: Vec<u8>, info: Vec<u8>, key_length: usize) {
        /// Step 1. HKDF-Extract(salt, IKM) -> PRK
        /// Step 2. HKDF-Expand(PRK, info, L) -> OKM
        return hkdfExpand(hkdfExtract(ikm, salt), info, key_length);
    }

    
    /// Performs HKDF key and token derivation for Whiteflag
    /// 
    /// This is a wrapper for the HKDF function allowing to provide
    /// the input as hexadecimal strings.
    pub fn hkdf(ikm: &str, salt: &str, info: &str, key_length: usize) {
        /// Step 1. HKDF-Extract(salt, IKM) -> PRK
        /// Step 2. HKDF-Expand(PRK, info, L) -> OKM
        return hex::encode(hkdf(
            hex::decode(ikm).unwrap(),
            hex::decode(salt).unwrap(),
            hex::decode(info).unwrap(),
            key_length
        )).unwrap();
    }
    
    /// Performs RFC 5869 HKDF Step 1: extract
    fn hkdfExtract(ikm: Vec<u8>, salt: Vec<u8>) {
        return getHMAC(salt).doFinal(ikm);
    }
    
    /// Performs RFC 5869 HKDF Step 2: expand
    fn hkdfExpand(prk: Vec<u8>, info: Vec<u8>, keyLength: usize) -> Vec<u8> {
        ByteBuffer okm = ByteBuffer.allocate(keyLength);
        let remainder = keyLength;

        Mac hmac = getHMAC(prk);
        let t: u8 = 0;
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
    fn getHMAC(key: Vec<u8>) {
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
