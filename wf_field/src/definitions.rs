use super::field_definition::*;
use wf_codec::encoding::{ConfiguredByteLength, Encoding};

pub fn get_body_from_code(code: &str) -> Vec<FieldDefinition> {
    get_body_from_code_char(&convert_value_to_code(code)).to_vec()
}

pub fn get_body_from_code_char(code: &char) -> Vec<FieldDefinition> {
    match code {
        'A' => Authentication::DEFINITIONS,
        'K' => Crypto::DEFINITIONS,
        'T' => Test::DEFINITIONS,
        'R' => Resource::DEFINITIONS,
        'F' => FreeText::DEFINITIONS,
        'P' | 'E' | 'D' | 'S' | 'I' | 'M' | 'Q' => Sign::DEFINITIONS,
        _ => panic!("'{}' is not a valid message code", code),
    }
    .to_vec()
}

/// fields that are codes are single characters
pub fn convert_value_to_code(value: &str) -> char {
    value
        .chars()
        .nth(0)
        .unwrap_or_else(|| panic!("invalid message code: {}", value))
}

pub fn generic_header_fields() -> &'static [FieldDefinition] {
    Header::DEFINITIONS
}

pub fn message_code() -> &'static FieldDefinition {
    &Header::MESSAGE_CODE
}

pub fn test_message_code() -> &'static FieldDefinition {
    &Test::PSEUDO_MESSAGE_CODE
}

pub enum FieldKind {
    GENERIC,
    AUTHENTICATION,
    CRYPTO,
    TEXT,
    RESOURCE,
    TEST,
    SIGNAL,
    REQUEST,
}

pub struct ParsedFieldDefinition {
    definition: &'static FieldDefinition,
    pub start_bit: usize,
    pub end_bit: usize,
    index: usize,
}

impl ParsedFieldDefinition {
    /// creates the `ParsedFieldDefinition` that is ordered after this one
    pub fn next(&self, next: &'static FieldDefinition) -> Self {
        ParsedFieldDefinition::new(self.index, self.end_bit, next)
    }

    pub fn new(index: usize, previous_end_bit: usize, current: &'static FieldDefinition) -> Self {
        let start_bit = previous_end_bit;
        let end_bit = start_bit + current.bit_length();
        ParsedFieldDefinition {
            definition: current,
            start_bit,
            end_bit,
            index,
        }
    }

