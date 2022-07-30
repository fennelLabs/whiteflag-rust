use super::basic_message::BasicMessage;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_field::definitions::{
    convert_value_to_code, get_body_from_code_char, test_message_code,
};
use crate::wf_field::{generic_header_fields, Field, FieldDefinition};
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
            /* 'Q' => {
                // one request object requires 2 fields of 8 bits
                let request_objects_length = (self.buffer.bit_length() - self.bit_cursor) /16;
            } */
            code => code,
        };

        /* Extend request message body with request fields (calculated from remaining bits) */
        /* final int nRequestObjects = (msgBuffer.bitLength() - bitCursor) / 16;   // One request object requires 2 fields of 8 bits
        body.append(new WfMessageSegment(messageType.createRequestFields(nRequestObjects)));
        break; */

        body.append(
            self.buffer
                .decode(get_body_from_code_char(&code), self.bit_cursor)
                .1
                .as_mut(),
        );

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

/* public final WfMessageCreator decode(final WfBinaryBuffer msgBuffer) throws WfCoreException {
    /* Keep track of fields and bit position */
    int bitCursor = 0;
    int nextField = 0;

    /* Decode message header, and determine message type */
    header = new WfMessageSegment(messageType.getHeaderFields());
    header.decode(msgBuffer, bitCursor, nextField);
    bitCursor += header.bitLength();
    messageType = WfMessageType.fromCode(header.get(FIELD_MESSAGETYPE));

    /* Decode message body and add fields as required for certain message types */
    body = new WfMessageSegment(messageType.getBodyFields());
    body.decode(msgBuffer, bitCursor, nextField);
    nextField = body.getNoFields();
    bitCursor += body.bitLength();
    switch (messageType) {
        case T:
            /* Determine pseudo message type and extend test message body with pseudo message body */
            final WfMessageType pseudoMessageType = WfMessageType.fromCode(body.get(FIELD_TESTMESSAGETYPE));
            body.append(new WfMessageSegment(pseudoMessageType.getBodyFields()));
            break;
        case Q:
            /* Extend request message body with request fields (calculated from remaining bits) */
            final int nRequestObjects = (msgBuffer.bitLength() - bitCursor) / 16;   // One request object requires 2 fields of 8 bits
            body.append(new WfMessageSegment(messageType.createRequestFields(nRequestObjects)));
            break;
        default:
            break;
    }
    body.decode(msgBuffer, bitCursor, nextField);
    return this;
} */
