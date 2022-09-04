use std::ops::Div;
use super::{message::Message, request::create_request_fields};
use wf_buffer::WhiteflagBuffer;
use wf_field::{definitions::convert_value_to_code, Field, FieldDefinition, FieldValue};
use wf_parser::{MessageCodeParser, MessageHeaderOrder};
use wf_validation::Validation;

pub trait FieldDefinitionParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
    /// fetch the field definitions for the body
    fn body_field_definitions(&self) -> Vec<FieldDefinition>;
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

    fn body_field_definitions(&self) -> Vec<FieldDefinition> {
        MessageCodeParser::parse_from_serialized(&self.message).get_field_definitions()
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

    fn body_field_definitions(&self) -> Vec<FieldDefinition> {
        MessageCodeParser::parse_for_encode(self.data).get_field_definitions()
    }
}

pub struct EncodedMessageParser {
    buffer: WhiteflagBuffer,
    bit_cursor: usize,
}

impl FieldDefinitionParser for EncodedMessageParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        let value = self
            .buffer
            .extract_message_value(definition, self.bit_cursor);
        self.bit_cursor += definition.bit_length();
        value
    }

    fn remaining(&self) -> usize {
        (self.buffer.bit_length() - self.bit_cursor) / 16
    }

    fn body_field_definitions(&self) -> Vec<FieldDefinition> {
        MessageCodeParser::parse_for_decode(&self.buffer).get_field_definitions()
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

pub fn builder_from_encoded(
    message: WhiteflagBuffer,
) -> WhiteflagMessageBuilder<EncodedMessageParser> {
    let parser = EncodedMessageParser {
        buffer: message,
        bit_cursor: 0,
    };
    WhiteflagMessageBuilder { parser }
}

impl<F: FieldDefinitionParser> WhiteflagMessageBuilder<F> {
    pub fn compile(mut self) -> Message {
        let header =
            self.convert_values_to_fields(wf_field::definitions::Header::DEFINITIONS.to_vec());

        let code = convert_value_to_code(header[MessageHeaderOrder::MessageCode.as_usize()].get());

        let body_defs = self.parser.body_field_definitions();

        let mut body = self.convert_values_to_fields(body_defs);

        if code == 'Q' {
            body.append(create_request_fields(&mut self.parser).as_mut());
        }

        Message::new(code, header, body, None, None)
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