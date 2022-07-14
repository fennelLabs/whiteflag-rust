use crate::wf_buffer::common::extract_bits;
use crate::{wf_codec::encoding::*, wf_core::error::{WhiteflagError, WhiteflagResult}};
use regex::Regex;

#[derive(Clone)]
pub struct Field {
    pub name: String,
    pattern: Option<Regex>,
    encoding: Encoding,
    pub start_byte: usize,
    pub end_byte: isize,
    value: Option<String>,
}

impl Field {
    pub fn new(
        name: &str,
        pattern: Option<Regex>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> Field {
        Field {
            name: String::from(name),
            pattern, //: pattern.expect(&format!("invalid regular expression pattern: {}", name)),
            encoding,
            start_byte,
            end_byte,
            value: None,
        }
    }

    pub fn new2(
        name: &str,
        pattern: Result<Regex, regex::Error>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> Field {
        Field {
            name: String::from(name),
            pattern: pattern.ok(), //: pattern.expect(&format!("invalid regular expression pattern: {}", name)),
            encoding,
            start_byte,
            end_byte,
            value: None,
        }
    }

    pub fn get_minimum_starting_position(&self) -> usize {
        if self.end_byte < 0 {
            return self.start_byte;
        }

        self.end_byte as usize
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
        if !self.is_valid(&Some(data.as_ref())) {
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
        self.is_valid(&self.value)
    }

    /**
     * Checks if the message field contains a valid value
     * @return TRUE if the field contains a valid value, else FALSE
     */
    pub fn is_valid<T: AsRef<str>>(&self, data: &Option<T>) -> bool {
        let value = match data {
            Some(x) => x,
            None => return false,
        };

        match self.pattern.as_ref() {
            Some(re) => re.is_match(value.as_ref()),
            None => true,
        }
    }

    pub fn encode(&self) -> Option<Vec<u8>> {
        match &self.value {
            Some(x) => Some(self.encoding.encode(x)),
            None => None,
        }
    }

    pub fn decode(&mut self, data: Vec<u8>) -> String {
        let s = self.encoding.decode(data, self.bit_length());
        self.value = Some(s.clone());
        s
    }

    /**
     * Gets the byte length of the unencoded field value
     * @return the byte length of the unencoded field value
     */
    pub fn byte_length(&self) -> usize {
        if self.end_byte < 0 {
            if let Some(v) = &self.value {
                return v.len();
            } else {
                return 0;
            }
        }

        return self.end_byte as usize - self.start_byte;
    }

    /**
     * Gets the bit length of the encoded field
     * @return the bit length of the compressed encoded field value
     */
    pub fn bit_length(&self) -> usize {
        return self.encoding.bit_length(self.byte_length());
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
            bit_length -= bit_length % &self.encoding.bit_length;
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
