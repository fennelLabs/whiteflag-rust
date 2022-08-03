use super::constants::{BYTE, HEXRADIX, QUADBIT};

/**
 * Encodes a (hexa)decimal string into a binary buffer
 * @since 1.1
 * @param bdxstr the (hexa)decimal string to encode
 * @return a binary buffer containing the encoded (hexa)decimal string
 * java equivalent: WfMessageCodec.encodeBDX
 */
pub fn encode_bdx<T: AsRef<str>>(raw_input_str: T) -> Vec<u8> {
    let mut data: Vec<char> = raw_input_str.as_ref().chars().collect();
    if data.len() % 2 == 1 {
        data.push('0');
    }

    let input_length = data.len();
    let mut buffer = vec![0; input_length / 2];

    for i in (0..input_length).step_by(2) {
        let digit0: u8 = to_hex_digit(data.get(i));
        let digit1: u8 = to_hex_digit(data.get(i + 1));
        buffer[i / 2] = (digit0 << QUADBIT) + digit1;
    }

    buffer
}

/**
 * java equivalent: N/A
 */
fn to_hex_digit(data: Option<&char>) -> u8 {
    u8::from_str_radix(&data.expect("out of bounds").to_string(), HEXRADIX as u32)
        .expect("failed to convert to digit")
}

/**
 * java equivalent: N/A
 */
fn from_hex_digit(data: u8) -> char {
    match data {
        0..=9 => (data + b'0') as char,
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        16.. => panic!("invalid data"),
    }
}

/**
 * Decodes a binary buffer into a (hexa)decimal string
 * @since 1.1
 * @param buffer the binary buffer containing the binary encoded (hexa)decimals to decode
 * @param bitLength the buffer length, i.e. the number of bits in the buffer to decode
 * @return a (hexa)decimal string with the decoded data
 * java equivalent: WfMessageCodec.decodeBDX
 */
pub fn decode_bdx(buffer: &[u8], bit_length: usize) -> String {
    (0..(bit_length / QUADBIT))
        .map(|i| {
            let x = i % 2;
            let b = buffer[(i - x) / 2];
            if x == 0 {
                (b >> QUADBIT) & 0xF
            } else {
                b & 0xF
            }
        })
        .map(from_hex_digit)
        .collect()
}
