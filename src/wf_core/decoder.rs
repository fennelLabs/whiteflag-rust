use super::basic_message::BasicMessage;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_field::definitions::{
    convert_value_to_code, get_body_from_code_char, test_message_code,
};
use crate::wf_field::{create_request_fields, Field, FieldDefinition};
use crate::wf_parser::FieldDefinitionParser;
use crate::wf_parser::MessageHeaderFields;

pub struct Decoder {
    buffer: WhiteflagBuffer,
    header: MessageHeaderFields,
    bit_cursor: usize,
}

impl Decoder {
    pub fn new<T: AsRef<str>>(message: T) -> Self {
        let mut buffer = match WhiteflagBuffer::decode_from_hexadecimal(message) {
            Ok(buffer) => buffer,
            Err(e) => panic!("{}", e),
        };

        let (bit_cursor, header) = MessageHeaderFields::from_buffer(&mut buffer);

        Decoder {
            bit_cursor,
            buffer,
            header,
        }
    }

    pub fn decode(mut self) -> BasicMessage {
        let mut body: Vec<Field> = Vec::new();

        let code = match self.header.get_code() {
            'T' => {
                let field = self.decode_field(test_message_code());
                let psuedo_message_code = convert_value_to_code(field.get());
                body.push(field);
                psuedo_message_code
            }
            code => code,
        };

        let mut field_body = self.decode_fields(get_body_from_code_char(&code));
        body.append(field_body.as_mut());

        if code == 'Q' {
            // one request object requires 2 fields of 8 bits
            let n = (self.buffer.bit_length() - self.bit_cursor) / 16;
            body.append(create_request_fields(n, &mut self).as_mut());
        }

        BasicMessage::new(code, self.header.to_vec(), body, None, None)
    }

    fn decode_field(&mut self, definition: FieldDefinition) -> Field {
        // extract field
        let psuedo_message_code = self
            .buffer
            .extract_message_value(&definition, self.bit_cursor);

        // if this is a test message, then the pseudo message code data needs to be ignored
        // in order to achieve this, the bit cursor needs to be shifted
        // shift the bit the bit cursor instructs the program where the data extraction should begin
        self.bit_cursor += definition.bit_length();

        Field::new(definition, psuedo_message_code)
    }

    fn decode_fields(&mut self, defs: Vec<FieldDefinition>) -> Vec<Field> {
        let (cursor, fields) = self.buffer.decode(defs, self.bit_cursor);
        self.bit_cursor = cursor;
        fields
    }
}

impl FieldDefinitionParser for Decoder {
    fn parse(&mut self, definition: &FieldDefinition) -> String {
        let value = self
            .buffer
            .extract_message_value(definition, self.bit_cursor);
        self.bit_cursor += definition.bit_length();
        value
    }
}
