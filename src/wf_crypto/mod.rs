use crate::error::WhiteflagResult;

#[cfg(test)]
mod keypair_tests;

#[cfg(test)]
mod crypto_util_tests;

#[cfg(test)]
mod cipher_tests;

pub mod cipher;
mod crypto_util;
pub mod ecdh_keypair;
pub mod encryption_method;
mod error;
pub mod wf_encryption_key;

pub fn hkdf(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> WhiteflagResult<Vec<u8>> {
    crypto_util::SimpleWhiteflagHkdf::<sha2::Sha256>::new(ikm, salt).expand(info, length)
}
