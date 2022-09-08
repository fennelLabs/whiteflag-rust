use wf_buffer::WhiteflagBuffer;
use wf_field::{
    convert_value_to_code, definitions, get_body_from_code, Field, FieldDefinition, MessageHeader,
    MessageHeaderOrder,
};

pub struct MessageHeaderValues {
    values: Vec<String>,
}

pub fn from_serialized(serialized: &str, definitions: &[FieldDefinition]) -> Vec<String> {
    definitions
        .iter()
        .map(|d| {
            if let Some(end) = d.positions.bytes.end {
                serialized[d.positions.bytes.start..end].to_owned()
            } else {
                serialized[d.positions.bytes.start..].to_owned()
            }
        })
        .collect()
}

impl MessageHeader for MessageHeaderValues {
    type Target = str;

    fn prefix(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::Prefix.as_usize()]
    }

    fn version(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::Version.as_usize()]
    }

    fn encryption_indicator(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::EncryptionIndicator.as_usize()]
    }

    fn duress_indicator(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::DuressIndicator.as_usize()]
    }

    fn message_code(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::MessageCode.as_usize()]
    }

    fn reference_indicator(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::ReferenceIndicator.as_usize()]
    }

    fn referenced_message(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::ReferencedMessage.as_usize()]
    }
}

impl MessageHeaderValues {
    pub fn from_serialized(serialized: &str) -> MessageHeaderValues {
        let fields: Vec<String> = from_serialized(serialized, definitions::header::DEFINITIONS);

        MessageHeaderValues { values: fields }
    }

    pub fn get_body_field_definitions(&self) -> Vec<FieldDefinition> {
        get_body_from_code(&self.message_code())
    }

    pub fn to_vec(self) -> Vec<String> {
        self.values
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
        let (cursor, header) = buffer.decode(definitions::header::DEFINITIONS, 0);
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

pub fn convert_definitions<F>(
    defs: &'static [FieldDefinition],
    convert: F,
) -> impl Iterator<Item = Field>
where
    F: Fn((usize, &'static FieldDefinition)) -> Field,
{
    defs.iter().enumerate().map(convert)
}

pub fn convert_header_definitions<F>(convert: F) -> impl Iterator<Item = Field>
where
    F: Fn((usize, &'static FieldDefinition)) -> Field,
{
    convert_definitions(definitions::header::DEFINITIONS, convert)
}
