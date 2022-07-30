#[cfg(test)]
mod test;

#[allow(dead_code)]
mod message_code_parser;
#[allow(dead_code)]
mod message_header_parser;
mod parsed_field_definition;

pub use message_code_parser::MessageCodeParser;
pub use message_header_parser::MessageHeaderParser;

use crate::wf_field::Field;

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MessageHeaderOrder {
    Prefix = 0,
    Version = 1,
    EncryptionIndicator = 2,
    DuressIndicator = 3,
    MessageCode = 4,
    ReferenceIndicator = 5,
    ReferencedMessage = 6,
}

impl<'a> MessageHeaderOrder {
    pub fn get<'b>(&'a self, fields: &'b [Field]) -> &'b Field {
        let index: usize = *self as usize;
        &fields[index]
    }
}

/// fields that are codes are single characters
pub fn convert_value_to_code(value: &str) -> char {
    value
        .chars()
        .nth(0)
        .unwrap_or_else(|| panic!("invalid message code"))
}
