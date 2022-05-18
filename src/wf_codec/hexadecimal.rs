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
    std::char::from_digit(data as u32, HEXRADIX as u32).expect("failed to convert to char")
}

/**
 * Decodes a binary buffer into a (hexa)decimal string
 * @since 1.1
 * @param buffer the binary buffer containing the binary encoded (hexa)decimals to decode
 * @param bitLength the buffer length, i.e. the number of bits in the buffer to decode
 * @return a (hexa)decimal string with the decoded data
 * java equivalent: WfMessageCodec.decodeBDX
 */
pub fn decode_bdx(buffer: Vec<u8>, bit_length: usize) -> String {
    let mut hexadecimal_string: Vec<char> = Vec::new();

    for bit_index in (0..bit_length).step_by(BYTE) {
        let byte_cursor = bit_index as usize / BYTE;
        hexadecimal_string.push(from_hex_digit((buffer[byte_cursor] >> QUADBIT) & 0xF));
        if (bit_index + QUADBIT) < bit_length {
            hexadecimal_string.push(from_hex_digit(buffer[byte_cursor] & 0xF));
        }
    }

    hexadecimal_string.into_iter().collect()
}
