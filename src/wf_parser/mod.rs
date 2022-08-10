#[cfg(test)]
mod test;

mod message_code_parser;
mod message_header_parser;
mod parsed_field_definition;
mod wf_header;

pub use message_code_parser::MessageCodeParser;
pub use message_header_parser::MessageHeaderParser;
pub use wf_header::MessageHeaderFields;

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

pub trait Parser {
    fn parse(&mut self, i: usize, definition: &FieldDefinition) -> String;
}

pub struct WhiteflagMessageBuilder<'a, T: FieldValue> {
    header: MessageHeaderFields,
    data: &'a [T],
    index: usize,
}

impl<'a, T: FieldValue> WhiteflagMessageBuilder<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        let (index, header) = MessageHeaderFields::from_values(data);
        WhiteflagMessageBuilder {
            header,
            data,
            index,
        }
    }

    pub fn compile(mut self) -> BasicMessage {
        let code = &self.header.get_code();
        let body_defs = get_body_from_code_char(&code);

        let mut body = convert_values_to_fields(body_defs, self.data.as_ref(), self.index);

        if code == &'Q' {
            let n = (self.data.len() - self.index) / 2;
            body.append(create_request_fields(n, &mut self).as_mut());
        }

        BasicMessage::new(*code, self.header.to_vec(), body)
    }
}

// need to mut index for each field parsed...

impl<'a, T: FieldValue> Parser for WhiteflagMessageBuilder<'a, T> {
    fn parse(&mut self, i: usize, definition: &FieldDefinition) -> String {
        let value = self.data[i].as_ref();

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

impl MessageHeaderFields {
    pub fn from_values<T: FieldValue>(data: &[T]) -> (usize, Self) {
        let defs = crate::wf_field::definitions::Header::DEFINITIONS;
        let header = convert_values_to_fields(defs.to_vec(), data.as_ref(), 0);
        (header.len(), MessageHeaderFields::from_fields(header))
    }
}

/// converts string values to their respective fields relative to their position and the corresponding field definition
pub fn convert_values_to_fields<T: FieldValue>(
    field_defs: Vec<FieldDefinition>,
    data: &[T],
    start_index: usize,
) -> Vec<Field> {
    if (data.len() - start_index) < field_defs.len() {
        panic!("not enough field definitions to process given values\nvalues: {:#?}\ndefinitions: {:#?}", data, field_defs);
    }

    let mut index = start_index;
    field_defs
        .into_iter()
        .map(|f| {
            let value = data[index].as_ref();
            match f.validate(value) {
                Err(e) => panic!(
                    "{} error while converting array of strings into fields\n{0:?}",
                    e
                ),
                _ => (),
            };

            let field = Field::new(f, value.into());

            index += 1;
            field
        })
        .collect()
}
