use super::FieldDefinition;
use wf_codec::encoding::*;

const FIELDNAME: &str = "TESTFIELD";

#[test]
fn utf_encoding() {
    let def = FieldDefinition::new(FIELDNAME, UTF8, 0, 0);
    let field = def.set("WF").unwrap();

    assert_eq!(
        "5746",
        field.encode_as_hex(),
        "UTF-8 field should be correctly hexadecimal encoded"
    );
    assert_eq!(
        2,
        field.byte_length(),
        "Unencoded UTF-8 field should be 2 bytes"
    );
    assert_eq!(
        16,
        field.bit_length(),
        "Encoded UTF-8 field should be 16 bits bytes"
    );
}

#[test]
fn utf_decoding() {
    let def = FieldDefinition::new(FIELDNAME, UTF8, 0, 0);
    let buffer = hex::decode("5746").unwrap();
    let result = "WF";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "UTF-8 field should be correctly decoded");
}

#[test]
fn bin_encoding_1() {
    let bin = FieldDefinition::new(FIELDNAME, BIN, 0, 8);
    let field = bin.set("10111011").unwrap();

    assert_eq!(
        "bb",
        field.encode_as_hex(),
        "Binary field should be correctly binary encoded"
    );
    assert_eq!(
        8,
        field.byte_length(),
        "Unencoded Binary field should be 8 bytes"
    );
    assert_eq!(
        8,
        field.bit_length(),
        "Encoded Binary field should be 8 bits"
    );
}

#[test]
fn bin_decoding_1() {
    let def = FieldDefinition::new(FIELDNAME, BIN, 1, 7);
    let buffer = hex::decode("aa").unwrap();
    let result = "101010";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "Binary field should be correctly decoded");
}

#[test]
fn bin_encoding_2() {
    let bin = FieldDefinition::new(FIELDNAME, BIN, 4, 5);
    let field = bin.set("1").unwrap();

    assert_eq!(
        "80",
        field.encode_as_hex(),
        "Binary field should be correctly binary encoded"
    );
    assert_eq!(
        1,
        field.byte_length(),
        "Unencoded Binary field should be 1 bytes"
    );
    assert_eq!(
        1,
        field.bit_length(),
        "Encoded Binary field should be 1 bits"
    );
}

#[test]
fn bin_decoding_2_a() {
    let def = FieldDefinition::new(FIELDNAME, BIN, 4, 5);
    let buffer = hex::decode("80").unwrap();
    let result = "1";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "Binary field should be correctly decoded");
}

#[test]
fn bin_decoding_2_b() {
    let def = FieldDefinition::new(FIELDNAME, BIN, 2, 3);
    let buffer = hex::decode("7f").unwrap();
    let result = "0";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "Binary field should be correctly decoded");
}

#[test]
fn dec_encoding() {
    let dec = FieldDefinition::new(FIELDNAME, DEC, 0, 4);
    let field = dec.set("1230").unwrap();

    assert_eq!(
        "1230",
        field.encode_as_hex(),
        "Decimal field should be correctly binary encoded"
    );
    assert_eq!(
        4,
        field.byte_length(),
        "Unencoded Decimal field should be 3 bytes"
    );
    assert_eq!(
        16,
        field.bit_length(),
        "Encoded Decimal field should be 12 bits"
    );
}

#[test]
fn dec_decoding() {
    let def = FieldDefinition::new(FIELDNAME, DEC, 0, 3);
    let buffer = hex::decode("1234").unwrap();
    let result = "123";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "Decimal field should be correctly decoded");
}

#[test]
fn hex_encoding() {
    let hex = FieldDefinition::new(FIELDNAME, HEX, 0, 2);
    let field = hex.set("3f").unwrap();

    assert_eq!(
        "3f",
        field.encode_as_hex(),
        "Hexadecimal field should be correctly binary encoded"
    );
    assert_eq!(
        2,
        field.byte_length(),
        "Unencoded Hexadecimal field should be 2 bytes"
    );
    assert_eq!(
        8,
        field.bit_length(),
        "Encoded Hexadecimal field should be 8 bits"
    );
}

