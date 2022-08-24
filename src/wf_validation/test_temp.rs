use crate::{
    wf_buffer::constants::{BYTE, OCTET, QUADBIT},
    wf_codec::encoding,
};
//usize::from_str_radix(src, radix)
#[test]
fn test_bin() {
    assert_eq!(187, usize::from_str_radix("10111011", 2).unwrap());
    assert_eq!(128, usize::from_str_radix("10000000", 2).unwrap());
    assert_eq!(160, usize::from_str_radix("10100000", 2).unwrap());
    assert_eq!(64, usize::from_str_radix("01000000", 2).unwrap());

    assert_eq!(vec![187], encoding::BIN.encode("10111011"));
    assert_eq!(vec![128], encoding::BIN.encode("10000000"));
    assert_eq!(vec![160], encoding::BIN.encode("10100000"));
    assert_eq!(vec![64], encoding::BIN.encode("01000000"));

    // WF BIN encode expands string to 8 bits by adding zeroes to end
    assert_eq!(vec![187], encoding::BIN.encode("10111011"));
    assert_eq!(vec![128], encoding::BIN.encode("1"));
    assert_eq!(vec![160], encoding::BIN.encode("101"));
    assert_eq!(vec![64], encoding::BIN.encode("01"));

    assert_eq!("10111011", encoding::BIN.decode(vec![187], 8));
    assert_eq!("10000000", encoding::BIN.decode(vec![128], 8));
    assert_eq!("10100000", encoding::BIN.decode(vec![160], 8));
    assert_eq!("01000000", encoding::BIN.decode(vec![64], 8));

    assert_eq!("10", encoding::BIN.decode(vec![187], 2));
    assert_eq!("10", encoding::BIN.decode(vec![128], 2));
    assert_eq!("10", encoding::BIN.decode(vec![160], 2));
    assert_eq!("01", encoding::BIN.decode(vec![64], 2));

    assert_eq!(vec![187, 187], encoding::BIN.encode("1011101110111011"));
    assert_eq!(vec![187, 64], encoding::BIN.encode("1011101101"));

    assert_eq!("1011101110111011", encoding::BIN.decode(vec![187, 187], 16));
    assert_eq!("1011101101000000", encoding::BIN.decode(vec![187, 64], 16));

    println!("{:?}", b"01000000");
}

#[test]
fn test_hex() {
    assert_eq!("31", hex::encode("1"));
    assert_eq!("32", hex::encode("2"));

    assert_eq!("31", encoding::HEX.decode(b"1".to_vec(), OCTET));
    assert_eq!("32", encoding::HEX.decode(b"2".to_vec(), OCTET));

    assert_eq!(b"1".to_vec(), hex::decode("31").unwrap());
    assert_eq!(b"2".to_vec(), hex::decode("32").unwrap());

    assert_eq!(b"1".to_vec(), encoding::HEX.encode("31"));
    assert_eq!(b"2".to_vec(), encoding::HEX.encode("32"));

    assert_eq!(vec![16], encoding::HEX.encode("1"));
    assert_eq!(vec![51, 49], hex_as_bytes("1"));
    assert_eq!(vec![49], hex::decode([51, 49]).unwrap());
    assert_eq!(vec![49], b"1");

    for s in vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "a", "b",
        "c", "d", "e", "f",
    ] {
        display_hex(s);
    }
}

fn hex_as_bytes(value: &str) -> Vec<u8> {
    let mut hex_buffer: Vec<u8> = vec![0; value.len() * 2];
    hex::encode_to_slice(value, &mut hex_buffer);
    hex_buffer
}

fn display_hex(value: &str) {
    let buffer = encoding::HEX.encode(value);
    let hex_buffer: Vec<u8> = hex_as_bytes(value);
    println!(
        r"
    value {}
    == UTF8 Encoding ==
        {}
    == WF Encoding ==
        {}
    == Hex Encoding ==
        {}
    ",
        value,
        display_bytes(value.as_bytes(), 8),
        display_bytes(&buffer, 4),
        display_bytes(&hex_buffer, 16)
    )
}

fn display_bytes(bytes: &[u8], bit_length: usize) -> String {
    format!(
        r"bytes {:?}
        binary {}",
        bytes,
        encoding::BIN.decode(bytes.to_vec(), bit_length)
    )
}
