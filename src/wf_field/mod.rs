use crate::wf_core::{segment::MessageSegment, types::MessageType};

pub const FIELD_PREFIX: &'static str = "Prefix";
pub const FIELD_VERSION: &'static str = "Version";
pub const FIELD_MESSAGETYPE: &'static str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &'static str = "PseudoMessageCode";

pub fn get_message_type(header: &MessageSegment) -> MessageType {
    let message_code = match header.get(&FIELD_MESSAGETYPE) {
        Some(x) => x.chars().next(),
        _ => None,
    };

    MessageType::from_code_option(message_code.as_ref())
}
