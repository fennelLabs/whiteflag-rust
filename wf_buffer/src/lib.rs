use hex::FromHexError;
use wf_common::{
    common::{extract_bits, remove_hexadecimal_prefix},
    constants::BYTE,
};
use wf_field::{Field, FieldDefinition};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_field;

mod common;
mod converions;
mod decode;
mod encode;

pub fn decode_hex<T: AsRef<str>>(value: T) -> Result<Vec<u8>, FromHexError> {
    hex::decode(remove_hexadecimal_prefix(value.as_ref()))
}

pub struct WhiteflagBuffer {
    data: Vec<u8>,
    bit_length: usize,
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

    pub fn append_field(&mut self, field: &Field) {
        self.append(field.into(), None);
    }

    pub fn extract_message_field(&self, definition: &FieldDefinition, start_bit: usize) -> Field {
        let value = self.extract_message_value(definition, start_bit);
        Field::new(definition.clone(), value)
    }

    pub fn extract_message_value(&self, definition: &FieldDefinition, start_bit: usize) -> String {
        let field_bit_length = definition.bit_length();
        let bit_length = if field_bit_length >= 1 {
            field_bit_length
        } else {
            let mut bit_length = self.bit_length - start_bit;
            bit_length -= bit_length % &definition.bytes.encoding.bit_length;
            bit_length
        };

        let field_buffer: Vec<u8> =
            extract_bits(&self.data, self.bit_length, start_bit, bit_length);

        definition.decode(&field_buffer)
    }

    pub fn bit_length(&self) -> usize {
        self.bit_length
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

pub trait BufferReader {
    fn read(&self, buffer: &WhiteflagBuffer) -> String;
}

impl BufferReader for FieldDefinition {
    /// used in the decoding process
    fn read(&self, buffer: &WhiteflagBuffer) -> String {
        buffer.extract_message_value(&self, self.positions.bit_start)
    }
}
