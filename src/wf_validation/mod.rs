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
        if let Some(len) = self.expected_byte_length() && value.len() != len {
            return Err(ValidationError::InvalidLength(
                value.to_string(),
                len,
            ));
        }

        self.encoding.validate(value)
    }
}
