/// there are four ways a value can be resolved as invalid
/// - [Encoding] invalid length according to encoding
/// - [Encoding] invalid character set according to encoding
/// - [FieldDefinition].{end_byte - start_byte} invalid length according to field definition
/// - [FieldDefinition] invalid value according to field definition
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
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
    fn validate(&self, value: &str) -> Result<(), ValidationError>;
}

pub fn test<T: Validation>(validator: T, data: &str) -> Result<(), ValidationError> {
    let result = validator.validate(data);
    assert!(result.is_err(), "the test did not throw an error");
    if result.is_err() {
        println!("{}", result.as_ref().unwrap_err());
    }
    result
}

pub fn invalid_length_test<T: Validation>(validator: T, data: &str, expected_length: usize) {
    // ensure we have an incorrect length
    assert!(
        data.len() != expected_length,
        "the input data byte_length should not be correct"
    );

    assert!(
        matches!(
            test(validator, data),
            Err(ValidationError::InvalidLength {
                data: _,
                expected_length: _,
                specification_level: _
            })
        ),
        "InvalidLength was not thrown"
    );
}
