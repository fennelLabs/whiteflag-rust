use aes_tools::FennelCipher;
use wf_buffer::WhiteflagBuffer;
use wf_field::definitions::WhiteflagFields;

/// everything is encrypted in a whiteflag buffer except for the first few header fields
///
/// Prefix, Version, and `EncryptionIndicator` do not get encrypted
/// Therefore, when encrypting, the buffer is split and the second half gets encrypted
/// Likewise, when decrypting, the buffer is split and the second half gets decrypted
pub struct CryptedBuffer {
    unencrypted_first_half: WhiteflagBuffer,
    encrypted_second_half: WhiteflagBuffer,
}

pub enum CryptMode {
    Encrypt,
    Decrypt,
}

impl CryptedBuffer {
    /// creates a new [`CryptedBuffer`] by splitting the given buffer at the `EncryptionHeader` bit index
    pub fn new(buffer: WhiteflagBuffer) -> Self {
        Self::new_split_at(
            buffer,
            WhiteflagFields::HeaderEncryptionIndicator
                .create_codec_position()
                .bit_end,
        )
    }

    /// splits the buffer at the chosen index
    fn new_split_at(buffer: WhiteflagBuffer, split_at: usize) -> Self {
        let unencrypted_first_half = buffer.extract_bits(0, split_at);
        let encrypted_second_half = buffer.extract_bits_from(split_at);

        Self {
            unencrypted_first_half,
            encrypted_second_half,
        }
    }

    /// creates an encrypted [`WhiteflagBuffer`] or decrypted [`WhiteflagBuffer`] depending on [`CryptMode`]
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
