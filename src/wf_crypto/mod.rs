#[cfg(test)]
mod keypair_tests;

#[cfg(test)]
mod crypto_util_tests;

mod crypto_util;
pub mod ecdh_keypair;

pub fn hkdf(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> Vec<u8> {
    crypto_util::SimpleWhiteflagHkdf::<sha2::Sha256>::new(ikm, salt)
        .expand(info, length)
        .unwrap()
}
