#[cfg(test)]
mod codec_tests;

mod definitions;
mod field;
mod field_definition;

use crate::wf_buffer::WhiteflagBuffer;
pub use definitions::{generic_header_fields, get_body_from_code};
pub use field::Field;
pub use field_definition::FieldDefinition;

pub const FIELD_PREFIX: &'static str = "Prefix";
pub const FIELD_VERSION: &'static str = "Version";
pub const FIELD_MESSAGETYPE: &'static str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &'static str = "PseudoMessageCode";

impl From<&Field> for Vec<u8> {
    fn from(field: &Field) -> Self {
        field.encode()
    }
}

impl From<&Field> for WhiteflagBuffer {
    fn from(field: &Field) -> Self {
        let length = field.bit_length();
        WhiteflagBuffer::new(field.into(), length)
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
        .find(|f| f.definition.name == field_name.as_ref())
        .map(|s| s.get())
}

pub fn get_message_code(fields: &[Field]) -> char {
    match get_field_value_from_array(fields, &FIELD_MESSAGETYPE) {
        Some(x) => x.chars().next(),
        _ => None,
    }
    .expect("expected message code but none was found")
}

pub fn get_message_body(fields: &[Field]) -> (Vec<FieldDefinition>, char) {
    let message_code = get_message_code(fields);
    (get_body_from_code(&message_code), message_code)
}
