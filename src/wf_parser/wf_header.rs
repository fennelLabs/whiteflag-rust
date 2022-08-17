use crate::wf_field::definitions::{convert_value_to_code, get_body_from_code};
use crate::wf_field::Field;
use crate::{wf_buffer::WhiteflagBuffer, wf_field::FieldDefinition};

pub struct MessageHeader {
    prefix: String,
    version: String,
    encryption_indicator: String,
    duress_indicator: String,
    message_code: String,
    reference_indicator: String,
    referenced_message: String,
}

impl MessageHeader {
    pub fn from_serialized(serialized: &str) -> MessageHeader {
        let fields: Vec<String> = super::from_serialized(serialized, crate::wf_field::definitions::Header::DEFINITIONS);

        MessageHeader {
            prefix: fields.remove(0),
            version: fields.remove(0),
            encryption_indicator: fields.remove(0),
            duress_indicator: fields.remove(0),
            message_code: fields.remove(0),
            reference_indicator: fields.remove(0),
            referenced_message: fields.remove(0),
        }
    }

    pub fn get_body_field_definitions(&self) -> Vec<FieldDefinition> {
        get_body_from_code(&self.message_code)
    }

    pub fn to_vec(self) -> Vec<String> {
        vec![
            self.prefix,
            self.version,
            self.encryption_indicator,
            self.duress_indicator,
            self.message_code,
            self.reference_indicator,
            self.referenced_message,
        ]
    }
}

pub struct MessageHeaderFields {
    prefix: Field,
    version: Field,
    encryption_indicator: Field,
    duress_indicator: Field,
    message_code: Field,
    reference_indicator: Field,
    referenced_message: Field,
}

impl MessageHeaderFields {
    pub fn from_buffer(buffer: &WhiteflagBuffer) -> (usize, MessageHeaderFields) {
        let (cursor, header) = buffer.decode(
            crate::wf_field::definitions::Header::DEFINITIONS.to_vec(),
            0,
        );
        (cursor, Self::from_fields(header))
    }

    pub fn from_fields(mut fields: Vec<Field>) -> MessageHeaderFields {
        MessageHeaderFields {
            prefix: fields.remove(0),
            version: fields.remove(0),
            encryption_indicator: fields.remove(0),
            duress_indicator: fields.remove(0),
            message_code: fields.remove(0),
            reference_indicator: fields.remove(0),
            referenced_message: fields.remove(0),
        }
    }

    pub fn get_code(&self) -> char {
        convert_value_to_code(self.message_code.get())
    }

    pub fn to_vec(self) -> Vec<Field> {
        vec![
            self.prefix,
            self.version,
            self.encryption_indicator,
            self.duress_indicator,
            self.message_code,
            self.reference_indicator,
            self.referenced_message,
        ]
    }
}

pub fn convert_header_definitions<F>(convert: F) -> Vec<Field>
where
    F: Fn((usize, &FieldDefinition)) -> Field,
{
    crate::wf_field::definitions::Header::DEFINITIONS
        .iter()
        .enumerate()
        .map(convert)
        .collect()
}
