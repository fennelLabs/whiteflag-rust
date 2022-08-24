#[cfg(test)]
mod test;

mod message_code_parser;
mod message_header_parser;
mod parsed_field_definition;
mod wf_header;

pub use message_code_parser::MessageCodeParser;
pub use message_header_parser::MessageHeaderParser;
pub use parsed_field_definition::ParsedFieldDefinition;
pub use wf_header::{MessageHeaderFields, MessageHeaderValues};

use crate::{
    wf_core::{basic_message::BasicMessage, FieldValue},
    wf_field::{create_request_fields, definitions::convert_value_to_code, Field, FieldDefinition},
    wf_validation::Validation,
};

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

pub struct SerializedMessageParser {
    message: String,
}

impl FieldDefinitionParser for SerializedMessageParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        if let Some(end) = definition.end_byte {
            self.message[definition.start_byte..end].to_owned()
        } else {
            self.message[definition.start_byte..].to_owned()
        }
    }

    fn remaining(&self) -> usize {
        todo!()
    }

    fn body_field_definitions(&self) -> MessageCodeParser {
        todo!()
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

pub fn from_field_values<T: FieldValue>(
    data: &[T],
) -> WhiteflagMessageBuilder<FieldValuesParser<T>> {
    let parser = FieldValuesParser { data, index: 0 };
    WhiteflagMessageBuilder { parser }
}

impl<F: FieldDefinitionParser> WhiteflagMessageBuilder<F> {
    pub fn compile(mut self) -> BasicMessage {
        let header = self
            .convert_values_to_fields(crate::wf_field::definitions::Header::DEFINITIONS.to_vec());

        let code_parser = self.parser.body_field_definitions();
        let body_defs = code_parser.get_field_definitions_for_encode();

        let mut body = self.convert_values_to_fields(body_defs);

        if code_parser.code == 'Q' {
            body.append(create_request_fields(&mut self.parser).as_mut());
        }

        BasicMessage::new(code_parser.code, header, body, None, None)
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

pub fn from_serialized(serialized: &str, definitions: &[FieldDefinition]) -> Vec<String> {
    definitions
        .iter()
        .map(|d| {
            if let Some(end) = d.end_byte {
                serialized[d.start_byte..end].to_owned()
            } else {
                serialized[d.start_byte..].to_owned()
            }
        })
        .collect()
}
