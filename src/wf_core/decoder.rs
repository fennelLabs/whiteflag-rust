use super::basic_message::BasicMessage;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_field::definitions::{
    convert_value_to_code, get_body_from_code_char, test_message_code,
};
use crate::wf_field::{create_request_fields, generic_header_fields, Field, FieldDefinition};
use crate::wf_parser::MessageHeaderOrder;

pub struct Decoder {
    buffer: WhiteflagBuffer,
    header: Vec<Field>,
    bit_cursor: usize,
}

impl Decoder {
    pub fn new<T: AsRef<str>>(message: T) -> Self {
        let buffer = match WhiteflagBuffer::decode_from_hexadecimal(message) {
            Ok(buffer) => buffer,
            Err(e) => panic!("{}", e),
        };

        let (bit_cursor, header) = buffer.decode(generic_header_fields().to_vec(), 0);

        Decoder {
            bit_cursor,
            buffer,
            header,
        }
    }

    pub fn decode(mut self) -> BasicMessage {
        let mut body: Vec<Field> = Vec::new();

        let code = match MessageHeaderOrder::get_code(&self.header).1 {
            'T' => {
                let field = self.decode_field(test_message_code());
                let psuedo_message_code = convert_value_to_code(field.get());
                body.push(field);
                psuedo_message_code
            }
            code => code,
        };

        let (cursor, mut field_body) = self
            .buffer
            .decode(get_body_from_code_char(&code), self.bit_cursor);
        body.append(field_body.as_mut());

        if code == 'Q' {
            // one request object requires 2 fields of 8 bits
            let n = (self.buffer.bit_length() - cursor) / 16;
            body.append(
                self.buffer
                    .decode(create_request_fields(n), cursor)
                    .1
                    .as_mut(),
            );
        }

        BasicMessage::new(code, self.header, body)
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
}
