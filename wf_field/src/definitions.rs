use crate::{
    byte_configuration::ByteConfiguration, codec_positions::CodecPositions, FieldDefinition,
};
use count_macro::count;
use paste::paste;
use seq_macro::seq;
use wf_codec::encoding::{ByteLength, Encoding};

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

/* pub const fn const_convert(i: usize, position: CodecPositions, configuration: &'static [ByteConfiguration]) -> CodecPositions {
    let current = configuration[i];
    let begin = current.to_position(0);

    position.next(config[i])

    if i == 0 {
        return begin;
    }

    if let Some(p) = position {
        return const_convert(i, configuration, Some(p.next(current)));
    } else {
        return begin;
    }

    panic!("recursive solution failed!");
} */

/* pub const fn next_position(current_index: usize, config: ByteConfiguration, prev: Option<ByteConfiguration>) -> CodecPositions {
    let mut begin = CodecPositions::start(config);
    if current_index == 0 {
        return begin;
    }

    if let Some(p) = prev {
        return CodecPositions::new(config, p.);
    }



    let mut i = 0;
    let mut position = 0;
    while i < current_index {
        //position += *all_config[i].bit_length();
        begin = begin.next(*all_config[i]);
        i += 1;
    }
    begin
} */

pub fn convert(configured_byte_positions: &[ByteConfiguration]) -> Vec<CodecPositions> {
    configured_byte_positions[1..].into_iter().fold(
        vec![configured_byte_positions[0].to_position(0)],
        |mut a, i| {
            if let Some(c) = a.last() {
                a.push(c.next(*i));
            }

            a
        },
    )
}

// declarative macros
// fragment specs: https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html
// https://docs.rs/paste/latest/paste/#case-conversion
macro_rules! module {
    (
        $name:ident, $($code:item)*
    ) => {
        pub mod $name {
            use super::*;
            $( $code )*
        }
    };
}

macro_rules! create_field_definition {
    (
        $count:literal, $prev:literal, $config:expr, $var_name:ident, $name:expr, $encoding:expr
    ) => {
        paste! {
            pub const [<POS $count>]: CodecPositions = if $prev == 0 { POS0 } else { [<POS $prev>].next($config) };
            pub const $var_name: FieldDefinition = FieldDefinition {
                name: Some($name),
                encoding: $encoding,
                positions: [<POS $count>]
            };
        }
    };
}

macro_rules! message_fields {
    (
        $(
            define $group:ident
            $( $name:ident, $upp:ident, $pat:expr, $encoding:ident, $start:expr, $end:expr );*
        )*
    ) => {
        paste! {
            $(
                pub mod [<$group:lower>] {
                    use super::*;

                    module!(names, $( pub const $upp: &str = stringify!($name); )*);
                    module!(config, $( pub const $upp: ByteConfiguration = ByteConfiguration::new($start, $end, wf_codec::encoding::$encoding); )*);

                    pub const BYTE_CONFIG: &'static [ByteConfiguration] = &[$( config::$upp, )*];

                    count!{
                        const POS_int_name_: CodecPositions = CodecPositions::start(BYTE_CONFIG[_int_]);

                        $(
                            create_field_definition!(_int_name_, _int_prev_, config::$upp, $upp, names::$upp, wf_codec::encoding::$encoding);
                        )*
                    }

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
