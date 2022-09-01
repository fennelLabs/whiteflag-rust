use super::{parsed_field_definition::ParsedFieldDefinition, MessageHeader, MessageHeaderOrder};
use wf_buffer::WhiteflagBuffer;
use wf_field::{definitions::test_message_code, Field, FieldDefinition};

pub struct MessageHeaderParser {
    values: Vec<ParsedFieldDefinition>,
}

impl MessageHeader for MessageHeaderParser {
    type Target = ParsedFieldDefinition;

    fn prefix(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::Prefix.as_usize()]
    }

    fn version(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::Version.as_usize()]
    }

    fn encryption_indicator(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::EncryptionIndicator.as_usize()]
    }

    fn duress_indicator(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::DuressIndicator.as_usize()]
    }

    fn message_code(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::MessageCode.as_usize()]
    }

    fn reference_indicator(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::ReferenceIndicator.as_usize()]
    }

    fn referenced_message(&self) -> &Self::Target {
        &self.values[MessageHeaderOrder::ReferencedMessage.as_usize()]
    }
}

impl MessageHeaderParser {
    pub fn new() -> Self {
        MessageHeaderParser {
            values: ParsedFieldDefinition::header(),
        }
    }

    pub fn to_fields(self, buffer: &WhiteflagBuffer) -> Vec<Field> {
        buffer.decode(self.to_vec().as_slice(), 0).1
    }

    /// the test message code is technically part of a test message body
    /// this field is always ordered after the header
    /// therefore, the `previous` position is the `end_bit` of the last header `ParsedFieldDefinition`
    pub fn test_message_code(&self) -> ParsedFieldDefinition {
        self.referenced_message().next(test_message_code())
    }

    pub fn to_vec_static(self) -> Vec<&'static FieldDefinition> {
        self.values.into_iter().map(|f| f.into()).collect()
    }

    pub fn to_vec(self) -> Vec<FieldDefinition> {
        self.values.into_iter().map(|f| f.to_definition()).collect()
    }
}

impl Default for MessageHeaderParser {
    fn default() -> Self {
        MessageHeaderParser::new()
    }
}
