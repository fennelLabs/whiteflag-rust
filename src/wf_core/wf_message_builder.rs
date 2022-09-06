use super::message::Message;
use std::ops::Div;
use wf_buffer::WhiteflagBuffer;
use wf_field::{FieldDefinition, FieldDefinitionParser, FieldValue, Parser};
use wf_parser::MessageCodeParser;
use wf_validation::Validation;

pub struct SerializedMessageParser<'a> {
    message: &'a str,
    last_byte: usize,
}

impl FieldDefinitionParser for SerializedMessageParser<'_> {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        if let Some(end) = definition.positions.end {
            self.last_byte = end;
            self.message[definition.positions.start..end].to_owned()
        } else {
            self.last_byte = self.message.len();
            self.message[definition.positions.start..].to_owned()
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

impl<T: FieldValue> FieldDefinitionParser for FieldValuesParser<'_, T> {
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
    pub fn compile(self) -> Message {
        let message = Parser::parse(self.parser);
        Message::new(message.code, message.header, message.body, None, None)
    }
}
