#[cfg(test)]
mod codec_tests;

mod definitions;
mod field;
mod field_definition;

pub use field_definition::FieldDefinition;
pub use definitions::{generic_header_fields, get_body_from_code};
use crate::wf_buffer::WhiteflagBuffer;
pub use field::Field;

pub const FIELD_PREFIX: &'static str = "Prefix";
pub const FIELD_VERSION: &'static str = "Version";
pub const FIELD_MESSAGETYPE: &'static str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &'static str = "PseudoMessageCode";

impl From<&Field> for Vec<u8> {
    fn from(field: &Field) -> Self {
        field.encode().expect("field has no value")
    }
}

impl From<&Field> for WhiteflagBuffer {
    fn from(field: &Field) -> Self {
        let length = field.bit_length();
        WhiteflagBuffer::new(field.into(), length)
    }
}

impl WhiteflagBuffer {
    pub fn append_field(&mut self, field: &Field) {
        self.append(field.into());
    }
}
