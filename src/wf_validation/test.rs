use super::*;
use crate::wf_codec::encoding::Encoding;
use crate::wf_field::definitions;

const PRINT_ERRORS: bool = true;

mod encoding {
    use super::*;
    use crate::{
        wf_buffer::constants::{BYTE, OCTET, QUADBIT},
        wf_codec::encoding,
    };

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
}

mod field_definition {
    use super::*;

    #[test]
    fn incorrect_length_1() {
        // Header::REFERENCED_MESSAGE can only be 64 bytes long
        let validator = definitions::Header::REFERENCED_MESSAGE;
        let data = "000000000000000000000000000000000000000000000000000000000000000";

        invalid_length_test(validator, data, 64);
    }

    #[test]
    fn incorrect_length_2() {
        // Sign::OBJECT_ORIENTATION can only be 3 bytes long
        let validator = definitions::Sign::OBJECT_ORIENTATION;
        let data = "1234";

        invalid_length_test(validator, data, 3);
    }

    #[test]
    fn correct_byte_config_1() {
        // Header::DURESS_INDICATOR expects 1 byte
        let def = definitions::Header::DURESS_INDICATOR;
        assert_eq!(1, def.expected_byte_length().expect("start and end byte for Header::DURESS_INDICATOR field definition is not configured properly"));
    }

    #[test]
    fn correct_byte_config_2() {
        // Request::OBJECT_TYPE_QUANT expects 2 bytes
        let def = definitions::Request::OBJECT_TYPE_QUANT;
        assert_eq!(2, def.expected_byte_length().expect("start and end byte for Request::OBJECT_TYPE_QUANT field definition is not configured properly"));
    }
}
/*
#[test]
fn invalid_values_1() {
    let def = definitions::Header::PREFIX;
    let data = "12";

    // ensure InvalidLength will not be thrown
    assert_eq!(
        data.len(),
        def.expected_byte_length().expect(
            "start and end byte for Header::PREFIX field definition is not configured properly"
        )
    );

    assert!(
        matches!(test(def, data), Err(ValidationError::InvalidCharset)),
        "InvalidCharset was not thrown"
    );
} */

fn test<T: Validation>(validator: T, data: &str) -> Result<bool, ValidationError> {
    let result = validator.validate(data);
    assert!(result.is_err(), "the test did not throw an error");
    if result.is_err() && PRINT_ERRORS {
        println!("{}", result.as_ref().unwrap_err());
    }
    result
}

fn invalid_length_test<T: Validation>(validator: T, data: &str, expected_length: usize) {
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