    pub fn parse(defs: &'static [FieldDefinition], start: usize) -> Vec<ParsedFieldDefinition> {
        let mut previous = start;
        defs.iter()
            .enumerate()
            .map(|(i, d)| {
                let p = ParsedFieldDefinition::new(i, previous, d);
                previous = p.end_bit;
                p
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct BytePositions {
    pub start: usize,
    /// most fields will have an end byte, but some are unbounded
    pub end: Option<usize>,
    /// (end - start) unless the encoding has a fixed byte length or it is the last field and isn't bounded (end = 0 or None)
    length: ConfiguredByteLength,
    encoding: Encoding,
    /// some encodings will have a fixed byte length
    is_fixed: bool,
    /// (length * encoding.bit_length) unless is_fixed is true, then it is equal to encoding.bit_length
    bit_length: usize,
}

impl BytePositions {
    /* pub fn new_from_opt(start: usize, end: Option<usize>) -> Self {
        Self { start, end, length: end.and_then(|e| Some(e - start)).or(None)}
    } */

    pub const fn new(start: usize, end: usize, encoding: Encoding) -> Self {
        let (end, length) = if end == 0 {
            (None, encoding.byte_length)
        } else {
            (Some(end), ConfiguredByteLength::new(end - start))
        };
        let is_fixed = encoding.byte_length.is_fixed();
        let bit_length = if is_fixed {
            encoding.bit_length
        } else {
            length.as_usize() * encoding.bit_length
        };
        Self {
            start,
            end,
            length,
            encoding,
            is_fixed,
            bit_length,
        }
    }

    pub const fn next() {}
}

struct BitPosition {}

macro_rules! message_fields {
    (
        $(
            define $group:ident
            $( $name:ident, $upp:ident, $pat:expr, $encoding:ident, $start:expr, $end:expr );*
        )*
    ) => {

        $(
            pub mod $group {
                use super::*;

                pub mod names {
                    $( pub const $upp: &str = stringify!($name); )*
                }

                $( pub const $upp: FieldDefinition = FieldDefinition {
                    name: Some(names::$upp),
                    encoding: wf_codec::encoding::$encoding,
                    positions: BytePositions::new($start, $end, wf_codec::encoding::$encoding)
                }; )*

                pub const DEFINITIONS: &'static [FieldDefinition] = &[$( $upp, )*];

                enum MessageFields {
                    $(
                        $name,
                    )*
                }

                pub mod rx {
                    use regex::Regex;
                    lazy_static!{
                        $( pub static ref $upp: Regex = Regex::new($pat).unwrap(); )*
                    }
                }
            }
        )*
    }
}

message_fields!(
    define Header
    Prefix, PREFIX, "^WF$", UTF8, 0, 2;
    Version, VERSION, "^[A-Z0-9]{1}$", UTF8, 2, 3;
    EncryptionIndicator, ENCRYPTION_INDICATOR, "^[A-Z0-9]{1}$", UTF8, 3, 4;
    DuressIndicator, DURESS_INDICATOR, "^[0-1]{1}$", BIN, 4, 5;
    MessageCode, MESSAGE_CODE, "^[A-Z]{1}$", UTF8, 5, 6;
    ReferenceIndicator, REFERENCE_INDICATOR, "^[a-fA-F0-9]{1}$", HEX, 6, 7;
    ReferencedMessage, REFERENCED_MESSAGE, "^[a-fA-F0-9]{64}$", HEX, 7, 71

    define Authentication
    VerificationMethod, VERIFICATION_METHOD, "(?=1|2)^[a-fA-F0-9]{1}$", HEX, 71, 72;
    VerificationData, VERIFICATION_DATA, r"^[\u0000-\u007F]*$", UTF8, 72, 0

    define Crypto
    CryptoDataType, CRYPTO_DATA_TYPE, "^[a-fA-F0-9]{2}$", HEX, 71, 73;
    CryptoData, CRYPTO_DATA, "^[a-fA-F0-9]*$", HEX, 73, 0

    define FreeText
    Text, TEXT, r"^[\u0000-\u007F]*$", UTF8, 71, 0

    define Resource
    ResourceMethod, RESOURCE_METHOD, "(?=1)^[a-fA-F0-9]{1}$", HEX, 71, 72;
    ResourceData, RESOURCE_DATA, r"^[\u0000-\u007F]*$", UTF8, 72, 0

    define Test
    PseudoMessageCode, PSEUDO_MESSAGE_CODE, "^[A-Z]{1}$", UTF8, 71, 72

    define Sign
    SubjectCode, SUBJECT_CODE, "^[a-fA-F0-9]{2}$", HEX, 71, 73;
    DateTime, DATE_TIME, "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$", DATETIME, 73, 93;
    Duration, DURATION, "^P[0-9]{2}D[0-9]{2}H[0-9]{2}M$", DURATION, 93, 103;
    ObjectType, OBJECT_TYPE, "^[a-fA-F0-9]{2}$", HEX, 103, 105;
    ObjectLatitude, OBJECT_LATITUDE, "^[+\\-][0-9]{2}\\.[0-9]{5}$", LAT, 105, 114;
    ObjectLongitude, OBJECT_LONGITUDE, "^[+\\-][0-9]{3}\\.[0-9]{5}$", LONG, 114, 124;
    ObjectSizeDim1, OBJECT_SIZE_DIM_1, "^[0-9]{4}$", DEC, 124, 128;
    ObjectSizeDim2, OBJECT_SIZE_DIM_2, "^[0-9]{4}$", DEC, 128, 132;
    ObjectOrientation, OBJECT_ORIENTATION, "^[0-9]{3}$", DEC, 132, 135

    define Request
    ObjectType, OBJECT_TYPE, "^[a-fA-F0-9]{2}$", HEX, 135, 137;
    ObjectTypeQuant, OBJECT_TYPE_QUANT, "^[0-9]{2}$", DEC, 137, 139
);

/* /// returns the byte length of the unencoded field value
/// if the field definition does not have a fixed length then it will return `0`
const fn byte_length(start: usize, end: Option<usize>) -> Option<usize> {
    match end {
        Some(e) if e > 0 && e > start => Some(e - start),
        _ => None,
    }
}

/**
 * Gets the bit length of the encoded field
 * @return the bit length of the compressed encoded field value
 */
const fn bit_length(encoding: Encoding, byte_length: Option<usize>) -> usize {
    return encoding
        .convert_to_bit_length(byte_length.unwrap_or(0));
}

/**
 * Returns the bit length of a field for a given encoding and unencoded field byte length
 * @param byteLength the number of bytes in the unencoded field
 * @return the number of bits in a compressed encoded field
 * java equivalent: Encoding.bitLength (WfMessageCodec.java)
 */
const fn convert_to_bit_length(encoding: &'static Encoding, byte_length: usize) -> usize {
    if encoding.byte_length != None {
        return encoding.bit_length;
    }

    encoding.byte_length.map(|_| encoding.bit_length);

    byte_length * encoding.bit_length
} */
