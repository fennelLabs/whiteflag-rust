use super::definitions;
use wf_validation::invalid_length_test;

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
