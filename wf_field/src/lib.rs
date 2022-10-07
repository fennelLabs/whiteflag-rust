#[cfg(test)]
mod codec_tests;

#[cfg(test)]
mod validation_test;

mod byte_configuration;
#[allow(dead_code)]
mod codec_positions;
#[allow(dead_code)]
pub mod definitions;
mod error;
mod field;
mod field_definition;
mod field_definition_parser;
mod message_body_type;
mod message_header;
mod request;
mod types;

#[cfg(test)]
mod test_field_definition;

pub use {
    field::Field,
    field_definition::FieldDefinition,
    field_definition_parser::{FieldDefinitionParser, FieldDefinitionParserBase},
    message_header::MessageHeaderOrder,
    request::create_request_fields,
    types::MessageCodeType,
};

pub trait FieldValue: AsRef<str> + Into<String> + std::fmt::Debug {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> + std::fmt::Debug {}

impl From<&Field> for Vec<u8> {
    fn from(field: &Field) -> Self {
        field.encode()
    }
}
