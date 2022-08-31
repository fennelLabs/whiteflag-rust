use super::hexadecimal::encode_from_bdx;
use wf_common::{
    common::{crop_bits, remove_all_invalid_hex_characters, shift_right},
    constants::QUADBIT,
};

const PLUS: &'static str = "+";
const MINUS: &'static str = "-";

/**
 * Encodes a datum string into binary buffer
 * @since 1.1
 * @param datumstr the datum string to encode
 * @return a binary buffer containing the encoded datum
 * java equivalent: WfMessageCodec.encodeLatLong
 */
pub fn encode_latlong<T: AsRef<str>>(data: T) -> Vec<u8> {
    let cleaned_input = remove_all_invalid_hex_characters(&data);
    let bit_length = 1 + cleaned_input.len() * QUADBIT;
    let mut buffer = encode_from_bdx(cleaned_input);

    let sign = &data.as_ref()[0..1];

    if sign == MINUS {
        buffer = shift_right(&buffer, 1);
    }

    if sign == PLUS {
        buffer = shift_right(&buffer, 1);
        buffer[0] |= 0x80;
    }

    crop_bits(buffer.as_mut(), bit_length);
    buffer
}
