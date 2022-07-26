use crate::{wf_buffer::common::decode_from_hexadecimal, wf_crypto::hkdf};

struct WhiteflagAuthMethod {
    value: String,
    length: usize,
    hkdf_salt: Vec<u8>,
}

impl WhiteflagAuthMethod {
    pub fn new(value: String, length: usize, hkdf_salt: Vec<u8>) -> WhiteflagAuthMethod {
        WhiteflagAuthMethod {
            value,
            length,
            hkdf_salt,
        }
    }
}

struct WhiteflagAuthToken {
    token: Vec<u8>,
    method: WhiteflagAuthMethod,
}

impl WhiteflagAuthToken {
    pub fn get_verification_data<T: AsRef<str>>(&self, context: T) -> String {
        let (buffer, _) = decode_from_hexadecimal(context);

        hex::encode(hkdf(
            &self.token,
            &self.method.hkdf_salt,
            &buffer,
            self.method.length,
        ))
    }
}
