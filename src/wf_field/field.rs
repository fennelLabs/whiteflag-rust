use super::field_definition::FieldDefinition;
use crate::wf_buffer::common::extract_bits;
use crate::wf_core::error::{WhiteflagError, WhiteflagResult};

#[derive(Clone, Debug)]
pub struct Field {
    pub definition: FieldDefinition,
    value: String,
    name: String,
}

impl Field {
    pub fn new(definition: FieldDefinition, value: String) -> Field {
        let name = definition.get_name();
        Field::new_with_name(definition, value, name)
    }

    pub fn new_with_name(definition: FieldDefinition, value: String, name: String) -> Field {
        Field {
            definition,
            value,
            name,
        }
    }

    pub fn get_name(&self) -> String {
        self.name
    }

    /* pub fn get(&self, data: Vec<String>) -> WhiteflagResult<String> {
        if data.len() < self.get_minimum_starting_position() {
            return Err(WhiteflagError::InvalidLength);
        }

        data[self.start_byte..self.end_byte as usize]
            .first()
            .ok_or(WhiteflagError::InvalidLength)
    } */

    pub fn get(&self) -> &String {
        &self.value
    }

    pub fn encode(&self) -> Vec<u8> {
        self.definition.encoding.encode(&self.value)
    }

    pub fn encode_as_hex(&self) -> String {
        hex::encode(self.encode())
    }

    pub fn decode(&mut self, data: &[u8]) -> String {
        let s = self.definition.encoding.decode(data, self.bit_length());
        self.value = s.clone();
        s
    }

    /**
     * Gets the byte length of the unencoded field value
     * @return the byte length of the unencoded field value
     */
    pub fn byte_length(&self) -> usize {
        if let Some(len) = self.definition.expected_byte_length() {
            return len;
        }

        self.value.len()
    }

    /**
     * Gets the bit length of the encoded field
     * @return the bit length of the compressed encoded field value
     */
    pub fn bit_length(&self) -> usize {
        return self
            .definition
            .encoding
            .convert_to_bit_length(self.byte_length());
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
    ) -> String {
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

        self.decode(&field_buffer)
    }
}

impl AsRef<FieldDefinition> for Field {
    fn as_ref(&self) -> &FieldDefinition {
        &self.definition
    }
}

impl AsRef<String> for Field {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl Into<String> for Field {
    fn into(self) -> String {
        self.value
    }
}
