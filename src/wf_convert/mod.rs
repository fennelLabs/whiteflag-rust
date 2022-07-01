use crate::wf_core::basic_message::BasicMessage;
use crate::wf_core::segment::MessageSegment;
use crate::wf_core::types::MessageType;

pub const FIELD_MESSAGETYPE: &str = "MessageCode";

pub trait FieldValue: AsRef<str> + Into<String> {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> {}

fn compile<T: FieldValue>(data: &[T]) -> BasicMessage {
    let mut header: MessageSegment = MessageSegment::generic_header_segment();
    header.set_all(data.as_ref(), 0);

    let mut message_type = get_message_type(&header);
    let body_start_index = header.get_number_of_fields();
    let body = &mut message_type.body;

    //need switch statement here

    body.set_all(data.as_ref(), body_start_index);
    BasicMessage::new(message_type.message_code, header, message_type.body)
}

impl<T: FieldValue> From<&[T]> for BasicMessage {
    fn from(data: &[T]) -> Self {
        compile(data)
    }
}

fn get_message_type(header: &MessageSegment) -> MessageType {
    let message_code = match header.get(&FIELD_MESSAGETYPE) {
        Some(x) => x.chars().next(),
        _ => None,
    };

    MessageType::from_code_option(message_code.as_ref())
}
