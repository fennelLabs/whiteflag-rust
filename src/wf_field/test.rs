use crate::wf_codec::encoding::{BIN, DATETIME, DEC, HEX, UTF8};
use crate::{wf_buffer::WhiteflagBuffer, wf_field::Field, wf_field::FieldDefinition};

use super::definitions;

const FIELDNAME: &str = "TESTFIELD";

#[test]
fn test_add_field_utf() {
    let mut buffer: WhiteflagBuffer = WhiteflagBuffer::default();

    let field = FieldDefinition::new(FIELDNAME, None, UTF8, 0, -1)
        .set("text")
        .expect("invalid");

    buffer.append_field(&field);

    assert_eq!(
        field.bit_length(),
        buffer.bit_length(),
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "74657874",
        buffer.as_hex(),
        "Message field (UTF) should be correctly encoded and added"
    );
}

#[test]
fn test_extract_field_utf() {
    let buffer: WhiteflagBuffer = vec![0x95, 0x74, 0x78, 0x74].into();
    let def = FieldDefinition::new(FIELDNAME, None, UTF8, 0, -1);
    let field = buffer.extract_message_value(&def, 8);

    assert_eq!(
        "txt", field,
        "Extracted message field (UTF) should contain the correct value"
    );
}

#[test]
fn test_add_field_bin_1() {
    let mut buffer: WhiteflagBuffer = WhiteflagBuffer::default();
    let field = FieldDefinition::new(FIELDNAME, None, BIN, 0, -1)
        .set("01")
        .expect("invalid");

    buffer.append_field(&field);

    assert_eq!(
        field.bit_length(),
        buffer.bit_length(),
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "40",
        buffer.as_hex(),
        "Message field (bin) should be correctly encoded and added"
    );
}

#[test]
fn test_add_field_bin_2() {
    let mut buffer: WhiteflagBuffer = WhiteflagBuffer::default();
    let field = FieldDefinition::new(FIELDNAME, None, BIN, 0, 3)
        .set("101")
        .expect("invalid");

    buffer.append_field(&field);

    assert_eq!(
        field.bit_length(),
        buffer.bit_length(),
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "a0",
        buffer.as_hex(),
        "Message field (bin) should be correctly encoded and added"
    );
}

#[test]
fn test_add_field_dec() {
    let mut buffer: WhiteflagBuffer = WhiteflagBuffer::default();
    let field = FieldDefinition::new(FIELDNAME, None, DEC, 0, 4)
        .set("1478")
        .expect("invalid");

    buffer.append_field(&field);

    assert_eq!(
        field.bit_length(),
        buffer.bit_length(),
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "1478",
        buffer.as_hex(),
        "Message field (bin) should be correctly encoded and added"
    );
}

#[test]
fn test_extract_field_dec() {
    let buffer: WhiteflagBuffer = vec![0x95, 0x91, 0xFF, 0xE7].into();
    let def = FieldDefinition::new(FIELDNAME, None, DEC, 0, 2);

    let field = buffer.extract_message_value(&def, 10);

    assert_eq!(
        "47", field,
        "Extracted message field (dec) should contain the correct value"
    );
}

#[test]
fn test_add_field_hex() {
    let mut buffer = WhiteflagBuffer::default();

    let field = FieldDefinition::new(FIELDNAME, None, HEX, 0, 4)
        .set("3f8C")
        .expect("invalid");

    buffer.append_field(&field);

    assert_eq!(
        field.bit_length(),
        buffer.bit_length(),
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "3f8c",
        buffer.as_hex(),
        "Message field (hex) should be correctly encoded and added"
    );
}

#[test]
fn test_extract_field_hex() {
    let buffer: WhiteflagBuffer = vec![0x95, 0xDD, 0xFF, 0xE7].into();

    let def: FieldDefinition = FieldDefinition::new(FIELDNAME, None, HEX, 0, 2);
    let field = buffer.extract_message_value(&def, 9);

    assert_eq!(
        "bb", field,
        "Extracted message field (dec) should contain the correct value"
    );
}

#[test]
fn test_add_field_date_time() {
    let mut buffer = WhiteflagBuffer::default();

    let field = FieldDefinition::new(FIELDNAME, None, DATETIME, 0, -1)
        .set("2020-07-01T21:42:23Z")
        .expect("invalid");

    buffer.append_field(&field);

    assert_eq!(
        field.bit_length(),
        buffer.bit_length(),
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "20200701214223",
        buffer.as_hex(),
        "Message field (hex) should be correctly encoded and added"
    );
}

#[test]
fn test_extract_message_code() {
    let def = definitions::message_code();
    let buffer = WhiteflagBuffer::decode_from_hexadecimal("5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380").unwrap();

    let field = buffer.extract_message_value(&def, 33);

    assert_eq!("A", field, "extracted message code should be A");
}
