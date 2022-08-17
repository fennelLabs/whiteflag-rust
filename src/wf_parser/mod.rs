#[cfg(test)]
mod test;

mod message_code_parser;
mod message_header_parser;
mod parsed_field_definition;
mod wf_header;

pub use message_code_parser::MessageCodeParser;
pub use message_header_parser::MessageHeaderParser;
pub use wf_header::{MessageHeaderFields, MessageHeader};

use crate::{
    wf_core::{basic_message::BasicMessage, FieldValue},
    wf_field::{
        create_request_fields,
        definitions::{convert_value_to_code, get_body_from_code_char},
        Field, FieldDefinition,
    },
    wf_validation::Validation,
};

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
    pub fn get<'b>(&'a self, fields: &'b [Field]) -> &'b Field {
        let index: usize = *self as usize;
        &fields[index]
    }

    pub fn get_code(fields: &[Field]) -> (&Field, char) {
        let field = Self::MessageCode.get(fields);
        (field, convert_value_to_code(field.get()))
    }
}

pub trait FieldDefinitionParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
}

pub struct WhiteflagMessageBuilder<'a, T: FieldValue> {
    data: &'a [T],
    index: usize,
}

impl<'a, T: FieldValue> WhiteflagMessageBuilder<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        WhiteflagMessageBuilder { data, index: 0 }
    }

    pub fn compile(mut self) -> BasicMessage {
        let defs = crate::wf_field::definitions::Header::DEFINITIONS;
        let header = self.convert_values_to_fields(defs.to_vec());

        let header = MessageHeaderFields::from_fields(header);
        let code = header.get_code();
        let body_defs =
            MessageCodeParser::parse_for_encode(self.data).get_field_definitions_for_encode();

        let mut body = self.convert_values_to_fields(body_defs);

        if code == 'Q' {
            let n = (self.data.len() - self.index) / 2;
            body.append(create_request_fields(n, &mut self).as_mut());
        }

        BasicMessage::new(code, header.to_vec(), body, None, None)
    }

    /// converts string values to their respective fields relative to their position and the corresponding field definition
    fn convert_values_to_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field> {
        if self.data.len() < field_defs.len() {
            panic!("not enough field definitions to process given values\nvalues: {:#?}\ndefinitions: {:#?}", self.data, field_defs);
        }

        field_defs
            .into_iter()
            .map(|f| {
                let value = self.parse(&f);
                Field::new(f, value)
            })
            .collect()
    }
}

impl<'a, T: FieldValue> FieldDefinitionParser for WhiteflagMessageBuilder<'a, T> {
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
}

pub fn from_serialized(serialized: &str, definitions: &[FieldDefinition]) -> Vec<String> {
    definitions.iter().map(|d| {
        if let Some(end) = d.end_byte {
            serialized[d.start_byte..end].to_owned()
        } else {
            serialized[d.start_byte..].to_owned()
        }
    }).collect()
}
