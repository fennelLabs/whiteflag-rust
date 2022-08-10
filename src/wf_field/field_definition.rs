use super::Field;
use crate::{wf_codec::encoding::*, wf_validation::*};

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    pub name: Option<&'static str>,
    pub encoding: Encoding,
    pub start_byte: usize,
    pub end_byte: Option<usize>,
}

impl FieldDefinition {
    pub fn get_name(&self) -> Option<&'static str> {
        self.name
    }

    pub fn new(
        name: &'static str,
        encoding: Encoding,
        start_byte: usize,
        end_byte: usize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: Some(name),
            encoding,
            start_byte,
            end_byte: if end_byte < 1 { None } else { Some(end_byte) },
        }
    }

    pub fn next(&self, end: Option<usize>) -> FieldDefinition {
        FieldDefinition {
            name: None,
            encoding: self.encoding.kind.get_encoding(),
            start_byte: self.end_byte.expect("next() assumes an end_byte"),
            end_byte: end,
        }
    }

    pub fn new_without_name(
        encoding: Encoding,
        start_byte: usize,
        end_byte: usize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: None,
            encoding,
            start_byte,
            end_byte: if end_byte < 1 { None } else { Some(end_byte) },
        }
    }

    pub fn get_minimum_starting_position(&self) -> usize {
        if let Some(e) = self.end_byte {
            return e;
        }

        return self.start_byte;
    }

    /**
     * Sets the value of the message field if not already set
     * @param data the data representing the field value
     * @return TRUE if field value is set, FALSE if field already set or data is invalid
     */
    pub fn set<T: AsRef<str> + Into<String>>(self, data: T) -> Result<Field, ValidationError> {
        self.validate(data.as_ref())?;
        Ok(Field::new(self, data.into()))
    }

    pub fn decode(&self, data: &[u8]) -> String {
        self.encoding.decode(data, self.bit_length())
    }

    pub fn decode_to_field(self, data: &[u8]) -> Field {
        let value = self.encoding.decode(data, self.bit_length());
        Field::new(self, value)
    }

    pub fn encode(&self, data: String) -> Vec<u8> {
        self.encoding.encode(data)
    }

    /// returns the byte length of the unencoded field value
    /// if the field definition does not have a fixed length then it will return `0`
    pub fn expected_byte_length(&self) -> Option<usize> {
        match self.end_byte {
            Some(e) if e > 0 && e > self.start_byte => Some(e - self.start_byte),
            _ => None,
        }
    }

    /**
     * Gets the bit length of the encoded field
     * @return the bit length of the compressed encoded field value
     */
    pub fn bit_length(&self) -> usize {
        return self
            .encoding
            .convert_to_bit_length(self.expected_byte_length().unwrap_or(0));
    }
}
