#[cfg(test)]
mod keypair_tests;

#[cfg(test)]
mod crypto_util_tests;

#[cfg(test)]
mod cipher_tests;

mod crypto_util;
pub mod ecdh_keypair;
pub mod encryption_method;
mod error;
pub mod wf_encryption_key;

pub use error::{CryptoError, CryptoResult};

pub fn hkdf(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> CryptoResult<Vec<u8>> {
    crypto_util::Hkdf::extract(ikm, salt).expand(info, length)
}
