use crate::{
    wf_core::FieldValue,
    wf_field::{
        definitions::{convert_value_to_code, get_body_from_code_char},
        FieldDefinition,
    },
};

#[derive(Debug)]
pub struct MessageCodeParser {
    pub code: char,
    pub test_code: Option<char>,
}

impl MessageCodeParser {
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
        get_body_from_code_char(&self.test_code.unwrap_or(self.code))
    }

    // collects all the field definitions based on the parsed codes
    pub fn get_field_definitions_for_encode(&self) -> Vec<FieldDefinition> {
        let mut defs = get_body_from_code_char(&self.code);

        match &self.test_code {
            Some(c) => {
                defs.append(get_body_from_code_char(c).as_mut());
            }
            None => (),
        };

        defs
    }
}
