use crate::{
    wf_field::definitions,
    wf_validation::{Validation, ValidationError},
};

use super::{basic_message::BasicMessage, decode, encode};

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

    assert_eq!(field_values.concat(), message.serialize());
}

#[test]
fn encode_auth_message() {
    let encoding_result: String = "5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380".to_string();

    let auth_message = vec![
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
        encode(&auth_message),
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

    assert_eq!(field_values.concat(), message.serialize());
}
