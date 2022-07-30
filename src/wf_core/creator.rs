use super::basic_message::BasicMessage;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_convert::FieldValue;
use crate::wf_field::definitions::{get_body_from_code_char, test_message_code};
use crate::wf_field::{generic_header_fields, get_body_from_code, Field};
use crate::wf_parser::MessageHeaderOrder;

pub const PREFIX: &str = "WF";
pub const PROTOCOL_VERSION: &str = "1";

pub fn encode<T: FieldValue>(fields: &[T]) -> String {
    let mut basic_message: BasicMessage = fields.into();
    let message_encoded = basic_message.encode();
    hex::encode(message_encoded)
}

/**
 * Decodes an encoded Whiteflag message and creates a new Whiteflag base message object
 * @since 1.1
 * @param msgBuffer a buffer with the compressed binary encoded message
 * @return this message creator
 * @throws WfCoreException if the encoded message is invalid
 */
pub fn decode<T: AsRef<str>>(message: T) -> BasicMessage {
    let buffer = match WhiteflagBuffer::decode_from_hexadecimal(message) {
        Ok(buffer) => buffer,
        Err(e) => panic!("{}", e),
    };
    //let mut next_field = 0;

    let (mut bit_cursor, header) = buffer.decode(generic_header_fields().to_vec(), 0);

    let mut body: Vec<Field> = vec![];
    let (_, code) = MessageHeaderOrder::get_code(&header);

    let definitions = match &code {
        'T' => {
            let test_def = test_message_code();
            let test_bit_length = test_def.bit_length();

            // extract the psuedo message field
            let field = buffer.extract_message_field(test_def, bit_cursor);
            let psuedo_message_code = field.get();
            let defs = get_body_from_code(psuedo_message_code);
            body.push(field);

            // if this is a test message, then the pseudo message code data needs to be ignored
            // in order to achieve this, the bit cursor needs to be shifted
            // shift the bit the bit cursor instructs the program where the data extraction should begin
            bit_cursor += test_bit_length;

            defs
        }
        _ => get_body_from_code_char(&code),
    };

    body.append(buffer.decode(definitions, bit_cursor).1.as_mut());

    //bit_cursor += header.bit_length();
    //next_field = body.fields.len();

    BasicMessage::new(code, header, body)
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
