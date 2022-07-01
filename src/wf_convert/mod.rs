use crate::wf_core::basic_message::BasicMessage;
use crate::wf_core::segment::MessageSegment;
use crate::wf_field::get_message_body;

pub trait FieldValue: AsRef<str> + Into<String> {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> {}

fn compile<T: FieldValue>(data: &[T]) -> BasicMessage {
    let mut header: MessageSegment = MessageSegment::generic_header_segment();
    header.set_all(data.as_ref(), 0);

    let (mut body, code) = get_message_body(&header);
    let body_start_index = header.get_number_of_fields();

    //need switch statement here

    body.set_all(data.as_ref(), body_start_index);
    BasicMessage::new(code, header, body)
}

impl<T: FieldValue> From<&[T]> for BasicMessage {
    fn from(data: &[T]) -> Self {
        compile(data)
    }
}
