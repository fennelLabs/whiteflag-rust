use super::AuthenticationMessage;

#[test]
fn serialize() {
    let prefix = "WF";
    let version = "1";
    let encryption_indicator = "0";
    let duress_indicator = "0";
    let message_code = "A";
    let reference_indicator = "0";
    let referenced_message = "0000000000000000000000000000000000000000000000000000000000000000";
    let verification_method = "1";
    let verification_data = "https://organisation.int/whiteflag";

    let json = format!(
        "{{
        \"prefix\": \"{}\",
        \"version\": \"{}\",
        \"encryptionIndicator\": \"{}\",
        \"duressIndicator\": \"{}\",
        \"messageCode\": \"{}\",
        \"referenceIndicator\": \"{}\",
        \"referencedMessage\": \"{}\",
        \"verificationMethod\": \"{}\",
        \"verificationData\": \"{}\"
    }}",
        prefix,
        version,
        encryption_indicator,
        duress_indicator,
        message_code,
        reference_indicator,
        referenced_message,
        verification_method,
        verification_data
    );

    let auth_message: AuthenticationMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(prefix, auth_message.prefix);
    assert_eq!(version, auth_message.version);
    assert_eq!(encryption_indicator, auth_message.encryption_indicator);
    assert_eq!(duress_indicator, auth_message.duress_indicator);
    assert_eq!(message_code, auth_message.message_code);
    assert_eq!(reference_indicator, auth_message.reference_indicator);
    assert_eq!(referenced_message, auth_message.referenced_message);
    assert_eq!(verification_method, auth_message.verification_method);
    assert_eq!(verification_data, auth_message.verification_data);
}
