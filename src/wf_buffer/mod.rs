use self::{
    common::{append_bits, crop_bits, extract_bits},
    constants::BYTE,
};
use crate::wf_field::{Field, FieldDefinition};

#[cfg(test)]
mod tests;

pub mod common;
pub mod constants;
mod decode;
mod encode;
mod hexadecimal_string;

use hex::FromHexError;
pub use hexadecimal_string::HexadecimalString;

pub struct WhiteflagBuffer {
    data: Vec<u8>,
    bit_length: usize,
}

impl WhiteflagBuffer {
    pub fn new(buffer: Vec<u8>, bit_length: usize) -> Self {
        WhiteflagBuffer {
            data: buffer,
            bit_length,
        }
    }

    pub fn append_field(&mut self, field: &Field) {
        self.append(field.into(), None);
    }

    pub fn append(&mut self, buffer: WhiteflagBuffer, bits: Option<usize>) {
        let bit_length_to_extract = bits.unwrap_or_else(|| buffer.bit_length);
        let (buffer, length) = append_bits(
            &self.data,
            self.bit_length,
            &buffer.data,
            bit_length_to_extract,
        );

        self.data = buffer;
        self.bit_length = length;
    }

    pub fn extract_message_field(&self, definition: FieldDefinition, start_bit: usize) -> Field {
        let field_bit_length = definition.bit_length();
        let bit_length = if field_bit_length >= 1 {
            field_bit_length
        } else {
            let mut bit_length = self.bit_length - start_bit;
            bit_length -= bit_length % &definition.encoding.bit_length;
            bit_length
        };

        let field_buffer: Vec<u8> =
            extract_bits(&self.data, self.bit_length, start_bit, bit_length);

        definition.decode(field_buffer)
    }

    pub fn crop(&self) -> Vec<u8> {
        crop_bits(&self.data, self.bit_length as isize)
    }

    pub fn bit_length(&self) -> usize {
        self.bit_length
    }

    pub fn as_hex(&self) -> HexadecimalString {
        self.into()
    }

    /**
     * decodes a hexadecimal string into a buffer and includes bit_length
     * java equivalent: WfBinaryBuffer.convertToByteArray
     */
    pub fn decode_from_hexadecimal<T: AsRef<str>>(hex: T) -> Result<WhiteflagBuffer, FromHexError> {
        let buffer = HexadecimalString::decode(hex)?;
        Ok(buffer.into())
    }
}

impl From<(Vec<u8>, usize)> for WhiteflagBuffer {
    fn from((buffer, bit_length): (Vec<u8>, usize)) -> Self {
        WhiteflagBuffer::new(buffer, bit_length)
    }
}

impl From<WhiteflagBuffer> for (Vec<u8>, usize) {
    fn from(buffer: WhiteflagBuffer) -> Self {
        (buffer.data, buffer.bit_length)
    }
}

impl AsRef<Vec<u8>> for WhiteflagBuffer {
    fn as_ref(&self) -> &Vec<u8> {
        &self.data
    }
}

impl From<Vec<u8>> for WhiteflagBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        let bit_length = buffer.len() * BYTE;
        WhiteflagBuffer {
            data: buffer,
            bit_length,
        }
    }
}

impl Default for WhiteflagBuffer {
    fn default() -> Self {
        Self {
            data: Default::default(),
            bit_length: Default::default(),
        }
    }
}

impl From<&WhiteflagBuffer> for HexadecimalString {
    fn from(buffer: &WhiteflagBuffer) -> Self {
        HexadecimalString::new(buffer.as_ref())
    }
}
