use super::constants::*;

/**
 * removes characters from string that are invalid in hexadecimal format
 * java equivalent: N/A
 */
pub fn remove_all_invalid_hex_characters<T: AsRef<str>>(data: T) -> String {
    let re = regex::Regex::new("[-+:.A-Z]").unwrap();
    re.replace_all(data.as_ref(), "").to_string()
}

pub fn remove_hexadecimal_prefix(data: &str) -> &str {
    if data.starts_with("0x") {
        return &data[2..];
    }

    data
}

pub fn remove_hexadecimal_prefix_mut(mut data: &str) {
    data = remove_hexadecimal_prefix(data);
}

/**
 * Calculates the number of bytes required to hold the given number of bits
 * java equivalent: WfBinaryBuffer.byteLength
 */
pub fn byte_length(bit_length: usize) -> usize {
    let i_byte = BYTE;
    (bit_length / i_byte) + (if (bit_length % i_byte) > 0 { 1 } else { 0 })
}

/**
 * Shortens the byte array to fit the length of the used bits
 * java equivalent: WfBinaryBuffer.cropBits
 */
pub fn crop_bits(buffer: &mut Vec<u8>, bit_length: usize) {
    if bit_length == 0 {
        return;
    }

    let buffer_length = buffer.len();
    let byte_len = byte_length(bit_length);
    buffer.drain(byte_len..);

    if byte_len > buffer_length {
        return;
    }

    /* Clear unused bits in last byte */
    let clear_bits = BYTE - (bit_length % BYTE);
    if !(clear_bits < BYTE) {
        return;
    }

    if let Some(x) = buffer.last_mut() {
        *x &= 0xFF << clear_bits;
    }
}

/**
 * Shifts bits in a byte array to the right modulo 8
 * java equivalent: WfBinaryBuffer.shiftRight
 */
pub fn shift_right(buffer: &[u8], shift: isize) -> Vec<u8> {
    if shift < 0 {
        return shift_left(buffer, -shift);
    }

    let modulate: usize = shift as usize % BYTE;

    if modulate == 0 {
        return buffer.to_vec();
    }

    let mask: u8 = 0xFF >> (BYTE - modulate);
    let length = buffer.len() + 1;
    let mut new_buffer = vec![0; length];

    for i in (1..length).rev() {
        let b = &buffer[i - 1];
        new_buffer[i] |= (0xFF & b & mask) << (BYTE - modulate);
        new_buffer[i - 1] = (0xFF & b) >> modulate;
    }

    new_buffer
}

/**
 * Shifts bits in a byte array to the left modulo 8
 * java equivalent: WfBinaryBuffer.shiftLeft
 */
pub fn shift_left(buffer: &[u8], shift: isize) -> Vec<u8> {
    if shift < 0 {
        return shift_right(buffer, -shift);
    }

    let modulate: usize = shift as usize % BYTE;

    if modulate == 0 {
        return buffer.to_vec();
    }

    let mask: u8 = 0xFF << (BYTE - modulate);
    let length = buffer.len();
    let mut new_buffer = vec![0; length];

    for i in 0..length {
        new_buffer[i] = (0xFF & buffer[i]) << modulate;
        if i < length - 1 {
            new_buffer[i] |= (0xFF & buffer[i + 1] & mask) >> (BYTE - modulate);
        }
    }

    new_buffer
}

/**
 * Returns a byte array with a subset of the bits in the buffer
 * @param startBit the first bit of the subset to extract
 * @param bitLength the length of the subset, i.e. the number of bits to extract
 * @return a byte array with the extracted bits
 * java equivalent: WfBinaryBuffer.extractBits
 */
