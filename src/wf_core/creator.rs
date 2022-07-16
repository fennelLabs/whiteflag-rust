use super::basic_message::BasicMessage;
use super::segment::MessageSegment;
use super::wf_buffer::common::{decode_from_hexadecimal, to_hex};
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_convert::FieldValue;
use crate::wf_field::{generic_header_fields, get_message_body};

pub const PREFIX: &str = "WF";
pub const PROTOCOL_VERSION: &str = "1";

pub fn encode<T: FieldValue>(fields: &[T]) -> String {
    let basic_message: BasicMessage = fields.into();
    let message_encoded = basic_message.encode();
    to_hex(&message_encoded)
}

/**
 * Decodes an encoded Whiteflag message and creates a new Whiteflag base message object
 * @since 1.1
 * @param msgBuffer a buffer with the compressed binary encoded message
 * @return this message creator
 * @throws WfCoreException if the encoded message is invalid
 */
pub fn decode<T: AsRef<str>>(message: T) -> BasicMessage {
    let buffer: WhiteflagBuffer = decode_from_hexadecimal(message).into();
    //let mut next_field = 0;

    let (bit_cursor, header) = buffer.decode(generic_header_fields().to_vec(), 0);

    let (body_field_defs, code) = get_message_body(&header);
    let (_, body) = buffer.decode(body_field_defs, bit_cursor);
    //bit_cursor += header.bit_length();
    //next_field = body.fields.len();

    BasicMessage::new(
        code,
        MessageSegment::from(header),
        MessageSegment::from(body),
    )
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
