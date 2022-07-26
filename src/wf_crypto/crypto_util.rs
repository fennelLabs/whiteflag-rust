/// Zeroises a byte array
pub fn zeroise(byteArray: &mut [u8]) {
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

/// Performs RFC 5869 HKDF Step 1: extract
pub fn hkdf_extract(ikm: &[u8], salt: &[u8]) -> Vec<u8> {
    hkdf::Hkdf::<sha2::Sha256>::extract(Some(salt), ikm)
        .0
        .to_vec()
}

/// Performs RFC 5869 HKDF Step 2: expand
pub fn hkdf_expand(prk: &[u8], info: &[u8], key_length: usize) -> Result<Vec<u8>, ()> {
    let hk = hkdf::Hkdf::<sha2::Sha256>::from_prk(prk).map_err(|e| ())?;
    let mut okm: Vec<u8> = vec![0; key_length];
    hk.expand(info, &mut okm).map_err(|e| ())?;
    Ok(okm)
}
