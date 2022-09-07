use crate::{message_header_parser::MessageHeaderParser, MessageHeader};
use wf_buffer::{BufferReader, WhiteflagBuffer};
use wf_field::{
    convert_value_to_code, get_body_from_code_char, FieldDefinition, FieldValue, MessageHeaderOrder,
};

#[derive(Debug)]
pub struct MessageCodeParser {
    pub code: char,
    pub test_code: Option<char>,
}

impl MessageCodeParser {
    pub fn parse_for_decode(message: &WhiteflagBuffer) -> MessageCodeParser {
        let header = MessageHeaderParser::default();
        let code = convert_value_to_code(header.message_code().read(&message).as_ref());
        let test_code = match code {
            'T' => Some(convert_value_to_code(
                header.test_message_code().read(&message).as_ref(),
            )),
            _ => None,
        };

        MessageCodeParser { code, test_code }
    }

    /// extracts message code and test message code using the parsed field definition api
    /// the parsed field definitions are field definitions, but contextualized within their proper order
    /// they have their start and end bytes properly set according to their relative field position
    pub fn parse_from_serialized(message: &str) -> MessageCodeParser {
        let header = MessageHeaderParser::default();

        let code = convert_value_to_code(header.message_code().read_from_serialized(message));
        let test_code = if code == 'T' {
            Some(convert_value_to_code(
                header.test_message_code().read_from_serialized(message),
            ))
        } else {
            None
        };

        MessageCodeParser { code, test_code }
    }

    /// extracts message code type from array of message values
    /// the 4th position is where the message code type resides
    /// if this is a test message (code = T) then there should be a psuedo message code to be extracted
    pub fn parse_for_encode<T: FieldValue>(data: &[T]) -> Self {
        if data.len() < 6 {
            panic!(
                "a valid message must contain at least a header which is 7 values long\n{:#?}",
                data.as_ref()
            );
        }

        let code: char =
            convert_value_to_code(data[MessageHeaderOrder::MessageCode.as_usize()].as_ref());
        let test_code = if code == 'T' {
            data.iter()
                .nth(7)
                .map(|v| convert_value_to_code(v.as_ref()))
        } else {
            None
        };

        MessageCodeParser { code, test_code }
    }

    // collects all the field definitions based on the parsed codes
    pub fn get_field_definitions(&self) -> Vec<FieldDefinition> {
        let mut defs = get_body_from_code_char(&self.code);

        if let Some(c) = &self.test_code {
            defs.append(get_body_from_code_char(c).as_mut());
        };

        defs
    }
}
