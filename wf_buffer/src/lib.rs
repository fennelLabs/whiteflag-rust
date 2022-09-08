use hex::FromHexError;
use wf_common::{
    common::{append_bits, crop_bits, extract_bits, remove_hexadecimal_prefix},
    constants::BYTE,
};
use wf_field::{Field, FieldDefinition};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_field;

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

    pub fn extract_bits(&self, start: usize, end: usize) -> WhiteflagBuffer {
        WhiteflagBuffer::new(
            extract_bits(&self.data, self.bit_length, start, end),
            end - start,
        )
    }

    pub fn extract_bits_from(&self, start: usize) -> WhiteflagBuffer {
        WhiteflagBuffer::new(
            extract_bits(&self.data, self.bit_length, start, self.bit_length),
            self.bit_length - start,
        )
    }

    pub fn append_field(&mut self, field: &Field) {
        self.append(field.into(), None);
    }

    pub fn append(&mut self, mut buffer: WhiteflagBuffer, bits: Option<usize>) {
        let bit_length_to_extract = bits.unwrap_or_else(|| buffer.bit_length);
        let (buffer, length) = append_bits(
            &self.data,
            self.bit_length,
            buffer.to_byte_array(),
            bit_length_to_extract,
        );

        self.data = buffer;
        self.bit_length = length;
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

    pub fn crop(&mut self) {
        crop_bits(self.data.as_mut(), self.bit_length);
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

    /// Returns the Whiteflag encoded message as a byte array
    /// @return a byte array with an encoded message
    pub fn to_byte_array<'a>(&'a mut self) -> &'a [u8] {
        crop_bits(self.data.as_mut(), self.bit_length);
        &self.data
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

impl Default for WhiteflagBuffer {
    fn default() -> Self {
        Self {
            data: Default::default(),
            bit_length: Default::default(),
        }
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

pub trait BufferReader {
    fn read(&self, buffer: &WhiteflagBuffer) -> String;
}

impl BufferReader for FieldDefinition {
    /// used in the decoding process
    fn read(&self, buffer: &WhiteflagBuffer) -> String {
        buffer.extract_message_value(&self, self.positions.bit_start)
    }
}
