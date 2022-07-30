use crate::{
    wf_buffer::WhiteflagBuffer,
    wf_convert::FieldValue,
    wf_field::{get_body_from_code, Field, FieldDefinition},
};

use super::{convert_value_to_code, message_header_parser::MessageHeaderParser};

#[derive(Debug)]
pub struct MessageCodeParser {
    pub code: char,
    pub test_code: Option<char>,
}

impl MessageCodeParser {
    pub fn parse_for_decode(buffer: &WhiteflagBuffer) -> (Self, Vec<Field>, Option<usize>) {
        let mut body: Vec<Field> = vec![];
        let header_parser = MessageHeaderParser::default();
        let code = convert_value_to_code(&header_parser.message_code().extract(buffer));
        let mut shift: Option<usize> = None;

        let test_code = if code == 'T' {
            let def = header_parser.test_message_code();
            shift = Some(def.bit_length());
            let value = def.extract(buffer);
            let code = convert_value_to_code(&value);
            body.push(Field::new(def.into(), value));
            Some(code)
        } else {
            None
        };

        (MessageCodeParser { code, test_code }, body, shift)
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

        let code: char = convert_value_to_code(data[4].as_ref());
        let test_code = if code == 'T' {
            data.iter()
                .nth(7)
                .map(|v| convert_value_to_code(v.as_ref()))
        } else {
            None
        };

        MessageCodeParser { code, test_code }
    }

    /// collects all the field definitions based on the parsed codes
    pub fn get_field_definitions_for_decode(&self) -> Vec<FieldDefinition> {
        match &self.test_code {
            Some(c) => get_body_from_code(c),
            None => get_body_from_code(&self.code),
        }
    }

    // collects all the field definitions based on the parsed codes
    pub fn get_field_definitions_for_encode(&self) -> Vec<FieldDefinition> {
        let mut defs = get_body_from_code(&self.code);

        match &self.test_code {
            Some(c) => {
                defs.append(get_body_from_code(c).as_mut());
            }
            None => (),
        };

        defs
    }

    /* pub fn decode_body(&self, buffer: &WhiteflagBuffer) {
        let body: Vec<Field> = if self.test_code == Some(c) { vec![c] } else { vec![] };

        buffer.decode(self.get_field_definitions(), 0);
    } */
}
