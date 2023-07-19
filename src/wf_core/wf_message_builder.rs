use std::ops::Div;
use wf_buffer::WhiteflagBuffer;
use wf_field::{FieldDefinition, FieldDefinitionParser, FieldValue};
use wf_parser::Parser;
use wf_validation::Validation;

pub struct SerializedMessageParser<'a> {
    message: &'a str,
    last_byte: usize,
}

impl FieldDefinitionParser for SerializedMessageParser<'_> {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        if let Some(end) = definition.positions.bytes.end {
            self.last_byte = end;
            self.message[definition.positions.bytes.start..end].to_owned()
        } else {
            self.last_byte = self.message.len();
            self.message[definition.positions.bytes.start..].to_owned()
        }
    }

    fn remaining(&self) -> usize {
        (self.message.len() - self.last_byte).div(4)
    }
}

pub struct FieldValuesParser<'a, T: FieldValue> {
    data: &'a [T],
    index: usize,
}

impl<T: FieldValue> FieldDefinitionParser for FieldValuesParser<'_, T> {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        let value = self.data[self.index].as_ref();

        if let Err(e) = definition.validate(value) {
            println!("current value: {}", value);
            panic!(
                "{} error while converting array of strings into fields\n{0:?}",
                e
            )
        };

        self.index += 1;

        value.into()
    }

    fn remaining(&self) -> usize {
        (self.data.len() - self.index) / 2
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
}

pub fn builder_from_field_values<T: FieldValue>(data: &[T]) -> Parser {
    let parser = FieldValuesParser { data, index: 0 };
    Parser::parse(parser)
}

pub fn builder_from_serialized(message: &str) -> Parser {
    let parser = SerializedMessageParser {
        message,
        last_byte: 0,
    };
    Parser::parse(parser)
}

pub fn builder_from_encoded(message: WhiteflagBuffer) -> Parser {
    let parser = EncodedMessageParser {
        buffer: message,
        bit_cursor: 0,
    };
    Parser::parse(parser)
}