#[test]
fn hex_decoding() {
    let def = FieldDefinition::new(FIELDNAME, HEX, 0, 2);
    let buffer = hex::decode("3f").unwrap();
    let result = "3f";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(
        result, actual,
        "Hexadecimal field should be correctly decoded"
    );
}

#[test]
fn datetime_encoding() {
    let datetime = FieldDefinition::new(FIELDNAME, DATETIME, 0, 0);
    let field = datetime.set("2020-07-01T21:42:23Z").unwrap();

    assert_eq!(
        "20200701214223",
        field.encode_as_hex(),
        "DateTime field should be correctly binary encoded"
    );
    assert_eq!(
        20,
        field.byte_length(),
        "Unencoded DateTime field should be 20 bytes"
    );
    assert_eq!(
        56,
        field.bit_length(),
        "Encoded DateTime field should be 56 bits"
    );
}

#[test]
fn datetime_decoding() {
    let def = FieldDefinition::new(FIELDNAME, DATETIME, 0, 0);
    let buffer = hex::decode("20200701214223").unwrap();
    let result = "2020-07-01T21:42:23Z";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "DateTime field should be correctly decoded");
}

#[test]
fn duration_encoding() {
    let duration = FieldDefinition::new(FIELDNAME, DURATION, 0, 10);
    let field = duration.set("P24D11H30M").unwrap();

    assert_eq!(
        "241130",
        field.encode_as_hex(),
        "Duration field should be correctly binary encoded"
    );
    assert_eq!(
        10,
        field.byte_length(),
        "Unencoded Duration field should be 10 bytes"
    );
    assert_eq!(
        24,
        field.bit_length(),
        "Encoded Duration field should be 24 bits"
    );
}

#[test]
fn duration_decoding() {
    let def = FieldDefinition::new(FIELDNAME, DURATION, 0, 10);
    let buffer = hex::decode("241130").unwrap();
    let result = "P24D11H30M";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "Duration field should be correctly decoded");
}

#[test]
fn latitude_encoding() {
    let lat = FieldDefinition::new(FIELDNAME, LAT, 0, 9);
    let field = lat.set("+23.34244").unwrap(); // 1001 0001 1001 1010 0001 0010 0010 0000

    assert_eq!(
        "919a1220",
        field.encode_as_hex(),
        "Latitude field should be correctly binary encoded"
    );
    assert_eq!(
        9,
        field.byte_length(),
        "Unencoded Latitude field should be 9 bytes"
    );
    assert_eq!(
        29,
        field.bit_length(),
        "Encoded Latitude field should be 29 bits"
    );
}

#[test]
fn latitude_decoding() {
    let def = FieldDefinition::new(FIELDNAME, LAT, 0, 9);
    let buffer = hex::decode("919a1220").unwrap();
    let result = "+23.34244";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(result, actual, "Latitude field should be correctly decoded");
}

#[test]
fn longitude_encoding() {
    let long = FieldDefinition::new(FIELDNAME, LONG, 0, 10);
    let field = long.set("-163.34245").unwrap(); // 0000 1011 0001 1001 1010 0001 0010 0010 1000

    assert_eq!(
        "0b19a12280",
        field.encode_as_hex(),
        "Longitude field should be correctly binary encoded"
    );
    assert_eq!(
        10,
        field.byte_length(),
        "Unencoded Longitude field should be 10 bytes"
    );
    assert_eq!(
        33,
        field.bit_length(),
        "Encoded Longitude field should be 33 bits"
    );
}

#[test]
fn longitude_decoding_1() {
    let def = FieldDefinition::new(FIELDNAME, LONG, 0, 10);
    let buffer = hex::decode("8b19a12380").unwrap();
    let result = "+163.34247";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(
        result, actual,
        "Longitude field should be correctly decoded"
    );
}

#[test]
fn longitude_decoding_2() {
    let def = FieldDefinition::new(FIELDNAME, LONG, 0, 10);
    let buffer = hex::decode("0319a12380").unwrap();
    let result = "-063.34247";

    let actual: String = def.decode(&buffer).unwrap();

    assert_eq!(
        result, actual,
        "Longitude field should be correctly decoded"
    );
}
