use self::common::{append_bits, crop_bits, extract_bits};
use crate::wf_field::{Field, FieldDefinition};

#[cfg(test)]
mod tests;

pub mod common;
pub mod constants;
mod decode;
mod encode;

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
        self.append(field.into());
    }

    pub fn append(&mut self, buffer: WhiteflagBuffer) {
        let (buffer, length) =
            append_bits(&self.data, self.bit_length, &buffer.data, buffer.bit_length);

        self.data = buffer;
        self.bit_length = length;
    }

    pub fn extract_message_field(
        &self,
        field: FieldDefinition,
        start_bit: usize,
    ) -> (usize, Field) {
        let field_bit_length = field.bit_length();
        let bit_length = if field_bit_length >= 1 {
            field_bit_length
        } else {
            let mut bit_length = self.bit_length - start_bit;
            bit_length -= bit_length % &field.encoding.bit_length;
            bit_length
        };

        let field_buffer: Vec<u8> =
            extract_bits(&self.data, self.bit_length, start_bit, bit_length);

        (bit_length, field.decode(field_buffer))
    }

    pub fn crop(&self) -> Vec<u8> {
        crop_bits(&self.data, self.bit_length as isize)
    }

    pub fn bit_length(&self) -> usize {
        self.bit_length
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

impl Default for WhiteflagBuffer {
    fn default() -> Self {
        Self {
            data: Default::default(),
            bit_length: Default::default(),
        }
    }
}
