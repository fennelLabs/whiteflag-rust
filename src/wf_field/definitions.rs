use super::{field_definition::FieldDefinition};
use crate::wf_codec::encoding::*;
use regex::Regex;

pub fn get_body_from_code(code: &char) -> Vec<FieldDefinition> {
    match code {
        'A' => authentication_body_fields().to_vec(),
        'K' => crypto_body_fields().to_vec(),
        'T' => test_body_fields().to_vec(),
        'R' => resource_body_fields().to_vec(),
        'F' => freetext_body_fields().to_vec(),
        'P' | 'E' | 'D' | 'S' | 'I' | 'M' | 'Q' => sign_signal_body_fields().to_vec(),
        _ => Vec::<FieldDefinition>::new(),
    }
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

fn message_code() -> FieldDefinition {
    FieldDefinition::new(
        "MessageCode",
        Regex::new("^[A-Z]{1}$").ok(), //"(?=A|K|T|P|E|S|D|I|M|Q|R|F)^[A-Z]{1}$"
        UTF8,
        5,
        6,
    )
}

pub fn generic_header_fields() -> [FieldDefinition; 7] {
    [
        FieldDefinition::new("Prefix", Regex::new("^WF$").ok(), UTF8, 0, 2),
        FieldDefinition::new("Version", Regex::new("^[A-Z0-9]{1}$").ok(), UTF8, 2, 3), //"(?=1)^[A-Z0-9]{1}$"
        FieldDefinition::new(
            "EncryptionIndicator",
            Regex::new("^[A-Z0-9]{1}$").ok(), //"(?=0|1|2)^[A-Z0-9]{1}$"
            UTF8,
            3,
            4,
        ),
        FieldDefinition::new("DuressIndicator", Regex::new("^[0-1]{1}$").ok(), BIN, 4, 5),
        message_code(),
        FieldDefinition::new(
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
        FieldDefinition::new(
            "ReferencedMessage",
            Regex::new(["^", HEX.charset, "{64}$"].concat().as_str()).ok(),
            HEX,
            7,
            71,
        ),
    ]
}

fn authentication_body_fields() -> [FieldDefinition; 2] {
    [
        FieldDefinition::new(
            "VerificationMethod",
            Regex::new(["(?=1|2)^", HEX.charset, "{1}$"].concat().as_str()).ok(),
            HEX,
            71,
            72,
        ),
        FieldDefinition::new(
            "VerificationData",
            Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
            UTF8,
            72,
            -1,
        ),
    ]
}

fn crypto_body_fields() -> [FieldDefinition; 2] {
    [
        FieldDefinition::new(
            "CryptoDataType",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            71,
            73,
        ),
        FieldDefinition::new(
            "CryptoData",
            Regex::new(["^", HEX.charset, "*$"].concat().as_str()).ok(),
            HEX,
            73,
            -1,
        ),
    ]
}

fn freetext_body_fields() -> [FieldDefinition; 1] {
    [FieldDefinition::new(
        "Text",
        Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
        UTF8,
        71,
        -1,
    )]
}

fn resource_body_fields() -> [FieldDefinition; 2] {
    [
        FieldDefinition::new(
            "ResourceMethod",
            Regex::new(["(?=1)^", HEX.charset, "{1}$"].concat().as_str()).ok(),
            HEX,
            71,
            72,
        ),
        FieldDefinition::new(
            "ResourceData",
            Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
            UTF8,
            72,
            -1,
        ),
    ]
}

fn test_body_fields() -> [FieldDefinition; 1] {
    [FieldDefinition::new(
        "PseudoMessageCode",
        Regex::new("^[A-Z]{1}$").ok(),
        UTF8,
        71,
        72,
    )]
}

fn sign_signal_body_fields() -> [FieldDefinition; 9] {
    [
        FieldDefinition::new(
            "SubjectCode",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            71,
            73,
        ),
        FieldDefinition::new(
            "DateTime",
            Regex::new(["^", DATETIME.charset, "$"].concat().as_str()).ok(),
            DATETIME,
            73,
            93,
        ),
        FieldDefinition::new(
            "Duration",
            Regex::new(["^", DURATION.charset, "$"].concat().as_str()).ok(),
            DURATION,
            93,
            103,
        ),
        FieldDefinition::new(
            "ObjectType",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            103,
            105,
        ),
        FieldDefinition::new(
            "ObjectLatitude",
            Regex::new(["^", LAT.charset, "$"].concat().as_str()).ok(),
            LAT,
            105,
            114,
        ),
        FieldDefinition::new(
            "ObjectLongitude",
            Regex::new(["^", LONG.charset, "$"].concat().as_str()).ok(),
            LONG,
            114,
            124,
        ),
        FieldDefinition::new(
            "ObjectSizeDim1",
            Regex::new(["^", DEC.charset, "{4}$"].concat().as_str()).ok(),
            DEC,
            124,
            128,
        ),
        FieldDefinition::new(
            "ObjectSizeDim2",
            Regex::new(["^", DEC.charset, "{4}$"].concat().as_str()).ok(),
            DEC,
            128,
            132,
        ),
        FieldDefinition::new(
            "ObjectOrientation",
            Regex::new(["^", DEC.charset, "{3}$"].concat().as_str()).ok(),
            DEC,
            132,
            135,
        ),
    ]
}

fn request_fields() -> [FieldDefinition; 2] {
    [
        FieldDefinition::new(
            "ObjectType",
            Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(),
            HEX,
            135,
            137,
        ),
        FieldDefinition::new(
            "ObjectTypeQuant",
            Regex::new(["^", DEC.charset, "{2}$"].concat().as_str()).ok(),
            DEC,
            137,
            139,
        ),
    ]
}
