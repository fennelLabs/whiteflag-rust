use super::field::Field;
use super::wf_codec::encoding::*;
use regex::Regex;

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

pub fn message_code() -> Field {
    Field::new(
        "MessageCode",
        Regex::new("^[A-Z]{1}$").ok(), //"(?=A|K|T|P|E|S|D|I|M|Q|R|F)^[A-Z]{1}$"
        UTF8,
        5,
        6,
    )
}

pub fn generic_header_fields() -> [Field; 7] {
    [
        Field::new("Prefix", Regex::new("^WF$").ok(), UTF8, 0, 2),
        Field::new("Version", Regex::new("^[A-Z0-9]{1}$").ok(), UTF8, 2, 3), //"(?=1)^[A-Z0-9]{1}$"
        Field::new(
            "EncryptionIndicator",
            Regex::new("^[A-Z0-9]{1}$").ok(), //"(?=0|1|2)^[A-Z0-9]{1}$"
            UTF8,
            3,
            4,
        ),
        Field::new("DuressIndicator", Regex::new("^[0-1]{1}$").ok(), BIN, 4, 5),
        message_code(),
        Field::new(
            "ReferenceIndicator",
            Regex::new(
                ["^", HEX.charset, "{1}$"] //"(?=0|1|2|3|4|5|6|7|8|9)^", HEX.charset, "{1}$"
                    .concat()
                    .as_str(),
            )
            .ok(),
            HEX,
            6,
            7,
        ),
        Field::new(
            "ReferencedMessage",
            Regex::new(["^", HEX.charset, "{64}$"].concat().as_str()).ok(),
            HEX,
            7,
            71,
        ),
    ]
}

pub fn authentication_body_fields() -> [Field; 2] {
    [
        Field::new(
            "VerificationMethod",
            Regex::new(["(?=1|2)^", HEX.charset, "{1}$"].concat().as_str()).ok(),
            HEX,
            71,
            72,
        ),
        Field::new(
            "VerificationData",
            Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
            UTF8,
            72,
            -1,
        ),
    ]
}

pub fn crypto_body_fields() -> [Field; 2] {
    [
        Field::new(
            "CryptoDataType",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            71,
            73,
        ),
        Field::new(
            "CryptoData",
            Regex::new(["^", HEX.charset, "*$"].concat().as_str()).ok(),
            HEX,
            73,
            -1,
        ),
    ]
}

pub fn freetext_body_fields() -> [Field; 1] {
    [Field::new(
        "Text",
        Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
        UTF8,
        71,
        -1,
    )]
}

pub fn resource_body_fields() -> [Field; 2] {
    [
        Field::new(
            "ResourceMethod",
            Regex::new(["(?=1)^", HEX.charset, "{1}$"].concat().as_str()).ok(),
            HEX,
            71,
            72,
        ),
        Field::new(
            "ResourceData",
            Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
            UTF8,
            72,
            -1,
        ),
    ]
}

pub fn test_body_fields() -> [Field; 1] {
    [Field::new(
        "PseudoMessageCode",
        Regex::new("^[A-Z]{1}$").ok(),
        UTF8,
        71,
        72,
    )]
}

pub fn sign_signal_body_fields() -> [Field; 9] {
    [
        Field::new(
            "SubjectCode",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            71,
            73,
        ),
        Field::new(
            "DateTime",
            Regex::new(["^", DATETIME.charset, "$"].concat().as_str()).ok(),
            DATETIME,
            73,
            93,
        ),
        Field::new(
            "Duration",
            Regex::new(["^", DURATION.charset, "$"].concat().as_str()).ok(),
            DURATION,
            93,
            103,
        ),
        Field::new(
            "ObjectType",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            103,
            105,
        ),
        Field::new(
            "ObjectLatitude",
            Regex::new(["^", LAT.charset, "$"].concat().as_str()).ok(),
            LAT,
            105,
            114,
        ),
        Field::new(
            "ObjectLongitude",
            Regex::new(["^", LONG.charset, "$"].concat().as_str()).ok(),
            LONG,
            114,
            124,
        ),
        Field::new(
            "ObjectSizeDim1",
            Regex::new(["^", DEC.charset, "{4}$"].concat().as_str()).ok(),
            DEC,
            124,
            128,
        ),
        Field::new(
            "ObjectSizeDim2",
            Regex::new(["^", DEC.charset, "{4}$"].concat().as_str()).ok(),
            DEC,
            128,
            132,
        ),
        Field::new(
            "ObjectOrientation",
            Regex::new(["^", DEC.charset, "{3}$"].concat().as_str()).ok(),
            DEC,
            132,
            135,
        ),
    ]
}

pub fn request_fields() -> [Field; 2] {
    [
        Field::new(
            "ObjectType",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            135,
            137,
        ),
        Field::new(
            "ObjectTypeQuant",
            Regex::new(["^", DEC.charset, "{2}$"].concat().as_str()).ok(),
            DEC,
            137,
            139,
        ),
    ]
}
