use crate::error::CodecResult;

use super::{
    binary::{decode_to_binary, encode_from_binary},
    hexadecimal::{decode_to_bdx, encode_from_bdx},
    latlong::encode_latlong,
};
use wf_common::{
    common::{remove_all_invalid_hex_characters, shift_left},
    constants::*,
};
use wf_validation::{Validation, ValidationError};

#[derive(Clone, Debug)]
pub struct Encoding {
    pub charset: &'static str,
    pub bit_length: usize,
    pub byte_length: Option<usize>,
    pub kind: EncodingKind,
}

/* impl EncodingKind {
    pub fn get_charset(&self) -> &'static str {
        match self {
            EncodingKind::BIN => "[01]",
            _ => "",
        }
    }
} */

impl Encoding {
    fn new(
        charset: &'static str,
        bit_length: usize,
        byte_length: Option<usize>,
        kind: EncodingKind,
    ) -> Encoding {
        Encoding {
            charset,
            bit_length,
            byte_length,
            kind,
        }
    }

    /**
     * Encodes a Whiteflag message field to compressed binary representation
     * @since 1.1
     * @param field the message field to be encoded
     * @return a binary buffer with the encoded field
     * java equivalent: WfMessageCodec.encodeField
     */
    pub fn encode<T: AsRef<str> + std::fmt::Display>(&self, value: T) -> Vec<u8> {
        match &self.kind {
            EncodingKind::UTF8 => value.as_ref().as_bytes().to_vec(),
            EncodingKind::BIN => encode_from_binary(value),
            EncodingKind::DEC | EncodingKind::HEX => encode_from_bdx(value),
            EncodingKind::DATETIME | EncodingKind::DURATION => {
                encode_from_bdx(remove_all_invalid_hex_characters(value))
            }
            EncodingKind::LAT | EncodingKind::LONG => encode_latlong(value),
        }
    }

    /**
     * Sets the field value from a binary buffer
     * @since 1.1
     * @param field the field for which to decode the binary value
     * @param buffer a binary buffer with the compressed binary encoded field data
     * @return the uncompressed value of the field
     * java equivalent: WfMessageCodec.decodeField
     */
    pub fn decode(&self, buffer: &[u8], bit_length: usize) -> CodecResult<String> {
        let mut s = String::new();

        match &self.kind {
            EncodingKind::UTF8 => {
                return Ok(std::str::from_utf8(buffer)?.to_string());
            }
            EncodingKind::BIN => {
                return Ok(decode_to_binary(buffer, bit_length));
            }
            EncodingKind::DEC | EncodingKind::HEX => {
                return Ok(decode_to_bdx(buffer, bit_length));
            }
            EncodingKind::DATETIME => {
                s.push_str(&decode_to_bdx(buffer, bit_length));

                s.insert(4, '-');
                s.insert(7, '-');
                s.insert(10, 'T');
                s.insert(13, ':');
                s.insert(16, ':');
                s.insert(19, 'Z');
            }
            EncodingKind::DURATION => {
                s.push_str(&decode_to_bdx(buffer, bit_length));

                s.insert(0, 'P');
                s.insert(3, 'D');
                s.insert(6, 'H');
                s.insert(9, 'M');
            }
            EncodingKind::LAT | EncodingKind::LONG => {
                let sign = if ((buffer[0] >> (BYTE - 1)) & 1) == 1 {
                    '+'
                } else {
                    '-'
                };

                s.push(sign);
                s.push_str(decode_to_bdx(&shift_left(buffer, 1), bit_length - 1).as_str());
                s.insert(s.len() - 5, '.');
            }
        }

        Ok(s)
    }

    pub fn is_fixed_length(&self) -> bool {
        self.byte_length != None
    }

    /**
     * Returns the bit length of a field for a given encoding and unencoded field byte length
     * @param byteLength the number of bytes in the unencoded field
     * @return the number of bits in a compressed encoded field
     * java equivalent: Encoding.bitLength (WfMessageCodec.java)
     */
    pub fn convert_to_bit_length(&self, byte_length: usize) -> usize {
        if self.is_fixed_length() {
            return self.bit_length;
        }
        byte_length * self.bit_length
    }
}

/**
 * The equivalent of following constants can be found as an enum called "Encoding" in WfMessageCodec.java
 */

macro_rules! encoding {
    (
        $( $name:ident, $charset:expr, $bit_length:expr, $byte_length:expr );*
    ) => {
        #[derive(Clone, Copy, Debug)]
        pub enum EncodingKind {
            $(
                $name,
            )*
        }

        impl EncodingKind {
            pub fn get_encoding(&self) -> Encoding {
                match &self {
                    $( EncodingKind::$name => $name, )*
                }
            }
        }

        impl Validation for Encoding {
            fn validate(&self, value: &str) -> Result<(), ValidationError> {
                match self.byte_length {
                    Some(x) if value.len() != x => return Err(ValidationError::InvalidLength {
                        data: value.to_string(),
                        expected_length: x,
                        specification_level: format!("== Encoding Error for {:?} ==", self.kind)
                    }),
                    _ => (),
                };

                if match self.kind {
                    $( EncodingKind::$name => rx::$name.is_match(value), )*
                } == false {
                    return Err(ValidationError::InvalidCharset);
                }

                Ok(())
            }
        }

        $( pub const $name: Encoding = Encoding {
            charset: charsets::$name,
            bit_length: $bit_length,
            byte_length: $byte_length,
            kind: EncodingKind::$name
        }; )*

        pub mod charsets {
            $( pub const $name: &'static str = $charset; )*
        }

        pub mod rx {
            use regex::Regex;
            lazy_static!{
                $( pub static ref $name: Regex = Regex::new(super::charsets::$name).unwrap(); )*
            }
        }
    };
}

encoding!(
    BIN, "[01]", BIT, None;
    DEC, "[0-9]", QUADBIT, None;
    HEX, "[a-fA-F0-9]", QUADBIT, None;
    UTF8, r"[\u0000-\u007F]", OCTET, None;
    DATETIME, "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z", 56, Some(20);
    DURATION, "P[0-9]{2}D[0-9]{2}H[0-9]{2}M", 24, Some(10);
    LAT, "[+\\-][0-9]{2}\\.[0-9]{5}", 29, Some(9);
    LONG, "[+\\-][0-9]{3}\\.[0-9]{5}", 33, Some(10)
);
