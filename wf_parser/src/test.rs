use wf_buffer::{BufferReader, WhiteflagBuffer};
use wf_field::definitions;

fn message_code() -> wf_field::FieldDefinition {
    definitions::header::MESSAGE_CODE
}

fn psuedo_message_code() -> wf_field::FieldDefinition {
    definitions::test::PSEUDO_MESSAGE_CODE
}

/// hexadecimal whiteflag authentication message
/// the field values are below
/// ```rust
/// vec![
///     "WF",
///     "1",
///     "0",
///     "0",
///     "A",
///     "0",
///     "0000000000000000000000000000000000000000000000000000000000000000",
///     "1",
///     "https://organisation.int/whiteflag",
/// ];
/// ```
const AUTH_MESSAGE: &'static str = "5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380";

/// hexadecimal whiteflag test message
///
/// the field values are below
/// ```rust
/// vec![
///     "WF",
///     "1",
///     "0",
///     "1",
///     "T",
///     "3",
///     "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
///     "M",
///     "80",
///     "2013-08-31T04:29:15Z",
///     "P00D00H00M",
///     "22",
///     "+30.79658",
///     "-037.82602",
///     "8765",
///     "3210",
///     "042",
/// ];
/// ```
const TEST_MESSAGE: &'static str = "57463130aa19f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d726c01009841882148a800000114c1e596006f04c050eca6420084";

#[test]
fn extract_code_for_a_message() {
    let def = message_code();
    let buffer = WhiteflagBuffer::decode_from_hexadecimal(AUTH_MESSAGE).unwrap();

    let field = buffer.extract_message_value(&def, 33);

    assert_eq!("A", field.unwrap(), "extracted message code should be A");
}

#[test]
fn extract_code_for_a_message_2() {
    let buffer = WhiteflagBuffer::decode_from_hexadecimal(AUTH_MESSAGE).unwrap();
    let code = message_code().read(&buffer);
    assert_eq!("A", code.unwrap(), "extracted message code should be A");
}

#[test]
fn extract_code_for_t_message() {
    let buffer = WhiteflagBuffer::decode_from_hexadecimal(TEST_MESSAGE).unwrap();
    let code = message_code().read(&buffer);

    assert_eq!("T", code.unwrap(), "extracted message code should be T");

    let test_code = psuedo_message_code().read(&buffer);

    assert_eq!(
        "M",
        test_code.unwrap(),
        "extracted message code should be T"
    );
}
