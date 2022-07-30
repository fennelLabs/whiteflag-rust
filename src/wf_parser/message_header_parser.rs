use super::parsed_field_definition::ParsedFieldDefinition;
use super::{convert_value_to_code, MessageHeaderOrder};
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_codec::encoding::*;
use crate::wf_field::Field;
use crate::wf_field::{
    definitions::{message_code, test_message_code},
    FieldDefinition,
};
use regex::Regex;

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
    pub fn parse(buffer: &WhiteflagBuffer) -> MessageHeaderFields {
        let (bit_cursor, header) = buffer.decode(Self::default().to_vec(), 0);
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
        buffer.decode(self.to_vec(), 0).1
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
        vec![
            self.prefix.into(),
            self.version.into(),
            self.encryption_indicator.into(),
            self.duress_indicator.into(),
            self.message_code.into(),
            self.reference_indicator.into(),
            self.referenced_message.into(),
        ]
    }
}

impl Default for MessageHeaderParser {
    fn default() -> Self {
        let prefix = FieldDefinition::new("Prefix", Regex::new("^WF$").ok(), UTF8, 0, 2);
        let version = FieldDefinition::new("Version", Regex::new("^[A-Z0-9]{1}$").ok(), UTF8, 2, 3); //"(?=1)^[A-Z0-9]{1}$"
        let encryption_indicator = FieldDefinition::new(
            "EncryptionIndicator",
            Regex::new("^[A-Z0-9]{1}$").ok(), //"(?=0|1|2)^[A-Z0-9]{1}$"
            UTF8,
            3,
            4,
        );
        let duress_indicator =
            FieldDefinition::new("DuressIndicator", Regex::new("^[0-1]{1}$").ok(), BIN, 4, 5);
        let message_code = message_code();
        let reference_indicator = FieldDefinition::new(
            "ReferenceIndicator",
            Regex::new(
                ["^", HEX.charset, "{1}$"] //"(?=0|1|2|3|4|5|6|7|8|9)^", HEX.charset, "{1}$"
                    .concat()
                    .as_str(),
            )
            .ok(),
            HEX,
            6,
            7,
        );
        let referenced_message = FieldDefinition::new(
            "ReferencedMessage",
            Regex::new(["^", HEX.charset, "{64}$"].concat().as_str()).ok(),
            HEX,
            7,
            71,
        );

        let mut parsed_defs = ParsedFieldDefinition::parse(vec![
            prefix,
            version,
            encryption_indicator,
            duress_indicator,
            message_code,
            reference_indicator,
            referenced_message,
        ]);

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
}

/* impl std::ops::Deref for MessageHeaderParser {
    type Target = [FieldDefinition];

    fn deref(&self) -> &Self::Target {
        .as_slice()
    }
} */
