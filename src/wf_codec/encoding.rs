use super::binary::{decode_binary, encode_binary};
use super::common::{remove_all_invalid_hex_characters, shift_left};
use super::constants::*;
use super::hexadecimal::{decode_bdx, encode_bdx};
use super::latlong::encode_latlong;

#[derive(Clone)]
pub struct Encoding {
    pub charset: &'static str,
    pub bit_length: usize,
    pub byte_length: Option<u8>,
    pub kind: EncodingKind,
}

#[derive(Clone)]
pub enum EncodingKind {
    BIN,
    DEC,
    HEX,
    UTF8,
    DATETIME,
    DURATION,
    LAT,
    LONG,
}

impl Encoding {
    fn new(
        charset: &'static str,
        bit_length: usize,
        byte_length: Option<u8>,
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
            EncodingKind::BIN => encode_binary(value),
            EncodingKind::DEC | EncodingKind::HEX => encode_bdx(value),
            EncodingKind::DATETIME | EncodingKind::DURATION => {
                encode_bdx(remove_all_invalid_hex_characters(value))
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
    pub fn decode(&self, buffer: Vec<u8>, bit_length: usize) -> String {
        let mut s = String::new();

        match &self.kind {
            EncodingKind::UTF8 => {
                return String::from_utf8(buffer).expect("utf8 error");
            }
            EncodingKind::BIN => {
                return decode_binary(&buffer, bit_length);
            }
            EncodingKind::DEC | EncodingKind::HEX => {
                return decode_bdx(buffer, bit_length);
            }
            EncodingKind::DATETIME => {
                s.push_str(decode_bdx(buffer, bit_length).as_str());

                s.insert(4, '-');
                s.insert(7, '-');
                s.insert(10, 'T');
                s.insert(13, ':');
                s.insert(16, ':');
                s.insert(19, 'Z');
            }
            EncodingKind::DURATION => {
                s.push_str(decode_bdx(buffer, bit_length).as_str());

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
                s.push_str(decode_bdx(shift_left(&buffer, 1), bit_length - 1).as_str());
                s.insert(s.len() - 5, '.');
            }
        }

        s
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
    pub fn bit_length(&self, byte_length: usize) -> usize {
        if self.is_fixed_length() {
            return self.bit_length;
        }
        byte_length * self.bit_length
    }
}

/**
 * The equivalent of following constants can be found as an enum called "Encoding" in WfMessageCodec.java
 */

pub const BIN: Encoding = Encoding {
    charset: "[01]",
    bit_length: BIT,
    byte_length: None,
    kind: EncodingKind::BIN,
};

pub const DEC: Encoding = Encoding {
    charset: "[0-9]",
    bit_length: QUADBIT,
    byte_length: None,
    kind: EncodingKind::DEC,
};

pub const HEX: Encoding = Encoding {
    charset: "[a-fA-F0-9]",
    bit_length: QUADBIT,
    byte_length: None,
    kind: EncodingKind::HEX,
};

pub const UTF8: Encoding = Encoding {
    charset: r"[\u0000-\u007F]",
    bit_length: OCTET,
    byte_length: None,
    kind: EncodingKind::UTF8,
};

pub const DATETIME: Encoding = Encoding {
    charset: "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
    bit_length: 56,
    byte_length: Some(20),
    kind: EncodingKind::DATETIME,
};

pub const DURATION: Encoding = Encoding {
    charset: "P[0-9]{2}D[0-9]{2}H[0-9]{2}M",
    bit_length: 24,
    byte_length: Some(10),
    kind: EncodingKind::DURATION,
};

pub const LAT: Encoding = Encoding {
    charset: "[+\\-][0-9]{2}\\.[0-9]{5}",
    bit_length: 29,
    byte_length: Some(9),
    kind: EncodingKind::LAT,
};

pub const LONG: Encoding = Encoding {
    charset: "[+\\-][0-9]{3}\\.[0-9]{5}",
    bit_length: 33,
    byte_length: Some(10),
    kind: EncodingKind::LONG,
};

/* protected final WfBinaryBuffer encode() throws WfCoreException {
    WfBinaryBuffer buffer = WfBinaryBuffer.create();
    int byteCursor = fields[0].startByte;
    for (WfMessageField field : fields) {
        if (field.startByte != byteCursor) {
            throw new WfCoreException("Invalid field order while encoding: did not expect field " + field.debugInfo() + " at byte " + byteCursor, null);
        }
        buffer.addMessageField(field);
        byteCursor = field.endByte;
    }
    return buffer;
} */
