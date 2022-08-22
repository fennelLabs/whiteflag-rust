use super::wf_encryption_key::{WfEncryptionKey, WhiteflagEncryptionKey};
use fennel_lib::{aes_decrypt, aes_encrypt, generate_keys};
use std::vec;

pub struct WhiteflagCipher {
    secret_key: [u8; 32],
    context: Vec<u8>,
}

pub trait WfCipher {
    fn new(key: [u8; 32]) -> Self;

    fn from_key(key: WhiteflagEncryptionKey) -> Self;

    fn set_context(&mut self, context: String);

    fn set_context_from_bytes(&mut self, context: Vec<u8>);

    fn encrypt(&self, data: String) -> String;

    fn decrypt(&self, data: String) -> String;

    fn get_context(&self) -> Vec<u8>;

    fn encrypt_as_bytes<T: AsRef<[u8]>>(&self, data: T) -> Vec<u8>;
}

impl WfCipher for WhiteflagCipher {
    fn new(key: [u8; 32]) -> Self {
        WhiteflagCipher {
            secret_key: key,
            context: vec![],
        }
    }

    fn from_key(key: WhiteflagEncryptionKey) -> Self {
        WhiteflagCipher {
            secret_key: key.get_secret_key().try_into().unwrap(),
            context: vec![],
        }
    }

    fn set_context(&mut self, context: String) {
        self.set_context_from_bytes(hex::decode(context).unwrap())
    }

    fn set_context_from_bytes(&mut self, context: Vec<u8>) {
        self.context = context;
    }

    fn encrypt(&self, data: String) -> String {
        let (key, _) = generate_keys(&self.secret_key);
        hex::encode(aes_encrypt(&key, data.as_bytes()))
    }

    fn encrypt_as_bytes<T: AsRef<[u8]>>(&self, data: T) -> Vec<u8> {
        let (key, _) = generate_keys(&self.secret_key);
        aes_encrypt(&key, data)
    }

    fn decrypt(&self, data: String) -> String {
        let (_, key) = generate_keys(&self.secret_key);
        String::from_utf8_lossy(&aes_decrypt(&key, hex::decode(data).unwrap().as_ref())).to_string()
    }

    fn get_context(&self) -> Vec<u8> {
        self.context.clone()
    }
}
