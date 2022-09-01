use super::{CryptoError, CryptoResult};
use hkdf::hmac::Hmac;

/// Zeroises a byte array
pub fn zeroise(byte_array: &mut [u8]) {
    for elem in byte_array.iter_mut() {
        *elem = 0xFF;
    } // All ones.
    for elem in byte_array.iter_mut() {
        *elem = 0xAA;
    } // Alternating.
    for elem in byte_array.iter_mut() {
        *elem = 0x55;
    } // Alternating the other way.
    for elem in byte_array.iter_mut() {
        *elem = 0x00;
    } // Zero it all out.
}

#[derive(Clone)]
pub struct Hkdf {
    hkdf: SimpleWhiteflagHkdf<sha2::Sha256>,
}

impl Hkdf {
    pub fn extract(ikm: &[u8], salt: &[u8]) -> Self {
        Hkdf {
            hkdf: SimpleWhiteflagHkdf::<sha2::Sha256>::new(ikm, salt),
        }
    }

    pub fn expand(&self, info: &[u8], key_length: usize) -> CryptoResult<Vec<u8>> {
        self.hkdf.expand(info, key_length)
    }
}

pub type SimpleWhiteflagHkdf<H> = WhiteflagHkdf<H, Hmac<H>>;

#[derive(Clone)]
pub struct WhiteflagHkdf<H, I>
where
    H: hkdf::hmac::digest::OutputSizeUser,
    I: hkdf::HmacImpl<H>,
{
    hk: hkdf::Hkdf<H, I>,
    prk: Vec<u8>,
}

impl<H, I> WhiteflagHkdf<H, I>
where
    H: hkdf::hmac::digest::OutputSizeUser,
    I: hkdf::HmacImpl<H>,
{
    /// Performs RFC 5869 HKDF Step 1: extract
    pub fn new(ikm: &[u8], salt: &[u8]) -> Self {
        let (prk, hk) = hkdf::Hkdf::<H, I>::extract(Some(salt), ikm);
        WhiteflagHkdf {
            hk,
            prk: prk.to_vec(),
        }
    }

    /// Performs RFC 5869 HKDF Step 2: expand
    pub fn expand(&self, info: &[u8], key_length: usize) -> CryptoResult<Vec<u8>> {
        let mut okm: Vec<u8> = vec![0; key_length];
        self.hk
            .expand(info, &mut okm)
            .map_err(|e| CryptoError::HkdfOutput(e))?;
        Ok(okm)
    }
}

impl<H, I> TryFrom<&[u8]> for WhiteflagHkdf<H, I>
where
    H: hkdf::hmac::digest::OutputSizeUser,
    I: hkdf::HmacImpl<H>,
{
    type Error = CryptoError;

    fn try_from(prk: &[u8]) -> Result<Self, Self::Error> {
        let hk = hkdf::Hkdf::<H, I>::from_prk(prk).map_err(|e| CryptoError::HkdfInput(e))?;
        Ok(WhiteflagHkdf {
            hk,
            prk: prk.to_vec(),
        })
    }
}

impl<H, I> AsRef<[u8]> for WhiteflagHkdf<H, I>
where
    H: hkdf::hmac::digest::OutputSizeUser,
    I: hkdf::HmacImpl<H>,
{
    fn as_ref(&self) -> &[u8] {
        &self.prk
    }
}
