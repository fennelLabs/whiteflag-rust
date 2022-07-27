use std::vec;

use fennel_lib::{aes_decrypt, aes_encrypt, generate_keys};

struct WhiteflagCipher {
    secret_key: [u8; 32],
    context: Vec<u8>,
}

trait WfCipher {
    fn new(key: [u8; 32]) -> Self;

    fn setContext(&mut self, context: String);

    fn setContextFromBytes(&mut self, context: Vec<u8>);

    fn encrypt(&self, data: String) -> String;

    fn decrypt(&self, data: String) -> String;

    fn getContext(&self) -> Vec<u8>;
}

impl WfCipher for WhiteflagCipher {
    fn new(key: [u8; 32]) -> Self {
        WhiteflagCipher {
            secret_key: key,
            context: vec![],
        }
    }

    fn setContext(&mut self, context: String) {
        return self.setContextFromBytes(hex::decode(context).unwrap());
    }

    fn setContextFromBytes(&mut self, context: Vec<u8>) {
        self.context = context;
    }

    fn encrypt(&self, data: String) -> String {
        let (key, _) = generate_keys(&hex::decode(self.secret_key).unwrap());
        return hex::encode(aes_encrypt(&key, data));
    }

    fn decrypt(&self, data: String) -> String {
        let (_, key) = generate_keys(&hex::decode(self.secret_key).unwrap());
        return aes_decrypt(&key, hex::decode(data).unwrap());
    }

    fn getContext(&self) -> Vec<u8> {
        self.context.clone()
    }
}
