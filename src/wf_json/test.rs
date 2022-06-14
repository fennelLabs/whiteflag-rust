use crate::{decode_from_hex, wf_json::deserialize::WhiteflagFieldValues};
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
