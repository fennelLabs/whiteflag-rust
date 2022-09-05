#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod codec_tests;

#[cfg(test)]
mod validation_test;

#[allow(dead_code)]
pub mod definitions;
mod field;
mod field_definition;
mod field_definition_parser;
mod header_order;

pub use definitions::{generic_header_fields, get_body_from_code, message_code, test_message_code};
pub use field::Field;
pub use field_definition::FieldDefinition;
pub use field_definition_parser::{FieldDefinitionParser, FieldDefinitionParserBase};
pub use header_order::MessageHeaderOrder;

use definitions::get_body_from_code_char;

pub const FIELD_PREFIX: &'static str = "Prefix";
pub const FIELD_VERSION: &'static str = "Version";
pub const FIELD_MESSAGETYPE: &'static str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &'static str = "PseudoMessageCode";

pub trait FieldValue: AsRef<str> + Into<String> + std::fmt::Debug {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> + std::fmt::Debug {}

impl From<&Field> for Vec<u8> {
    fn from(field: &Field) -> Self {
        field.encode()
    }
}

/**
 * Gets the value of the field specified by name
 * @param fieldname the name of the requested field
 * @return the field value, or NULL if field does not exist
 */
pub fn get_field_value_from_array<T: AsRef<str>>(
    fields: &[Field],
    field_name: T,
) -> Option<&String> {
    fields
        .iter()
        .find(|f| f.get_name() == field_name.as_ref())
        .map(|s| s.get())
}

pub fn get_message_code(fields: &[Field]) -> char {
    get_message_code_base(fields, FIELD_MESSAGETYPE)
}

pub fn get_test_message_code(fields: &[Field]) -> char {
    get_message_code_base(fields, FIELD_TESTMESSAGETYPE)
}

fn get_message_code_base(fields: &[Field], name: &'static str) -> char {
    match get_field_value_from_array(fields, name) {
        Some(x) => x.chars().next(),
        _ => None,
    }
    .expect("expected message code but none was found")
}

pub fn get_message_body(fields: &[Field]) -> (Vec<FieldDefinition>, char) {
    let message_code = get_message_code(fields);
    let body = get_body_from_code_char(&message_code);

    (body, message_code)
}
