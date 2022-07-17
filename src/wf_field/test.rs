use crate::wf_buffer::common::to_hex;
use crate::wf_codec::encoding::{BIN, DEC, UTF8};
use crate::{wf_buffer::WhiteflagBuffer, wf_field::Field, wf_field::FieldDefinition};

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
        to_hex(buffer.as_ref()),
        "Message field (UTF) should be correctly encoded and added"
    );
}

#[test]
fn test_extract_field_utf() {
    let buffer: WhiteflagBuffer = vec![0x95, 0x74, 0x78, 0x74].into();
    let def = FieldDefinition::new(FIELDNAME, None, UTF8, 0, -1);
    let (_, field) = buffer.extract_message_field(def, 8);

    assert_eq!(
        "txt",
        field.get(),
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
        to_hex(buffer.as_ref()),
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
        to_hex(buffer.as_ref()),
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
        to_hex(buffer.as_ref()),
        "Message field (bin) should be correctly encoded and added"
    );
}

#[test]
fn test_extract_field_dec() {
    let buffer: WhiteflagBuffer = vec![0x95, 0x91, 0xFF, 0xE7].into();
    let def = FieldDefinition::new(FIELDNAME, None, DEC, 0, 2);

    let (_, field) =
        buffer.extract_message_field(def, 0);

    assert_eq!(
        "47",
        field.get(),
        "Extracted message field (dec) should contain the correct value"
    );
}
