use crate::{wf_codec::encoding::*, wf_validation::*};
use regex::Regex;

use super::Field;

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    pub name: &'static str,
    pub pattern: Option<Regex>,
    pub encoding: Encoding,
    pub start_byte: usize,
    pub end_byte: isize,
}

impl FieldDefinition {
    pub fn new(
        name: &'static str,
        pattern: Option<Regex>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> FieldDefinition {
        FieldDefinition {
            name,
            pattern, //: pattern.expect(&format!("invalid regular expression pattern: {}", name)),
            encoding,
            start_byte,
            end_byte,
        }
    }

    pub fn new2(
        name: &'static str,
        pattern: Result<Regex, regex::Error>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> FieldDefinition {
        FieldDefinition {
            name,
            pattern: pattern.ok(), //: pattern.expect(&format!("invalid regular expression pattern: {}", name)),
            encoding,
            start_byte,
            end_byte,
        }
    }

    pub fn get_minimum_starting_position(&self) -> usize {
        if self.end_byte < 0 {
            return self.start_byte;
        }

        self.end_byte as usize
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

    pub fn decode(&self, data: Vec<u8>) -> String {
        self.encoding.decode(data, self.bit_length())
    }

    pub fn decode_to_field(self, data: Vec<u8>) -> Field {
        let value = self.encoding.decode(data, self.bit_length());
        Field::new(self, value)
    }

    pub fn encode(&self, data: String) -> Vec<u8> {
        self.encoding.encode(data)
    }

    /**
     * Gets the byte length of the unencoded field value
     * @return the byte length of the unencoded field value
     */
    pub fn byte_length(&self) -> usize {
        if self.end_byte < 0 {
            return 0;
        }

        return self.end_byte as usize - self.start_byte;
    }

    /**
     * Gets the bit length of the encoded field
     * @return the bit length of the compressed encoded field value
     */
    pub fn bit_length(&self) -> usize {
        return self.encoding.convert_to_bit_length(self.byte_length());
    }
}