pub fn extract_bits(
    buffer: &[u8],
    buffer_bit_length: usize,
    start_bit: usize,
    mut bit_length: usize,
) -> Vec<u8> {
    if bit_length < 1 || bit_length > (buffer_bit_length - start_bit) {
        bit_length = buffer_bit_length - start_bit;
    }

    let start_byte = start_bit / BYTE;
    let byte_length = byte_length(bit_length);
    let shift = start_bit % BYTE;
    let mask: u8 = (BYTE - shift).checked_shl(0xFF).unwrap_or(u8::MAX as usize) as u8;

    let mut new_byte_array: Vec<u8> = vec![0; byte_length];
    if shift == 0 {
        /* Faster loop if no shift needed */
        for byte_index in 0..byte_length {
            new_byte_array[byte_index] = buffer[start_byte + byte_index];
        }
    } else {
        /* Loop through bytes to shift */
        for byte_index in 0..byte_length {
            new_byte_array[byte_index] = (0xFF & buffer[start_byte + byte_index]) << shift;
        }

        let end_byte = if byte_length < (buffer.len() - start_byte) {
            byte_length
        } else {
            byte_length - 1
        };

        for byte_index in 0..end_byte {
            new_byte_array[byte_index] |=
                (0xFF & buffer[start_byte + byte_index + 1] & mask) >> (BYTE - shift);
        }
    }

    crop_bits(new_byte_array.as_mut(), bit_length);
    new_byte_array
}

/**
 * Appends the specified number of bits from a bytes array to the binary buffer
 * @param byteArray the byte array with the bits to be appended
 * @param nBits the number of bits to be appended from the byte array
 * @return this binary buffer
 * @throws IllegalStateException if the buffer is marked complete and cannot be altered
 * java equivalent: WfBinaryBuffer.appendBits
 */
pub fn append_bits(
    buffer_1: &[u8],
    len_1: usize,
    buffer_2: &[u8],
    mut len_2: usize,
) -> (Vec<u8>, usize) {
    /* Check number of bits */
    let max_number_of_bits = buffer_2.len() * BYTE;
    if len_2 > max_number_of_bits {
        len_2 = max_number_of_bits;
    }

    /* Add bits to the end of the buffer */
    let new_buffer = concatinate_bits(&buffer_1, len_1, &buffer_2, len_2);

    (new_buffer, len_1 + len_2)
}

/**
 * Concatinates two bitsets
 * @param byte_array_1 byte array containing the first bitset
 * @param n_bits_1 number of bits in the first bitset, i.e. which bits to take from the first byte array
 * @param byte_array_2 byte array containing the second bitset
 * @param n_bits_2 number of bits in the second bitset, i.e. which bits to take from the second byte array
 * @return a new byte array with the concatinated bits
 * java equivalent: WfBinaryBuffer.concatinateBits
 */
pub fn concatinate_bits(
    byte_array_1: &[u8],
    mut n_bits_1: usize,
    byte_array_2: &[u8],
    mut n_bits_2: usize,
) -> Vec<u8> {
    /* Check number of bits */
    if n_bits_1 > (byte_array_1.len() * BYTE) {
        n_bits_1 = byte_array_1.len() * BYTE;
    }

    if n_bits_2 > (byte_array_2.len() * BYTE) {
        n_bits_2 = byte_array_2.len() * BYTE;
    }

    /* Calculate parameters */
    let shift = n_bits_1 % BYTE;
    let free_bits = if shift == 0 { 0 } else { BYTE - shift };
    let byte_length_1 = (n_bits_1 / BYTE) + (if free_bits == 0 { 0 } else { 1 });
    let bit_length = n_bits_1 + n_bits_2;
    let byte_length = byte_length(bit_length);

    /* Prepare byte arrays */
    let byte_array_2_shift = shift_right(&byte_array_2, shift as isize);
    let mut new_byte_array = vec![0; byte_length as usize];

    /* Concatination */
    let mut byte_cursor = 0;
    let mut start_byte_2 = 0;
    if byte_length_1 != 0 {
        /* Add first byte array */
        for byte_index in 0..byte_length_1 {
            byte_cursor = byte_index;
            new_byte_array[byte_cursor] = byte_array_1[byte_index];
        }

        /* Add overlapping byte from second byte array*/
        if free_bits > 0 {
            new_byte_array[byte_cursor] |= byte_array_2_shift[0];
            start_byte_2 = 1;
        }
        byte_cursor += 1;
    }
    /* Add the rest of the second byte array */
    let end_byte_2 = start_byte_2 + byte_length - byte_cursor;

    for byte_index in start_byte_2..end_byte_2 {
        new_byte_array[byte_cursor] = byte_array_2_shift[byte_index];
        byte_cursor += 1;
    }

    crop_bits(new_byte_array.as_mut(), bit_length);
    new_byte_array
}
