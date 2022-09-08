use hex::FromHexError;
use wf_common::{
    common::{extract_bits, remove_hexadecimal_prefix},
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
