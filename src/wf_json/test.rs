use crate::{decode_from_hex, wf_json::deserialize::WhiteflagFieldValues, wf_core::basic_message::BasicMessage};
use serde_json::json;

#[test]
fn deserialize_message_from_json_to_fields() {
    let prefix = "WF";
    let version = "1";
    let encryption_indicator = "0";
    let duress_indicator = "0";
    let message_code = 'A';
    let reference_indicator = "0";
    let referenced_message = "0000000000000000000000000000000000000000000000000000000000000000";
    let verification_method = "1";
    let verification_data = "https://organisation.int/whiteflag";

    let json = json!({
        "prefix": prefix,
        "version": version,
        "encryptionIndicator": encryption_indicator,
        "duressIndicator": duress_indicator,
        "messageCode": message_code,
        "referenceIndicator": reference_indicator,
        "referencedMessage": referenced_message,
        "verificationMethod": verification_method,
        "verificationData": verification_data
    })
    .to_string();

    let message: WhiteflagFieldValues = serde_json::from_str(&json).unwrap();

    assert_eq!(
        vec![
            prefix,
            version,
            encryption_indicator,
            duress_indicator,
            &message_code.to_string(),
            reference_indicator,
            referenced_message,
            verification_method,
            verification_data
        ],
        message.fields
    )
}

#[test]
fn test_serialize_auth_message_from_hex() {
    let hex = "5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380";

    let json = json!({
        "prefix": "WF",
        "version": "1",
        "encryptionIndicator": "0",
        "duressIndicator": "0",
        "messageCode": 'A',
        "referenceIndicator": "0",
        "referencedMessage": "0000000000000000000000000000000000000000000000000000000000000000",
        "verificationMethod": "1",
        "verificationData": "https://organisation.int/whiteflag"
    })
    .to_string();

    test_json(&json, &decode_from_hex(hex).unwrap());
}

fn test_json(actual: &str, expected: &str) {
    let a = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(actual).unwrap();
    let e = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(expected).unwrap();
    assert_eq!(a, e);
}

#[test]
fn json_deserialization() {
    let message_str = "WF100F5f6c1e1ed8950b137bb9e0edcf21593d62c03a7fb39dacfd554c593f72c8942dfWhiteflag test message!";
    let json = json!({ 
        "MetaHeader": {},
        "MessageHeader": {
            "Prefix": "WF", 
            "Version":"1",
            "EncryptionIndicator":"0",
            "DuressIndicator": "0",
            "MessageCode": "F",
            "ReferenceIndicator": "5",
            "ReferencedMessage": "f6c1e1ed8950b137bb9e0edcf21593d62c03a7fb39dacfd554c593f72c8942df"
        },
        "MessageBody": {
            "Text": "Whiteflag test message!"
        }
    }).to_string();
    let message = BasicMessage::deserialize_from_json(json).unwrap();
    assert_eq!(message_str, message.serialize());
}

/*
#[test]
fn testJsonSerialization() {
    let mut message1 = BasicMessage::deserialize("WF100F5f6c1e1ed8950b137bb9e0edcf21593d62c03a7fb39dacfd554c593f72c8942dfWhiteflag test message!").unwrap();
    let jsonMessageStr: String = message1.to_json();
    let message2 = BasicMessage::deserializeJson(jsonMessageStr).unwrap();

    assert_eq!(message1.message_type(), message2.message_type());
    assert_eq!(message1.prefix(), message2.prefix());
    assert_eq!(message1.version(), message2.version());
    assert!(!message1.set_encryption_indicator("2".to_string()));
    assert_eq!(
        message1.encryption_indicator(),
        message2.encryption_indicator()
    );
    assert_eq!(message1.duress_indictor(), message2.duress_indictor());
    assert_eq!(message1.message_code(), message2.message_code());
    assert_eq!(
        message1.reference_indicator(),
        message2.reference_indicator()
    );
    assert_eq!(message1.referenced_message(), message2.referenced_message());
    assert!(!message2.set_text("alternate text"));
    assert_eq!(message1.text(), message2.text());
    assert!(message1.is_valid());
    assert!(message2.is_valid());
}

 */