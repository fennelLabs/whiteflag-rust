use crate::wf_codec::encoding::Encoding;
use crate::wf_field::FieldDefinition;

#[cfg(test)]
mod test;

/// there are four ways a value can be resolved as invalid
/// - [Encoding] invalid length according to encoding
/// - [Encoding] invalid character set according to encoding
/// - [FieldDefinition].{end_byte - start_byte} invalid length according to field definition
/// - [FieldDefinition] invalid value according to field definition
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("{specification_level}\nunencoded byte length is invalid\nunencoded value: {data}\nexpected byte_length to be {expected_length} but was {}", .data.len())]
    InvalidLength {
        data: String,
        expected_length: usize,
        specification_level: String,
    },
    #[error("the value contains characters that are invalid according to the field's encoding")]
    InvalidCharset,
}

pub trait Validation {
    fn validate(&self, value: &str) -> Result<bool, ValidationError>;
}

impl Validation for FieldDefinition {
    fn validate(&self, value: &str) -> Result<bool, ValidationError> {
        if let Some(len) = self.expected_byte_length() && value.len() != len {
            return Err(ValidationError::InvalidLength {
                data: value.to_string(),
                expected_length: len,
                specification_level: format!("== Field Definition Error for {} ==", self.name)
            });
        }

        self.encoding.validate(value)
    }
}
