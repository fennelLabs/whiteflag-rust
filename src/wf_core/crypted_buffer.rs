use fennel_lib::FennelCipher;
use wf_buffer::WhiteflagBuffer;
use wf_parser::{MessageHeaderOrder, ParsedFieldDefinition};

pub struct CryptedBuffer {
    unencrypted_first_half: WhiteflagBuffer,
    encrypted_second_half: WhiteflagBuffer,
}

pub enum CryptMode {
    Encrypt,
    Decrypt,
}

impl CryptedBuffer {
    pub fn new(buffer: WhiteflagBuffer) -> Self {
        let indicator =
            &ParsedFieldDefinition::header()[MessageHeaderOrder::EncryptionIndicator.as_usize()];
        Self::new_split_at(buffer, indicator.end_bit)
    }

    fn new_split_at(buffer: WhiteflagBuffer, split_at: usize) -> Self {
        let unencrypted_first_half = buffer.extract_bits(0, split_at);
        let encrypted_second_half = buffer.extract_bits_from(split_at);

        CryptedBuffer {
            unencrypted_first_half,
            encrypted_second_half,
        }
    }

    pub fn crypt<T: FennelCipher>(self, cipher: &T, mode: CryptMode) -> WhiteflagBuffer {
        let mut buffer = WhiteflagBuffer::default();

        let crypted_half = match mode {
            CryptMode::Encrypt => cipher.encrypt(&self.encrypted_second_half),
            CryptMode::Decrypt => cipher.decrypt(&self.encrypted_second_half),
        };

        buffer.append(self.unencrypted_first_half, None);
        buffer.append(crypted_half.into(), None);
        buffer
    }
}

impl CryptMode {
    pub fn crypt<T: FennelCipher>(
        &self,
        cipher: &T,
        input: WhiteflagBuffer,
        position: usize,
    ) -> WhiteflagBuffer {
        let mut buffer = WhiteflagBuffer::default();
        // add unencrypted part
        buffer.append(input.extract_bits(0, position), None);

        let second_half = input.extract_bits_from(position);

        let crypted_half = match self {
            CryptMode::Encrypt => cipher.encrypt(second_half),
            CryptMode::Decrypt => cipher.decrypt(second_half),
        };

        // add decrypted/encrypted part
        buffer.append(crypted_half.into(), None);
        buffer
    }
}
