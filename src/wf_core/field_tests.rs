use super::{
    field::Field,
    wf_codec::{
        common::{decode_from_hexadecimal, to_hex},
        encoding::*,
    },
};

const FIELDNAME: &str = "TESTFIELD";

#[test]
fn utf_encoding() {
    let mut field = Field::new(FIELDNAME, None, UTF8, 0, -1);
    field.set("WF").unwrap();

    assert_eq!(
        "5746",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, UTF8, 0, -1);
    let (buffer, _) = decode_from_hexadecimal("5746");
    let result = "WF";

    assert_eq!(
        result,
        field.decode(buffer),
        "UTF-8 field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "UTF-8 decoded field value should be correctly set"
    );
}

#[test]
fn bin_encoding_1() {
    let mut field = Field::new(FIELDNAME, None, BIN, 0, 8);
    field.set("10111011").unwrap();

    assert_eq!(
        "bb",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, BIN, 1, 7);
    let (buffer, _) = decode_from_hexadecimal("aa");
    let result = "101010";

    assert_eq!(
        result,
        field.decode(buffer),
        "Binary field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Binary decoded field value should be correctly set"
    );
}

#[test]
fn bin_encoding_2() {
    let mut field = Field::new(FIELDNAME, None, BIN, 4, 5);
    field.set("1").unwrap();

    assert_eq!(
        "80",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, BIN, 4, 5);
    let (buffer, _) = decode_from_hexadecimal("80");
    let result = "1";

    assert_eq!(
        result,
        field.decode(buffer),
        "Binary field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Binary decoded field value should be correctly set"
    );
}

#[test]
fn bin_decoding_2_b() {
    let mut field = Field::new(FIELDNAME, None, BIN, 2, 3);
    let (buffer, _) = decode_from_hexadecimal("7f");
    let result = "0";

    assert_eq!(
        result,
        field.decode(buffer),
        "Binary field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Binary decoded field value should be correctly set"
    );
}

#[test]
fn dec_encoding() {
    let mut field = Field::new(FIELDNAME, None, DEC, 0, 3);
    field.set("1230").unwrap();

    assert_eq!(
        "1230",
        to_hex(&field.encode().expect("tried encoding empty field")),
        "Decimal field should be correctly binary encoded"
    );
    assert_eq!(
        3,
        field.byte_length(),
        "Unencoded Decimal field should be 3 bytes"
    );
    assert_eq!(
        12,
        field.bit_length(),
        "Encoded Decimal field should be 12 bits"
    );
}

#[test]
fn dec_decoding() {
    let mut field = Field::new(FIELDNAME, None, DEC, 0, 3);
    let (buffer, _) = decode_from_hexadecimal("1234");
    let result = "123";

    assert_eq!(
        result,
        field.decode(buffer),
        "Decimal field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Decimal decoded field value should be correctly set"
    );
}

#[test]
fn hex_encoding() {
    let mut field = Field::new(FIELDNAME, None, HEX, 0, 2);
    field.set("3f").unwrap();

    assert_eq!(
        "3f",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, HEX, 0, 2);
    let (buffer, _) = decode_from_hexadecimal("0x3f");
    let result = "3f";

    assert_eq!(
        result,
        field.decode(buffer),
        "Hexadecimal field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Hexadecimal decoded field value should be correctly set"
    );
}

#[test]
fn datetime_encoding() {
    let mut field = Field::new(FIELDNAME, None, DATETIME, 0, -1);
    field.set("2020-07-01T21:42:23Z").unwrap();

    assert_eq!(
        "20200701214223",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, DATETIME, 0, -1);
    let (buffer, _) = decode_from_hexadecimal("20200701214223");
    let result = "2020-07-01T21:42:23Z";

    assert_eq!(
        result,
        field.decode(buffer),
        "DateTime field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "DateTime decoded field value should be correctly set"
    );
}

#[test]
fn duration_encoding() {
    let mut field = Field::new(FIELDNAME, None, DURATION, 0, 10);
    field.set("P24D11H30M").unwrap();

    assert_eq!(
        "241130",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, DURATION, 0, 10);
    let (buffer, _) = decode_from_hexadecimal("241130");
    let result = "P24D11H30M";

    assert_eq!(
        result,
        field.decode(buffer),
        "Duration field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Duration decoded field value should be correctly set"
    );
}

#[test]
fn latitude_encoding() {
    let mut field = Field::new(FIELDNAME, None, LAT, 0, 9);
    field.set("+23.34244").unwrap(); // 1001 0001 1001 1010 0001 0010 0010 0000

    assert_eq!(
        "919a1220",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, LAT, 0, 9);
    let (buffer, _) = decode_from_hexadecimal("919a1220");
    let result = "+23.34244";

    assert_eq!(
        result,
        field.decode(buffer),
        "Latitude field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Latitude decoded field value should be correctly set"
    );
}

#[test]
fn longitude_encoding() {
    let mut field = Field::new(FIELDNAME, None, LONG, 0, 10);
    field.set("-163.34245").unwrap(); // 0000 1011 0001 1001 1010 0001 0010 0010 1000

    assert_eq!(
        "0b19a12280",
        to_hex(&field.encode().expect("tried encoding empty field")),
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
    let mut field = Field::new(FIELDNAME, None, LONG, 0, 10);
    let (buffer, _) = decode_from_hexadecimal("8b19a12380");
    let result = "+163.34247";

    assert_eq!(
        result,
        field.decode(buffer),
        "Longitude field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Longitude decoded field value should be correctly set"
    );
}

#[test]
fn longitude_decoding_2() {
    let mut field = Field::new(FIELDNAME, None, LONG, 0, 10);
    let (buffer, _) = decode_from_hexadecimal("0319a12380");
    let result = "-063.34247";

    assert_eq!(
        result,
        field.decode(buffer),
        "Longitude field should be correctly decoded"
    );
    assert_eq!(
        result,
        field.get().as_ref().expect("no value was set"),
        "Longitude decoded field value should be correctly set"
    );
}
