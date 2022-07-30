use crate::wf_core::basic_message::BasicMessage;

mod test_message {
    pub const SERIALIZED: &'static str = "WF101T33efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3aeM802013-08-31T04:29:15ZP00D00H00M22+30.79658-037.8260287653210042";
    pub const ENCODED: &'static str = "57463130aa19f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d726c01009841882148a800000114c1e596006f04c050eca6420084";
    pub const VALUES: &'static [&'static str] = &[
        "WF",
        "1",
        "0",
        "1",
        "T",
        "3",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "M",
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
}

#[test]
fn test_t_message() {
    let message: BasicMessage = crate::wf_convert::compile(test_message::VALUES);
    let encoded_message = super::creator::encode(test_message::VALUES);
    let decoded_message = super::creator::decode(test_message::ENCODED);

    assert_eq!(
        test_message::SERIALIZED,
        &message.serialize(),
        "message should serialize correctly"
    );

    assert_eq!(
        test_message::ENCODED,
        encoded_message,
        "message should encode correctly"
    );

    assert_eq!(
        test_message::SERIALIZED,
        &decoded_message.serialize(),
        "message should decode correctly"
    );

    /* assert_eq!(None, message.get_transaction_hash());
    assert_eq!("T", message.message_type());
    assert_eq!("T", messageDecoded.message_type());
    assert_eq!(messageSerialized, message.serialize());
    assert_eq!(messageSerialized, message.serialize());
    assert_eq!(messageSerialized, messageDecoded.serialize());
    assert_eq!(fieldValues[0], message.prefix());
    assert_eq!(fieldValues[0], messageDecoded.prefix());
    assert_eq!(fieldValues[1], message.version());
    assert_eq!(fieldValues[2], message.encryption_indicator());
    assert_eq!(fieldValues[3], message.duress_indictor());
    assert_eq!(fieldValues[4], message.message_code());
    assert_eq!(fieldValues[5], message.reference_indicator());
    assert!(!message.set_reference_indicator("6"));
    assert_eq!(fieldValues[6], message.referenced_message());
    assert_eq!(fieldValues[6], messageDecoded.referenced_message());
    assert_eq!(fieldValues[7], message.pseudo_message_code());
    assert_eq!(fieldValues[7], messageDecoded.pseudo_message_code());
    assert_eq!(fieldValues[8], message.get_subject_code());
    assert_eq!(fieldValues[9], message.datetime());
    assert_eq!(fieldValues[10], message.duration());
    assert_eq!(fieldValues[11], message.get_object_type());
    assert_eq!(fieldValues[12], message.object_latitude());
    assert_eq!(fieldValues[13], message.object_longitude());
    assert_eq!(fieldValues[14], message.object_size_dim_one());
    assert_eq!(fieldValues[15], message.object_size_dim_two());
    assert_eq!(fieldValues[16], messageDecoded.object_orientation());
    assert!(message.is_valid());
    assert!(messageDecoded.is_valid());
    assert_eq!(
        None,
        messageDecoded.set_transaction_hash("a1b2c3".to_string())
    );
    assert_eq!(
        "a1b2c3",
        messageDecoded
            .set_transaction_hash("d4e5f6".to_string())
            .unwrap()
    );
    assert_eq!(
        None,
        messageDecoded.set_originator_address("abc123".to_string())
    );
    assert_eq!("abc123", messageDecoded.get_originator_address()); */
}

/* #[test]
fn testRequestMessage() {
    let messageSerialized = "WF101Q13efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae802013-08-31T04:29:15ZP01D00H00M22+31.79658-033.826028799321000010022003";
    let fieldValues = vec![
        "WF",
        "1",
        "0",
        "1",
        "Q",
        "1",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P01D00H00M",
        "22",
        "+31.79658",
        "-033.82602",
        "8799",
        "3210",
        "000",
        "10",
        "02",
        "20",
        "03",
    ];
    let message = WhiteflagMessage::compile(fieldValues).unwrap();
    let messageEncoded = &hex::encode(message.encode());
    let messageDecoded = WhiteflagMessage::decode(messageEncoded).unwrap();

    assert_eq!("Q", message.message_type());
    assert_eq!("Q", messageDecoded.message_type());
    assert_eq!(messageSerialized, message.serialize());
    assert_eq!(messageDecoded.serialize(), message.serialize());
    assert_eq!(fieldValues[0], message.prefix());
    assert_eq!(fieldValues[1], message.version());
    assert_eq!(fieldValues[2], message.encryption_indicator());
    assert_eq!(fieldValues[3], message.duress_indictor());
    assert_eq!(fieldValues[4], message.message_code());
    assert_eq!(fieldValues[5], message.reference_indicator());
    assert_eq!(fieldValues[6], message.referenced_message());
    assert_eq!(fieldValues[7], message.get_subject_code());
    assert_eq!(fieldValues[8], message.datetime());
    assert_eq!(fieldValues[9], message.duration());
    assert_eq!(fieldValues[10], message.get_object_type());
    assert_eq!(fieldValues[11], message.object_latitude());
    assert_eq!(fieldValues[12], message.object_longitude());
    assert_eq!(fieldValues[13], message.object_size_dim_one());
    assert_eq!(fieldValues[14], message.object_size_dim_two());
    assert_eq!(fieldValues[15], message.object_orientation());
    assert_eq!(fieldValues[16], message.object_type_one());
    assert_eq!(fieldValues[17], message.object_type_one_quantity());
    assert_eq!(fieldValues[18], message.object_type_two());
    assert_eq!(fieldValues[19], message.object_type_two_quantity());
    assert!(message.is_valid());
    assert!(messageDecoded.is_valid());
} */
