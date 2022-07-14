use crate::{wf_buffer::WhiteflagBuffer, wf_core::field::Field};

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
