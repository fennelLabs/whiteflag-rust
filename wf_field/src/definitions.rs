use crate::{
    byte_configuration::ByteConfiguration, codec_positions::CodecPositions, FieldDefinition,
};
use count_macro::count;
use paste::paste;

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

macro_rules! message_fields {
    (
        $(
            define $group:ident
            $( $name:ident, $upp:ident, $pat:expr, $encoding:ident, $start:expr, $end:expr );*
        )*
    ) => {
        paste! {

            pub const ALL_BYTE_CONFIG: &'static [ByteConfiguration] = &[$(
                $( ByteConfiguration::new($start, $end, wf_codec::encoding::$encoding), )*
            )*];

            count! {
                //#[repr(usize)]
                #[derive(Copy, Clone)]
                pub enum WhiteflagFields {
                    $( $( [<$group $name>] = _int_, )* )*
                }
            }

            impl WhiteflagFields {
                pub const fn get_start_bit(&self) -> usize {
                    let index = self.as_usize();
                    if index == 0 { 0 }
                    else {
                        // first field after last header field must begin at the last header
                        let prev_index = if WhiteflagFields::is_first_in_group(&self) { WhiteflagFields::HeaderReferencedMessage.as_usize() } else { index - 1 };
                        let prev_field = WhiteflagFields::as_enum(prev_index);
                        ALL_BYTE_CONFIG[prev_index].bit_length() + WhiteflagFields::get_start_bit(&prev_field)
                    }
                }

                pub const fn as_usize(&self) -> usize {
                    *self as usize
                }

                pub const fn as_enum(i: usize) -> Self {
                    count! {
                        match i {
                            $( $( _int_ => WhiteflagFields::[<$group $name>], )* )*
                            _ => panic!("number is not supported!"),
                        }
                    }
                }

                pub const fn get_byte_config(&self) -> ByteConfiguration {
                    ALL_BYTE_CONFIG[self.as_usize()]
                }

                pub const fn is_first_in_group(&self) -> bool {
                    match &self {
                        WhiteflagFields::HeaderPrefix => true,
                        WhiteflagFields::AuthenticationVerificationMethod => true,
                        WhiteflagFields::CryptoCryptoDataType => true,
                        WhiteflagFields::FreeTextText => true,
                        WhiteflagFields::ResourceResourceMethod => true,
                        WhiteflagFields::TestPseudoMessageCode => true,
                        WhiteflagFields::SignSubjectCode => true,
                        WhiteflagFields::RequestObjectType => true,
                        _ => false,
                    }
                }

                pub const fn create_codec_position(&self) -> CodecPositions {
                    CodecPositions::new(
                        WhiteflagFields::get_byte_config(self),
                        WhiteflagFields::get_start_bit(self)
                    )
                }
            }

            $(
                pub mod [<$group:lower>] {
                    use super::*;

                    module!(names, $( pub const $upp: &str = stringify!($name); )*);

                    $(
                        pub const $upp: FieldDefinition = FieldDefinition::create_definition(
                            names::$upp,
                            WhiteflagFields::[<$group $name>]
                        );
                    )*

                    pub const DEFINITIONS: &'static [FieldDefinition] = &[$( $upp, )*];

                    /* pub mod rx {
                        use regex::Regex;
                        lazy_static!{
                            $( pub static ref $upp: Regex = Regex::new($pat).unwrap(); )*
                        }
                    } */
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
