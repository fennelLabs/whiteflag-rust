use super::*;

#[test]
fn test_append_bits_1() {
    let byte_array_1: Vec<u8> = vec![0xE6, 0x38, 0x87]; // 1110 0110 | 0011 1000 | 1000 0111
    let byte_array_2: Vec<u8> = vec![0x6E, 0x7f]; // 0110 1110 | 0111 1111
    let mut buffer = WhiteflagBuffer::default();

    assert_eq!(
        buffer.bit_length(),
        0,
        "Binary buffer length should be 0 bits"
    );

    buffer.append(byte_array_1.into(), Some(22)); // 1110 0110 | 0011 1000 | 1000 01(00)
    assert_eq!(
        buffer.bit_length(),
        22,
        "Binary buffer length should be 22 bits"
    );
    assert_eq!(
        "e63884",
        buffer.as_hex(),
        "Byte array 1 should have been correctly added to the binary buffer"
    );

    buffer.append(byte_array_2.into(), Some(13)); // 1110 0110 | 0011 1000 | 1000 0101 | 1011 1001 | 1110 0000
    assert_eq!(
        buffer.bit_length(),
        35,
        "Binary buffer length should be 35 bits"
    );
    assert_eq!(
        "e63885b9e0",
        buffer.as_hex(),
        "Byte array 2 should have been correctly added to the binary buffer"
    );
}

#[test]
fn test_append_bits_2() {
    let byte_array_1: Vec<u8> = vec![0xE6, 0x38, 0x87]; // 1110 0110 | 0011 1000 | 1000 0111
    let byte_array_2: Vec<u8> = vec![0x6E, 0x6f]; // 0110 1110 | 0111 1111

    let mut buffer: WhiteflagBuffer = byte_array_1.into();

    assert_eq!(
        buffer.bit_length(),
        24,
        "Binary buffer length should be 24 bits"
    );
    assert_eq!(
        buffer.as_hex(),
        "e63887",
        "Byte array 1 should have been correctly added to the binary buffer"
    );

    buffer.append(byte_array_2.into(), Some(12));

    assert_eq!(
        buffer.bit_length(),
        36,
        "Binary buffer length should be 36 bits"
    );
    assert_eq!(
        buffer.as_hex(),
        "e638876e60",
        "Byte array 2 should have been correctly added to the binary buffer"
    );
}

#[test]
fn test_append_bits_3() {
    let byte_array_1: Vec<u8> = vec![0xDD, 0xFF]; // 1101 1101 | 1111 1111
    let byte_array_2: Vec<u8> = vec![0xBF]; // 1011 1111
    let mut buffer = WhiteflagBuffer::default();

    assert_eq!(
        buffer.bit_length(),
        0,
        "Binary buffer length should be 0 bits"
    );

    buffer.append(byte_array_1.into(), Some(4));
    assert_eq!(
        buffer.bit_length(),
        4,
        "Binary buffer length should be 4 bits"
    );
    assert_eq!(
        buffer.as_hex(),
        "d0",
        "Byte array 1 should have been correctly added to the buffer"
    );

    buffer.append(byte_array_2.into(), Some(3));
    assert_eq!(
        buffer.bit_length(),
        7,
        "Binary buffer length should be 7 bits"
    );
    assert_eq!(
        buffer.as_hex(),
        "da",
        "Byte array 2 should have been correctly added to the buffer"
    );
}
