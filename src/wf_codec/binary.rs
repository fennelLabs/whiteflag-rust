use super::constants::BYTE;

/**
 * Encodes a binary string into a binary buffer
 * @since 1.1
 * @param binstr the binary string to encode
 * @return a binary buffer containing the bits from the binary string
 * java equivalent: WfMessageCodec.encodeBIN
 */
pub fn encode_binary<T: AsRef<str>>(binary_str: T) -> Vec<u8> {
    let bit_length: usize = binary_str.as_ref().len();
    let byte_length: usize = (bit_length / BYTE)
        + (match bit_length % BYTE == 0 {
            true => 0,
            false => 1,
        });
    let mut buffer = vec![0; byte_length];

    for bit_index in 0..bit_length {
        if binary_str
            .as_ref()
            .chars()
            .nth(bit_index)
            .expect("something wrong")
            == '1'
        {
            let byte_cursor: usize = bit_index / BYTE;
            let bit_position: usize = bit_index % BYTE;
            buffer[byte_cursor] |= 0x80 >> bit_position;
        }
    }

    buffer
}

/**
 * Decodes a binary buffer into a binary string
 * @since 1.1
 * @param buffer the binary buffer to decode
 * @param bitLength the buffer length, i.e. the number of bits in the buffer to decode
 * @return a binary string containing the bits from the binary buffer
 * java equivalent: WfMessageCodec.decodeBIN
 */
pub fn decode_binary(buffer: &[u8], bit_length: usize) -> String {
    let mut data: Vec<char> = Vec::new();

    for bit_index in 0..bit_length {
        let byte_cursor: usize = bit_index / BYTE;
        let bit_position: usize = bit_index % BYTE;

        if (buffer[byte_cursor] >> (BYTE - bit_position - 1) & 1) == 1 {
            data.push('1');
        } else {
            data.push('0');
        }
    }

    /* TODO: may need to convert characters to lower per java implementation */
    data.into_iter().collect()
}
