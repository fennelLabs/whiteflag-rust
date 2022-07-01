use crate::wf_core::{definitions::*, field::Field, segment::MessageSegment};

pub const FIELD_PREFIX: &'static str = "Prefix";
pub const FIELD_VERSION: &'static str = "Version";
pub const FIELD_MESSAGETYPE: &'static str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &'static str = "PseudoMessageCode";

pub fn get_message_code(header: &MessageSegment) -> char {
    match header.get(&FIELD_MESSAGETYPE) {
        Some(x) => x.chars().next(),
        _ => None,
    }
    .expect("expected message code but none was found")
}

pub fn get_message_body(header: &MessageSegment) -> (MessageSegment, char) {
    let message_code = get_message_code(header);
    (get_body_from_code(&message_code), message_code)
}

pub fn get_body_from_code(code: &char) -> MessageSegment {
    let body: Vec<Field> = match code {
        'A' => authentication_body_fields().to_vec(),
        'K' => crypto_body_fields().to_vec(),
        'T' => test_body_fields().to_vec(),
        'R' => resource_body_fields().to_vec(),
        'F' => freetext_body_fields().to_vec(),
        'P' | 'E' | 'D' | 'S' | 'I' | 'M' | 'Q' => sign_signal_body_fields().to_vec(),
        _ => Vec::<Field>::new(),
    };

    MessageSegment::from(body)
}
