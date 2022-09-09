#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod codec_tests;

#[cfg(test)]
mod validation_test;

mod byte_configuration;
mod codec_positions;
#[allow(dead_code)]
pub mod definitions;
mod field;
mod field_definition;
mod field_definition_parser;
mod header;
mod request;
mod types;

#[cfg(test)]
mod test_field_definition;

pub use {
    field::Field,
    field_definition::FieldDefinition,
    field_definition_parser::{FieldDefinitionParser, Parser},
    header::MessageHeaderOrder,
    types::MessageType,
};

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
