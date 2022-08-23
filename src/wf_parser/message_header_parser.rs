use super::parsed_field_definition::ParsedFieldDefinition;
use super::MessageHeaderOrder;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_field::definitions::{self, convert_value_to_code};
use crate::wf_field::Field;
use crate::wf_field::{definitions::test_message_code, FieldDefinition};

pub struct MessageHeaderParser {
    prefix: ParsedFieldDefinition,
    version: ParsedFieldDefinition,
    encryption_indicator: ParsedFieldDefinition,
    duress_indicator: ParsedFieldDefinition,
    message_code: ParsedFieldDefinition,
    reference_indicator: ParsedFieldDefinition,
    referenced_message: ParsedFieldDefinition,
}

pub struct MessageHeader {
    prefix: String,
    version: String,
    encryption_indicator: String,
    duress_indicator: String,
    message_code: char,
    reference_indicator: String,
    referenced_message: String,
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

trait MessageHeaderVariant<T> {
    fn get(&self, field: MessageHeaderOrder) -> &T;
    fn to_vec(&self) -> Vec<T>;
}

impl MessageHeaderVariant<Field> for MessageHeaderFields {
    fn get(&self, field: MessageHeaderOrder) -> &Field {
        match field {
            MessageHeaderOrder::Prefix => &self.prefix,
            MessageHeaderOrder::Version => &self.version,
            MessageHeaderOrder::EncryptionIndicator => &self.encryption_indicator,
            MessageHeaderOrder::DuressIndicator => &self.duress_indicator,
            MessageHeaderOrder::MessageCode => &self.message_code,
            MessageHeaderOrder::ReferenceIndicator => &self.reference_indicator,
            MessageHeaderOrder::ReferencedMessage => &self.referenced_message,
        }
    }

    fn to_vec(&self) -> Vec<Field> {
        vec![
            self.get(MessageHeaderOrder::Prefix).to_owned(),
            self.get(MessageHeaderOrder::Version).to_owned(),
            self.get(MessageHeaderOrder::EncryptionIndicator).to_owned(),
            self.get(MessageHeaderOrder::DuressIndicator).to_owned(),
            self.get(MessageHeaderOrder::MessageCode).to_owned(),
            self.get(MessageHeaderOrder::ReferenceIndicator).to_owned(),
            self.get(MessageHeaderOrder::ReferencedMessage).to_owned(),
        ]
    }
}

impl MessageHeaderFields {
    pub fn new(mut fields: Vec<Field>) -> Self {
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
}

impl MessageHeaderParser {
    pub fn new(defs: &[FieldDefinition]) -> Self {
        let mut parsed_defs = ParsedFieldDefinition::header();

        MessageHeaderParser {
            prefix: parsed_defs.remove(0),
            version: parsed_defs.remove(0),
            encryption_indicator: parsed_defs.remove(0),
            duress_indicator: parsed_defs.remove(0),
            message_code: parsed_defs.remove(0),
            reference_indicator: parsed_defs.remove(0),
            referenced_message: parsed_defs.remove(0),
        }
    }

    pub fn parse(buffer: &WhiteflagBuffer) -> MessageHeaderFields {
        let (bit_cursor, header) = buffer.decode(&Self::default().to_vec(), 0);
        let code = MessageHeaderOrder::MessageCode.get(&header);

        MessageHeaderFields::new(header)
    }

    pub fn extract(&self, buffer: &WhiteflagBuffer) -> MessageHeader {
        MessageHeader {
            prefix: self.prefix.extract(&buffer),
            version: self.version.extract(&buffer),
            encryption_indicator: self.encryption_indicator.extract(&buffer),
            duress_indicator: self.duress_indicator.extract(&buffer),
            message_code: convert_value_to_code(&self.message_code.extract(&buffer)),
            reference_indicator: self.reference_indicator.extract(&buffer),
            referenced_message: self.referenced_message.extract(&buffer),
        }
    }

    pub fn to_fields(self, buffer: &WhiteflagBuffer) -> Vec<Field> {
        buffer.decode(self.to_vec().as_slice(), 0).1
        //self.to_vec().into_iter().map(|f| )
    }

    /// message code is a default header
    pub fn message_code(&self) -> &ParsedFieldDefinition {
        &self.message_code
    }

    /// the test message code is technically part of a test message body
    /// this field is always ordered after the header
    /// therefore, the `previous` position is the `end_bit` of the last header `ParsedFieldDefinition`
    pub fn test_message_code(&self) -> ParsedFieldDefinition {
        self.referenced_message.next(test_message_code())
    }

    pub fn to_vec(self) -> Vec<FieldDefinition> {
        let defs: Vec<&'static FieldDefinition> = vec![
            self.prefix.into(),
            self.version.into(),
            self.encryption_indicator.into(),
            self.duress_indicator.into(),
            self.message_code.into(),
            self.reference_indicator.into(),
            self.referenced_message.into(),
        ];

        defs.into_iter().map(|f| f.to_owned()).collect()
    }
}

impl Default for MessageHeaderParser {
    fn default() -> Self {
        MessageHeaderParser::new(definitions::Header::DEFINITIONS)
    }
}

/* impl std::ops::Deref for MessageHeaderParser {
    type Target = [FieldDefinition];

    fn deref(&self) -> &Self::Target {
        .as_slice()
    }
} */
