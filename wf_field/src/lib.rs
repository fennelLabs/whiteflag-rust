#[cfg(test)]
mod codec_tests;

#[cfg(test)]
mod validation_test;

mod byte_configuration;
#[allow(dead_code)]
mod codec_positions;
#[allow(dead_code)]
pub mod definitions;
mod field;
mod field_definition;
mod field_definition_parser;
mod request;
mod types;
mod error;

#[cfg(test)]
mod test_field_definition;

pub use {
    field::Field,
    field_definition::FieldDefinition,
    field_definition_parser::{FieldDefinitionParser, FieldDefinitionParserBase},
    request::create_request_fields,
    types::MessageType,
};

pub trait FieldValue: AsRef<str> + Into<String> + std::fmt::Debug {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> + std::fmt::Debug {}

impl From<&Field> for Vec<u8> {
    fn from(field: &Field) -> Self {
        field.encode()
    }
}

pub trait MessageHeader {
    type Target: ?Sized;

    fn prefix(&self) -> &Self::Target;
    fn version(&self) -> &Self::Target;
    fn encryption_indicator(&self) -> &Self::Target;
    fn duress_indicator(&self) -> &Self::Target;
    fn message_code(&self) -> &Self::Target;
    fn reference_indicator(&self) -> &Self::Target;
    fn referenced_message(&self) -> &Self::Target;
}

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
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn get<'b>(&'a self, fields: &'b [Field]) -> &'b Field {
        &fields[self.as_usize()]
    }
}
