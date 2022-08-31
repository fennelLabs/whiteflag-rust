#[cfg(test)]
mod test;

mod message_code_parser;
mod message_header_parser;
mod parsed_field_definition;
mod request;
mod wf_header;

pub use request::create_request_fields;

use std::ops::Div;

pub use message_code_parser::MessageCodeParser;
pub use message_header_parser::MessageHeaderParser;
pub use parsed_field_definition::ParsedFieldDefinition;
pub use wf_header::{MessageHeaderFields, MessageHeaderValues};

use crate::{
    wf_core::{message::Message, FieldValue},
    wf_field::{definitions::convert_value_to_code, Field, FieldDefinition},
};

use wf_validation::Validation;

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

    pub fn get_code(fields: &[Field]) -> (&Field, char) {
        let field = Self::MessageCode.get(fields);
        (field, convert_value_to_code(field.get()))
    }
}

pub trait FieldDefinitionParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
    /// fetch the field definitions for the body
    fn body_field_definitions(&self) -> MessageCodeParser;
    /// meant to calculate remaining values (if any) for request field definitions
    fn remaining(&self) -> usize;
}

pub struct SerializedMessageParser<'a> {
    message: &'a str,
    last_byte: usize,
}

impl FieldDefinitionParser for SerializedMessageParser<'_> {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        if let Some(end) = definition.end_byte {
            self.last_byte = end;
            self.message[definition.start_byte..end].to_owned()
        } else {
            self.last_byte = self.message.len();
            self.message[definition.start_byte..].to_owned()
        }
    }

    fn remaining(&self) -> usize {
        (self.message.len() - self.last_byte).div(4)
    }

    fn body_field_definitions(&self) -> MessageCodeParser {
        MessageCodeParser::parse_from_serialized(&self.message)
    }
}

pub struct FieldValuesParser<'a, T: FieldValue> {
    data: &'a [T],
    index: usize,
}

impl<'a, T: FieldValue> FieldDefinitionParser for FieldValuesParser<'a, T> {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        let value = self.data[self.index].as_ref();

        match definition.validate(value) {
            Err(e) => panic!(
                "{} error while converting array of strings into fields\n{0:?}",
                e
            ),
            _ => (),
        };

        self.index += 1;

        value.into()
    }

    fn remaining(&self) -> usize {
        (self.data.len() - self.index) / 2
    }

    fn body_field_definitions(&self) -> MessageCodeParser {
        MessageCodeParser::parse_for_encode(self.data)
    }
}

pub struct WhiteflagMessageBuilder<F: FieldDefinitionParser> {
    parser: F,
}

pub fn builder_from_field_values<T: FieldValue>(
    data: &[T],
) -> WhiteflagMessageBuilder<FieldValuesParser<T>> {
    let parser = FieldValuesParser { data, index: 0 };
    WhiteflagMessageBuilder { parser }
}

pub fn builder_from_serialized<'a>(
    message: &'a str,
) -> WhiteflagMessageBuilder<SerializedMessageParser<'a>> {
    let parser = SerializedMessageParser {
        message,
        last_byte: 0,
    };
    WhiteflagMessageBuilder { parser }
}

impl<F: FieldDefinitionParser> WhiteflagMessageBuilder<F> {
    pub fn compile(mut self) -> Message {
        let header = self
            .convert_values_to_fields(crate::wf_field::definitions::Header::DEFINITIONS.to_vec());

        let code_parser = self.parser.body_field_definitions();
        let body_defs = code_parser.get_field_definitions_for_encode();

        let mut body = self.convert_values_to_fields(body_defs);

        if code_parser.code == 'Q' {
            body.append(create_request_fields(&mut self.parser).as_mut());
        }

        Message::new(code_parser.code, header, body, None, None)
    }

    /// converts string values to their respective fields relative to their position and the corresponding field definition
    fn convert_values_to_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field> {
        /* if self.data.len() < field_defs.len() {
            panic!("not enough field definitions to process given values\nvalues: {:#?}\ndefinitions: {:#?}", self.data, field_defs);
        } */

        field_defs
            .into_iter()
            .map(|f| {
                let value = self.parser.parse(&f);
                Field::new(f, value)
            })
            .collect()
    }
}
