use std::ops::Div;
use wf_buffer::WhiteflagBuffer;
use wf_codec::CodecError;
use wf_field::{FieldDefinition, FieldDefinitionParser, FieldValue};
use wf_parser::Parser;
use wf_validation::Validation;

pub struct SerializedMessageParser<'a> {
    message: &'a str,
    last_byte: usize,
}

impl FieldDefinitionParser for SerializedMessageParser<'_> {
    fn parse(&mut self, definition: &FieldDefinition) -> Result<String, CodecError> {
        if let Some(end) = definition.positions.bytes.end {
            self.last_byte = end;
            Ok(self.message[definition.positions.bytes.start..end].to_owned())
        } else {
            self.last_byte = self.message.len();
            Ok(self.message[definition.positions.bytes.start..].to_owned())
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
    fn parse(&mut self, definition: &FieldDefinition) -> Result<String, CodecError> {
        // Bounds checking to prevent index out of bounds panic
        if self.index >= self.data.len() {
            return Err(CodecError::Validation {
                error: format!(
                    "Not enough field values provided. Expected at least {} values, but only {} were provided.",
                    self.index + 1,
                    self.data.len()
                ),
            });
        }

        let value = self.data[self.index].as_ref();

        if let Err(e) = definition.validate(value) {
            return Err(CodecError::Validation {
                error: e.to_string(),
            });
        };

        self.index += 1;

        Ok(value.into())
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
    fn parse(&mut self, definition: &FieldDefinition) -> Result<String, CodecError> {
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

pub fn builder_from_field_values<T: FieldValue>(data: &[T]) -> Result<Parser, wf_field::Error> {
    let parser = FieldValuesParser { data, index: 0 };
    Parser::parse(parser)
}

pub fn builder_from_serialized(message: &str) -> Result<Parser, wf_field::Error> {
    let parser = SerializedMessageParser {
        message,
        last_byte: 0,
    };
    Parser::parse(parser)
}

pub fn builder_from_encoded(message: WhiteflagBuffer) -> Result<Parser, wf_field::Error> {
    let parser = EncodedMessageParser {
        buffer: message,
        bit_cursor: 0,
    };
    Parser::parse(parser)
}

#[cfg(test)]
mod wf_message_builder_tests {
    use super::*;

    #[test]
    fn test_bounds_checking_with_insufficient_data() {
        // Test case that would previously cause index out of bounds panic
        let insufficient_data = vec!["WF"]; // Only 1 element, but parser needs at least 7 for header

        let result = builder_from_field_values(&insufficient_data);

        // Should return an error instead of panicking
        assert!(result.is_err());

        // Check that the error message contains our bounds checking message
        if let Err(e) = result {
            let error_string = format!("{e}");
            assert!(error_string.contains("Not enough field values provided"));
        }
    }

    #[test]
    fn test_bounds_checking_with_sufficient_data() {
        // Test with sufficient data - should work normally
        let sufficient_data = vec![
            "WF",                                                               // Prefix
            "1",                                                                // Version
            "0", // EncryptionIndicator
            "0", // DuressIndicator
            "A", // MessageCode
            "0", // ReferenceIndicator
            "0000000000000000000000000000000000000000000000000000000000000000", // ReferencedMessage
            "1", // VerificationMethod
            "https://test.com", // VerificationData
        ];

        let result = builder_from_field_values(&sufficient_data);

        // Should succeed with sufficient data
        assert!(result.is_ok());
    }
}
