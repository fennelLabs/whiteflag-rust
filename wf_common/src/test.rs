use super::{common::extract_bits, constants::BYTE, *};

fn assert_array_eq<T: PartialEq + std::fmt::Debug>(l: Vec<T>, r: Vec<T>) {
    let success = l.iter().eq(r.iter());
    if !success {
        println!("expected: {:?}\nwas: {:?}", l, r);
    }
    assert!(success);
}

#[test]
fn test_shift_right_0() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let shifted_bytes = common::shift_right(&original.clone(), 0);

    assert_array_eq(original, shifted_bytes);
}

#[test]
fn test_shift_right_1() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let expected: Vec<u8> = vec![0x0A, 0x6F, 0xA0];

    assert_array_eq(expected.clone(), common::shift_right(&original.clone(), 3));
    assert_array_eq(expected.clone(), common::shift_left(&original.clone(), -3));
}

#[test]
fn test_shift_right_2() {
    let original: Vec<u8> = vec![0xF6, 0x38, 0x6D];
    let expected: Vec<u8> = vec![0x07, 0xB1, 0xC3, 0x68];
    let shifted_bytes = common::shift_right(&original.clone(), 5);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_shift_right_3() {
    let original: Vec<u8> = vec![0xE6, 0x38, 0x6D, 0x84];
    let expected: Vec<u8> = vec![0x0E, 0x63, 0x86, 0xD8, 0x40];

    assert_array_eq(expected.clone(), common::shift_right(&original.clone(), 12));
    assert_array_eq(expected.clone(), common::shift_left(&original.clone(), -12));
}

#[test]
fn test_shift_left_0() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let shifted_bytes = common::shift_left(&original.clone(), 0);

    assert_array_eq(original, shifted_bytes);
}

#[test]
fn test_shift_left_1() {
    let original: Vec<u8> = vec![0x53, 0x7D];
    let expected: Vec<u8> = vec![0x9B, 0xE8];

    assert_array_eq(expected.clone(), common::shift_left(&original.clone(), 3));
    assert_array_eq(
        expected.clone(),
        common::shift_right(&original.clone(), -11),
    );
}

#[test]
fn test_shift_left_2() {
    let original: Vec<u8> = vec![0xE6, 0x38, 0x87];
    let expected: Vec<u8> = vec![0x1C, 0x43, 0x80];
    let shifted_bytes = common::shift_left(&original.clone(), 7);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_shift_left_3() {
    let original: Vec<u8> = vec![0xD4, 0x4B, 0x93, 0x93];
    let expected: Vec<u8> = vec![0x89, 0x72, 0x72, 0x60];
    let shifted_bytes = common::shift_left(&original.clone(), 5);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_shift_left_4() {
    let original: Vec<u8> = vec![0xE6, 0x38, 0x87, 0x0f];
    let expected: Vec<u8> = vec![0x63, 0x88, 0x70, 0xf0];
    let shifted_bytes = common::shift_left(&original.clone(), 4);

    assert_array_eq(expected, shifted_bytes);
}

#[test]
fn test_extract_bits_1() {
    let byte_array_1: Vec<u8> = vec![0xDD, 0xFF]; // 110|1110111|111111
    let result: Vec<u8> = vec![0xEE]; //    |1110111|0

    assert_eq!(byte_array_1.len(), 2, "Binary buffer length should be 2");
    assert_eq!(
        result,
        extract_bits(&byte_array_1, 16, 3, 7),
        "Should have correctly extracted 7 bits from binary buffer"
    );
}

#[test]
fn test_extract_bits_2() {
    let byte_array_1: Vec<u8> = vec![0xDD, 0xE7, 0xD0]; // 1101110111100|111|11010000
    let result: Vec<u8> = vec![0xE0]; // |111|00000

    assert_eq!(byte_array_1.len(), 3, "Binary buffer length should be 3");

    assert_eq!(
        result,
        extract_bits(&byte_array_1, 24, 13, 3),
        "Should have correctly extracted 3 bits from binary buffer"
    );
}

#[test]
fn test_extract_bits_3() {
    let byte_array_1: Vec<u8> = vec![0x95, 0xDD, 0xFF, 0xE7]; // 1001010111|0111011111|111111100111
    let result: Vec<u8> = vec![0x77, 0xC0]; //           |0111011111|000000

    assert_eq!(
        byte_array_1.len() * BYTE,
        32,
        "Binary buffer length should be 32 bits"
    );

    assert_eq!(
        result,
        extract_bits(&byte_array_1, 32, 10, 10),
        "Should have correctly extracted 10 bits from binary buffer"
    );
}

#[test]
fn test_extract_bits_4() {
    let byte_array_1: Vec<u8> = vec![0x95, 0xDD, 0xFF, 0xE7]; // 1001010111|0111011111|111111100111
    let result: Vec<u8> = vec![0xDF, 0xFE]; //           |0111011111|000000

    assert_eq!(
        byte_array_1.len() * BYTE,
        32,
        "Binary buffer length should be 10 bits"
    );

    assert_eq!(
        result,
        extract_bits(&byte_array_1, 32, 12, 16),
        "Should have correctly extracted 10 bits from binary buffer"
    );
}

#[test]
fn removes_invalid_hex_characters() {
    let input = common::remove_all_invalid_hex_characters("-i-... HELLO::am::WORLD +val+:.id");
    assert_eq!(input, "i am valid");
}

#[test]
fn remove_hexadecimal_prefix() {
    let input_1 = common::remove_hexadecimal_prefix("0xf2sa0xasd");
    let input_2 = common::remove_hexadecimal_prefix("f2sa0xasd");
    assert_eq!(input_1, "f2sa0xasd");
    assert_eq!(input_2, "f2sa0xasd");
}
