use super::common::{crop_bits, remove_all_invalid_hex_characters, shift_right};
use super::constants::QUADBIT;
use super::hexadecimal::encode_bdx;

/**
 * Encodes a datum string into binary buffer
 * @since 1.1
 * @param datumstr the datum string to encode
 * @return a binary buffer containing the encoded datum
 * java equivalent: WfMessageCodec.encodeLatLong
 */
pub fn encode_latlong<T: AsRef<str>>(data: T) -> Vec<u8> {
    let input = data.as_ref();
    let cleaned_input = remove_all_invalid_hex_characters(&input);
    let length = &cleaned_input.len();
    let mut buffer = encode_bdx(cleaned_input);

    if &input[0..1] == "-" {
        buffer = shift_right(&buffer, 1);
    }

    if &input[0..1] == "+" {
        buffer = shift_right(&buffer, 1);
        buffer[0] |= 0x80;
    }

    let bit_length = 1 + length * QUADBIT;
    crop_bits(buffer, bit_length as isize)
}
