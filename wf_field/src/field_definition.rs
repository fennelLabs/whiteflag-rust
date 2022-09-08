use crate::{byte_configuration::ByteConfiguration, codec_positions::CodecPositions, Field};
use wf_codec::encoding::Encoding;
use wf_validation::{Validation, ValidationError};

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    pub name: Option<&'static str>,
    pub encoding: Encoding,
    pub positions: CodecPositions,
}

impl FieldDefinition {
    pub fn get_name(&self) -> Option<&'static str> {
        self.name
    }

    /*
    /// used in the compiling process
    pub fn read_from_values<'a, T: FieldValue>(&self, values: &'a [T]) -> &'a str {
        values[self.index].as_ref()
    }
    */

    pub fn new(
        name: &'static str,
        encoding: Encoding,
        start_byte: usize,
        end_byte: usize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: Some(name),
            encoding: encoding.kind.get_encoding(),
            positions: CodecPositions::new(
                ByteConfiguration::new(start_byte, end_byte, encoding),
                0,
            ),
        }
    }

    pub fn new_without_name(
        encoding: Encoding,
        start_byte: usize,
        end_byte: usize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: None,
            encoding: encoding.kind.get_encoding(),
            positions: CodecPositions::new(
                ByteConfiguration::new(start_byte, end_byte, encoding),
                0,
            ),
        }
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
        match self.encoding.decode(data, self.bit_length()) {
            Ok(r) => r,
            Err(e) => {
                panic!("error: {}\n\t{:#?}", e, &self);
            }
        }
    }

    pub fn decode_to_field(self, data: &[u8]) -> Field {
        let value = self.decode(data);
        Field::new(self, value)
    }

    pub fn encode(&self, data: String) -> Vec<u8> {
        self.encoding.encode(data)
    }

    /// returns the byte length of the unencoded field value
    /// if the field definition does not have a fixed length then it will return `0`
    pub fn expected_byte_length(&self) -> Option<usize> {
        match self.positions.bytes.end {
            Some(e) if e > 0 && e > self.positions.bytes.start => {
                Some(e - self.positions.bytes.start)
            }
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

const NULL_FIELD_NAME: &'static str = "NULL FIELD NAME";

impl Validation for FieldDefinition {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        match self.expected_byte_length() {
            Some(len) if len != value.len() => Err(ValidationError::InvalidLength {
                data: value.to_string(),
                expected_length: len,
                specification_level: format!(
                    "== Field Definition Error for {} ==",
                    self.get_name().unwrap_or(NULL_FIELD_NAME)
                ),
            }),
            _ => self.encoding.validate(value),
        }
    }
}
