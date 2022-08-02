use crate::wf_field::FieldDefinition;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("length is invalid\nvalue: {0}\nexpected byte length: {1}")]
    InvalidLength(String, usize),
    #[error("value characters are invalid")]
    InvalidCharset,
}

pub trait Validation {
    fn validate(&self, value: &str) -> Result<bool, ValidationError>;
}

impl Validation for FieldDefinition {
    fn validate(&self, value: &str) -> Result<bool, ValidationError> {
        let expected_byte_length = self.byte_length();
        if expected_byte_length != 0 && value.len() != expected_byte_length {
            return Err(ValidationError::InvalidLength(
                value.to_string(),
                expected_byte_length,
            ));
        }

        self.encoding.validate(value)
    }
}
