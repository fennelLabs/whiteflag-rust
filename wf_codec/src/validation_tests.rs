use super::encoding;
use wf_common::constants::{BYTE, OCTET, QUADBIT};
use wf_validation::{invalid_length_test, test, ValidationError};

const PRINT_ERRORS: bool = true;

#[test]
fn invalid_charset_1() {
    // DEC encoding only accepts numbers
    let validator = encoding::DEC;
    let data = "AB";

    assert!(
        matches!(test(validator, data), Err(ValidationError::InvalidCharset)),
        "InvalidCharset was not thrown"
    );
}

#[test]
fn invalid_charset_2() {
    // BIN encoding only accepts 0s and 1s
    let validator = encoding::BIN;
    let data = "4";

    assert!(
        matches!(test(validator, data), Err(ValidationError::InvalidCharset)),
        "InvalidCharset was not thrown"
    );
}

#[test]
fn incorrect_length_1() {
    // encoding::DATETIME should be 20 bytes
    let validator = encoding::DATETIME;
    let data = "000000000000000000000000000000000000000000000000000000000000000";

    invalid_length_test(validator, data, 20);
}

#[test]
fn incorrect_length_2() {
    // encoding::LAT should be 9 bytes
    let validator = encoding::LAT;
    let data = "1234";

    invalid_length_test(validator, data, 3);
}
