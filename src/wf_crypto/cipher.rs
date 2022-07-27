use std::vec;

use fennel_lib::{aes_decrypt, aes_encrypt, generate_keys};

struct WhiteflagCipher {
    secret_key: [u8; 32],
    context: Vec<u8>,
}

trait WfCipher {
    fn new(key: [u8; 32]) -> Self;

    fn set_context(&mut self, context: String);

    fn set_context_from_bytes(&mut self, context: Vec<u8>);

    fn encrypt(&self, data: String) -> String;

    fn decrypt(&self, data: String) -> String;

    fn get_context(&self) -> Vec<u8>;
}

impl WfCipher for WhiteflagCipher {
    fn new(key: [u8; 32]) -> Self {
        WhiteflagCipher {
            secret_key: key,
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
        let (key, _) = generate_keys(&hex::decode(self.secret_key).unwrap());
        hex::encode(aes_encrypt(&key, data))
    }

    fn decrypt(&self, data: String) -> String {
        let (_, key) = generate_keys(&hex::decode(self.secret_key).unwrap());
        aes_decrypt(&key, hex::decode(data).unwrap())
    }

    fn get_context(&self) -> Vec<u8> {
        self.context.clone()
    }
}
