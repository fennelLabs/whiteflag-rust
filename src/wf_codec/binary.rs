use super::constants::BYTE;

/// encodes a binary string into a binary buffer
///
/// java: WfMessageCodec.encodeBIN
pub fn encode_from_binary<T: AsRef<str>>(binary_str: T) -> Vec<u8> {
    let binary = binary_str.as_ref();
    let bit_length: usize = binary.len();
    let byte_length: usize = (bit_length / BYTE) + (if bit_length % BYTE == 0 { 0 } else { 1 });

    let mut buffer = vec![0; byte_length];

    binary
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == ONE)
        .for_each(|(i, _)| {
            let byte_cursor: usize = i / BYTE;
            let bit_position: usize = i % BYTE;
            buffer[byte_cursor] |= 0x80 >> bit_position;
        });

    buffer
}

const ZERO: char = '0';
const ONE: char = '1';

/// decodes a binary buffer into a binary string
///
/// java: WfMessageCodec.decodeBIN
pub fn decode_to_binary(buffer: &[u8], bit_length: usize) -> String {
    let mut data = String::new();

    for bit_index in 0..bit_length {
        let byte_cursor: usize = bit_index / BYTE;
        let bit_position: usize = bit_index % BYTE;

        if (buffer[byte_cursor] >> (BYTE - bit_position - 1) & 1) == 1 {
            data.push(ONE);
        } else {
            data.push(ZERO);
        }
    }

    data
}
