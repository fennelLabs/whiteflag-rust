use crate::WhiteflagBuffer;
use hex::FromHexError;
use wf_common::{common::remove_hexadecimal_prefix, constants::BYTE};
use wf_field::Field;

pub fn decode_hex<T: AsRef<str>>(value: T) -> Result<Vec<u8>, FromHexError> {
    hex::decode(remove_hexadecimal_prefix(value.as_ref()))
}

impl WhiteflagBuffer {
    pub fn from(buffer: Vec<u8>) -> Self {
        let bit_length = buffer.len() * BYTE;
        WhiteflagBuffer {
            data: buffer,
            bit_length,
        }
    }

    pub fn new(buffer: Vec<u8>, bit_length: usize) -> Self {
        WhiteflagBuffer {
            data: buffer,
            bit_length,
        }
    }

    pub fn as_hex(&self) -> String {
        hex::encode(&self.data)
    }

    /**
     * decodes a hexadecimal string into a buffer and includes bit_length
     * java equivalent: WfBinaryBuffer.convertToByteArray
     */
    pub fn decode_from_hexadecimal<T: AsRef<str>>(hex: T) -> Result<WhiteflagBuffer, FromHexError> {
        let buffer = decode_hex(hex)?;
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

impl AsRef<[u8]> for WhiteflagBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl From<Vec<u8>> for WhiteflagBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        WhiteflagBuffer::from(buffer)
    }
}

impl From<WhiteflagBuffer> for Vec<u8> {
    fn from(buffer: WhiteflagBuffer) -> Self {
        buffer.data
    }
}

impl From<&Field> for WhiteflagBuffer {
    fn from(field: &Field) -> Self {
        let length = field.bit_length();
        WhiteflagBuffer::new(field.into(), length)
    }
}
