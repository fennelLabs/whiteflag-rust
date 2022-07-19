use math::round;
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
    for elem in byteArray.iter_mut() {
        *elem = 0xFF;
    } // All ones.
    for elem in byteArray.iter_mut() {
        *elem = 0xAA;
    } // Alternating.
    for elem in byteArray.iter_mut() {
        *elem = 0x55;
    } // Alternating the other way.
    for elem in byteArray.iter_mut() {
        *elem = 0x00;
    } // Zero it all out.
}

/// Performs HKDF key and token derivation for Whiteflag
///
/// The HKDF function as defined in RFC 5869 to derive the tokens and
/// encryption keys used for Whiteflag. This function performs the full
/// HKDF expand and extract.
///
/// Whiteflag Specification 5.2.3 Key and Token Derivation
fn hkdf(ikm: Vec<u8>, salt: Vec<u8>, info: Vec<u8>, key_length: usize) -> Vec<u8> {
    /// Step 1. HKDF-Extract(salt, IKM) -> PRK
    /// Step 2. HKDF-Expand(PRK, info, L) -> OKM
    return hkdf_expand(hkdf_extract(ikm, salt), info, key_length);
}

/// Performs HKDF key and token derivation for Whiteflag
///
/// This is a wrapper for the HKDF function allowing to provide
/// the input as hexadecimal strings.
/// Note from Fennel Labs:
/// In whiteflag-java, this is an overload of hkdf. We've kept it for completeness,
/// but really it'd be preferable for this to be the public-facing interface.
pub fn hkdf_strings(ikm: &str, salt: &str, info: &str, key_length: usize) -> &str {
    /// Step 1. HKDF-Extract(salt, IKM) -> PRK
    /// Step 2. HKDF-Expand(PRK, info, L) -> OKM
    return hex::encode(hkdf(
        hex::decode(ikm).unwrap(),
        hex::decode(salt).unwrap(),
        hex::decode(info).unwrap(),
        key_length,
    ))
    .unwrap();
}

/// Performs RFC 5869 HKDF Step 1: extract
fn hkdf_extract(ikm: Vec<u8>, salt: Vec<u8>) {
    return get_hmac(salt).doFinal(ikm);
}

/// Performs RFC 5869 HKDF Step 2: expand
fn hkdf_expand(prk: Vec<u8>, info: Vec<u8>, keyLength: usize) -> Vec<u8> {
    // TODO we need something similar to ByteBuffer's behavior.
    let okm: ByteBuffer = ByteBuffer.allocate(keyLength);
    let remainder = keyLength;

    let hmac: Mac = get_hmac(prk); // Mac isn't a real type yet.
    let t: u8 = 0;
    let N = round::ceil((keyLength as f32) / (hmac.getMacLength() as f32)) as usize;

    for i in 1..N {
        hmac.update(t);
        hmac.update(info);
        hmac.update(i as u8);
        t = hmac.doFinal();

        // Home-baked min(x,y) function, just like ma used to make.
        let length = if remainder < t.length {
            remainder
        } else {
            t.length
        };

        okm.put(t, 0, length);
        remainder -= length;
    }
    return okm.array();
}

/// Creates a HMAC object initialised with the provide key
fn get_hmac(key: Vec<u8>) -> WhiteflagCryptoResult<Mac> {
    // Mac isn't a real type here yet.
    let hmac: Mac;
    hmac = Mac.getInstance(HKDF_HASHALG);
    hmac.init(SecretKeySpec::new(key, HKDF_HASHALG)); // For now, pretending this is a real type. We'll need to change this to our secret key.
    return hmac;
}
