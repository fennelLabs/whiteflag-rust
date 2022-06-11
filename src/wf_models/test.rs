use super::{AuthenticationMessage, WhiteflagMessage};
use serde_json::json;

#[test]
fn serialize_authentication_message() {
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

    let auth_message: AuthenticationMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(prefix, auth_message.header.prefix);
    assert_eq!(version, auth_message.header.version);
    assert_eq!(
        encryption_indicator,
        auth_message.header.encryption_indicator
    );
    assert_eq!(duress_indicator, auth_message.header.duress_indicator);
    assert_eq!(message_code, auth_message.header.message_code);
    assert_eq!(reference_indicator, auth_message.header.reference_indicator);
    assert_eq!(referenced_message, auth_message.header.referenced_message);
    assert_eq!(verification_method, auth_message.verification_method);
    assert_eq!(verification_data, auth_message.verification_data);
}

#[test]
fn serialize_message() {
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

    let message: WhiteflagMessage = serde_json::from_str(&json).unwrap();
    let wf_message: AuthenticationMessage = message.try_into().unwrap();

    assert_eq!(prefix, wf_message.header.prefix);
    assert_eq!(version, wf_message.header.version);
    assert_eq!(encryption_indicator, wf_message.header.encryption_indicator);
    assert_eq!(duress_indicator, wf_message.header.duress_indicator);
    assert_eq!(message_code, wf_message.header.message_code);
    assert_eq!(reference_indicator, wf_message.header.reference_indicator);
    assert_eq!(referenced_message, wf_message.header.referenced_message);
    assert_eq!(verification_method, wf_message.verification_method);
    assert_eq!(verification_data, wf_message.verification_data);
}

#[test]
fn serialize_message_into_values() {
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

    let message: WhiteflagMessage = serde_json::from_str(&json).unwrap();
    let values: Vec<String> = message.try_into().unwrap();

    assert_eq!(prefix, values[0]);
    assert_eq!(version, values[1]);
    assert_eq!(encryption_indicator, values[2]);
    assert_eq!(duress_indicator, values[3]);
    assert_eq!(message_code.to_string(), values[4]);
    assert_eq!(reference_indicator, values[5]);
    assert_eq!(referenced_message, values[6]);
    assert_eq!(verification_method, values[7]);
    assert_eq!(verification_data, values[8]);
}
