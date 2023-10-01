use crate::error::HexDecodeError;
use wf_common::constants::QUADBIT;

/// encodes a hexadecimal string into a binary buffer
///
/// the input string may have an odd length, and that should be fixed by adding a zero
///
/// java: WfMessageCodec.encodeBDX
pub fn encode_from_bdx<T: AsRef<str>>(hex: T) -> Vec<u8> {
    let mut data = hex.as_ref().to_string();

    if data.len() % 2 == 1 {
        data.push('0');
    }

    hex::decode(data).unwrap()
}

fn from_hex_digit(data: u8) -> Result<char, HexDecodeError> {
    match data {
        0..=9 => Ok((data + b'0') as char),
        10 => Ok('a'),
        11 => Ok('b'),
        12 => Ok('c'),
        13 => Ok('d'),
        14 => Ok('e'),
        15 => Ok('f'),
        16.. => Err(HexDecodeError::InvalidHexDigit(data as char)),
    }
}

/// decodes binary buffer into hexadecimal
///
/// the input buffer will have a bit_length multiple of 4 and, therefore, is not valid for hex::encode
///
/// if the bit_length is not a multiple of 8, then the last byte is only split once, and `b & 0xF` is ignored
///
/// java: WfMessageCodec.decodeBDX
pub fn decode_to_bdx(buffer: &[u8], bit_length: usize) -> Result<String, HexDecodeError> {
    (0..(bit_length / QUADBIT))
        .map(|i| {
            // traverse buffer by 2
            let x = i % 2;
            let byte_index = (i - x) / 2;
            let b = buffer[byte_index];
            if x == 0 {
                (b >> QUADBIT) & 0xF
            } else {
                b & 0xF
            }
        })
        .map(from_hex_digit)
        .collect()
}
