use crate::wf_core::message::Message;

mod test_message {
    pub const SERIALIZED: &str = "WF101T33efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3aeM802013-08-31T04:29:15ZP00D00H00M22+30.79658-037.8260287653210042";
    //pub const ENCODED: &'static str = "57463130aa19f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d726c01009841882148a800000114c1e596006f04c050eca6420084";
    pub const VALUES: &[&str] = &[
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

mod request_signal_message {
    pub const SERIALIZED: &str= "WF101Q13efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae802013-08-31T04:29:15ZP01D00H00M22+31.79658-033.826028799321000010022003";
    //pub const ENCODED: &'static str = "";
    pub const VALUES: &[&str] = &[
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
}

#[test]
fn test_t_message() {
    test(test_message::SERIALIZED, test_message::VALUES)
}

#[test]
fn test_q_message() {
    test(
        request_signal_message::SERIALIZED,
        request_signal_message::VALUES,
    )
}

fn test(serialized: &'static str, values: &'static [&'static str]) {
    let message: Message = values.into();
    let encoded_message = super::encode(values).unwrap();
    let decoded_message = super::decode(&encoded_message).unwrap();

    assert_eq!(
        serialized,
        &message.serialize(),
        "message should serialize correctly"
    );

    assert_eq!(
        serialized,
        &decoded_message.serialize(),
        "message should serialize correctly"
    );

    assert_eq!(
        encoded_message,
        message.encode_as_hex(),
        "message should encode correctly"
    );

    assert_eq!(
        encoded_message,
        decoded_message.encode_as_hex(),
        "message should encode correctly"
    );
}
