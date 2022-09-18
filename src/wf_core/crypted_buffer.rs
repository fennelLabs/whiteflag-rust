use aes_tools::FennelCipher;
use wf_buffer::WhiteflagBuffer;
use wf_field::definitions::WhiteflagFields;

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
        Self::new_split_at(
            buffer,
            WhiteflagFields::HeaderEncryptionIndicator
                .create_codec_position()
                .bit_end,
        )
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
