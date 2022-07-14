pub const FIELD_PREFIX: &'static str = "Prefix";
pub const FIELD_VERSION: &'static str = "Version";
pub const FIELD_MESSAGETYPE: &'static str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &'static str = "PseudoMessageCode";

#[cfg(test)]
mod codec_tests;

mod definitions;
mod field;
mod field_definition;

pub use definitions::{generic_header_fields, get_body_from_code};
pub use field::Field;
