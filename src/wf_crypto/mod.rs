#[cfg(test)]
mod keypair_tests;

#[cfg(test)]
mod crypto_util_tests;

#[cfg(test)]
mod wf_encryption_key_tests;

mod crypto_util;
mod cipher;
mod error;
pub mod ecdh_keypair;
pub mod wf_encryption_key;
pub mod encryption_method;

pub fn hkdf(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, ()> {
    crypto_util::SimpleWhiteflagHkdf::<sha2::Sha256>::new(ikm, salt).expand(info, length)
}
