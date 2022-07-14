use super::field_definition::FieldDefinition;
use crate::wf_buffer::common::extract_bits;
use crate::{
    wf_codec::encoding::*,
    wf_core::error::{WhiteflagError, WhiteflagResult},
};
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Field {
    pub definition: FieldDefinition,
    value: Option<String>,
}

impl Field {
    pub fn from_definition(definition: FieldDefinition, value: Option<String>) -> Field {
        Field { definition, value }
    }

    pub fn new(
        name: &str,
        pattern: Option<Regex>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> Field {
        Field::from_definition(
            FieldDefinition::new(name, pattern, encoding, start_byte, end_byte),
            None,
        )
    }

    /* pub fn get(&self, data: Vec<String>) -> WhiteflagResult<String> {
        if data.len() < self.get_minimum_starting_position() {
            return Err(WhiteflagError::InvalidLength);
        }

        data[self.start_byte..self.end_byte as usize]
            .first()
            .ok_or(WhiteflagError::InvalidLength)
    } */

    /**
     * Sets the value of the message field if not already set
     * @param data the data representing the field value
     * @return TRUE if field value is set, FALSE if field already set or data is invalid
     */
    pub fn set<T: AsRef<str> + Into<String>>(&mut self, data: T) -> WhiteflagResult<()> {
        if !self.definition.is_valid(&Some(data.as_ref())) {
            return Err(WhiteflagError::InvalidPattern);
        }

        self.value = Some(data.into());
        Ok(())
    }

    pub fn get(&self) -> &Option<String> {
        &self.value
    }

    /**
     * Checks if the message field value has been set. FieldDefinition is considered set if it contains a valid value.
     * @return TRUE if the field has been set, else FALSE
     */
    pub fn is_set(&self) -> bool {
        self.definition.is_valid(&self.value)
    }

    pub fn encode(&self) -> Option<Vec<u8>> {
        match &self.value {
            Some(x) => Some(self.definition.encoding.encode(x)),
            None => None,
        }
    }

    pub fn decode(&mut self, data: Vec<u8>) -> String {
        let s = self.definition.encoding.decode(data, self.bit_length());
        self.value = Some(s.clone());
        s
    }

    /**
     * Gets the byte length of the unencoded field value
     * @return the byte length of the unencoded field value
     */
    pub fn byte_length(&self) -> usize {
        if self.definition.end_byte < 0 {
            if let Some(v) = &self.value {
                return v.len();
            } else {
                return 0;
            }
        }

        return self.definition.end_byte as usize - self.definition.start_byte;
    }

    /**
     * Gets the bit length of the encoded field
     * @return the bit length of the compressed encoded field value
     */
    pub fn bit_length(&self) -> usize {
        return self.definition.encoding.bit_length(self.byte_length());
    }

    /**
     * Extracts and decodes a Whiteflag message field from the binary buffer
     * @param field the message field to be extracted and decoded
     * @param startBit the bit where the encoded field is located in the buffer
     * @return String with the decoded field value
     * @throws WfCoreException if field connot be decoded
     */
    pub fn extract_message_field(
        &mut self,
        message_buffer: &[u8],
        message_buffer_bit_length: usize,
        start_bit: usize,
    ) -> usize {
        let bit_length = if self.bit_length() >= 1 {
            self.bit_length()
        } else {
            let mut bit_length = message_buffer_bit_length - start_bit;
            bit_length -= bit_length % &self.definition.encoding.bit_length;
            bit_length
        };

        let field_buffer: Vec<u8> = extract_bits(
            message_buffer,
            message_buffer_bit_length,
            start_bit,
            bit_length,
        );
        self.decode(field_buffer);

        bit_length
    }
}

impl AsRef<FieldDefinition> for Field {
    fn as_ref(&self) -> &FieldDefinition {
        &self.definition
    }
}
