#[cfg(test)]
mod test;

mod message_code_parser;
mod message_header_parser;
mod parsed_field_definition;
mod wf_header;

pub use message_code_parser::MessageCodeParser;
pub use message_header_parser::MessageHeaderParser;
pub use wf_header::MessageHeaderFields;

use crate::wf_field::{definitions::convert_value_to_code, Field};

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

    pub fn get_code(fields: &[Field]) -> (&Field, char) {
        let field = Self::MessageCode.get(fields);
        (field, convert_value_to_code(field.get()))
    }
}
