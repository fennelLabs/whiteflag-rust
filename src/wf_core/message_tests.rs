use super::creator::{decode, encode};

#[test]
fn encode_sign_signal_message() {
    let encoding_result: String = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084".to_string();
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "M",
        "4",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];

    assert_eq!(
        encoding_result,
        encode(&field_values),
        "Encoding should be correct"
    );
}

#[test]
fn decode_sign_signal_message() {
    let encoding_result: String = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084".to_string();
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "M",
        "4",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];

    let message = decode(encoding_result);

    //assert_eq!(message.set("Version", "2"), "Should not be able to change version field");
    assert_eq!(
        field_values[0],
        message.get("Prefix"),
        "Prefix should be correctly set"
    );
    assert_eq!(
        field_values[1],
        message.get("Version"),
        "Version number should be correctly set"
    );
    assert_eq!(
        field_values[2],
        message.get("EncryptionIndicator"),
        "Encryption indicator should be correctly set"
    );
    assert_eq!(
        field_values[3],
        message.get("DuressIndicator"),
        "Duress indicator should be correctly set"
    );
    assert_eq!(
        field_values[4],
        message.get("MessageCode"),
        "Message code should be correctly set"
    );
    assert_eq!(
        field_values[5],
        message.get("ReferenceIndicator"),
        "Reference indicator should be correctly set"
    );
    assert_eq!(
        field_values[6],
        message.get("ReferencedMessage"),
        "Referenced message should be correctly set"
    );
    assert_eq!(
        field_values[7],
        message.get("SubjectCode"),
        "Subject code should be correctly set"
    );
    assert_eq!(
        field_values[8],
        message.get("DateTime"),
        "DateTime should be correctly set"
    );
    assert_eq!(
        field_values[9],
        message.get("Duration"),
        "Duration should be correctly set"
    );
    assert_eq!(
        field_values[10],
        message.get("ObjectType"),
        "Object code  should be correctly set"
    );
    assert_eq!(
        field_values[11],
        message.get("ObjectLatitude"),
        "Latitude should be correctly set"
    );
    assert_eq!(
        field_values[12],
        message.get("ObjectLongitude"),
        "Longitude should be correctly set"
    );
    assert_eq!(
        field_values[13],
        message.get("ObjectSizeDim1"),
        "Size dimention 1 should be correctly set"
    );
    assert_eq!(
        field_values[14],
        message.get("ObjectSizeDim2"),
        "Size dimention 2 should be correctly set"
    );
    assert_eq!(
        field_values[15],
        message.get("ObjectOrientation"),
        "Orientation should be correctly set"
    );
}

#[test]
fn encode_auth_message() {
    let encoding_result: String = "5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380".to_string();
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "A",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "1",
        "https://organisation.int/whiteflag",
    ];

    assert_eq!(
        encoding_result,
        encode(&field_values),
        "Encoding should be correct"
    );
}

#[test]
fn decode_auth_message() {
    /* Setup */
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "A",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "1",
        "https://organisation.int/whiteflag",
    ];

    let message = decode("5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380");

    /* Verify */
    //assert_eq!("Message type should be correct", A, message.getType());
    //assert_eq!(fieldValues.length, message.getNoFields(), "Number of fields should be equal to number of provided fields");
    //assert_eq!(message.getFieldNames().size(), message.getNoFields(), "Number of fields should be equal to number of field names in set");
    assert_eq!(
        field_values[0],
        message.get("Prefix"),
        "Prefix should be correctly set"
    );
    assert_eq!(
        field_values[1],
        message.get("Version"),
        "Version number should be correctly set"
    );
    assert_eq!(
        field_values[2],
        message.get("EncryptionIndicator"),
        "Encryption indicator should be correctly set"
    );
    assert_eq!(
        field_values[3],
        message.get("DuressIndicator"),
        "Duress indicator should be correctly set"
    );
    assert_eq!(
        field_values[4],
        message.get("MessageCode"),
        "Message code should be correctly set"
    );
    assert_eq!(
        field_values[5],
        message.get("ReferenceIndicator"),
        "Reference indicator should be correctly set"
    );
    assert_eq!(
        field_values[6],
        message.get("ReferencedMessage"),
        "Referenced message should be correctly set"
    );
    assert_eq!(
        field_values[7],
        message.get("VerificationMethod"),
        "Verification method should be correctly set"
    );
    assert_eq!(
        field_values[8],
        message.get("VerificationData"),
        "Verification data should be correctly set"
    );
    //assertTrue("Message should be valid", message.isValid());
}
