use super::basic_message::BasicMessage;
use super::segment::MessageSegment;
use super::types::MessageType;
use super::wf_codec::common::{crop_bits, decode_from_hexadecimal, to_hex};

pub const PREFIX: &str = "WF";
pub const PROTOCOL_VERSION: &str = "1";
pub const FIELD_PREFIX: &str = "Prefix";
pub const FIELD_VERSION: &str = "Version";
pub const FIELD_MESSAGETYPE: &str = "MessageCode";
pub const FIELD_TESTMESSAGETYPE: &str = "PseudoMessageCode";

pub fn compile<T: AsRef<str> + Into<String>>(data: &[T]) -> BasicMessage {
    let mut header: MessageSegment = MessageSegment::generic_header_segment();
    header.set_all(data.as_ref(), 0);

    let message_type = get_message_type(&header);
    let body_start_index = header.get_number_of_fields();
    let mut body = message_type.body.clone();

    //need switch statement here

    body.set_all(data.as_ref(), body_start_index);
    BasicMessage::new(message_type, header, body)
}

pub fn encode<T: AsRef<str> + Into<String>>(fields: &[T]) -> String {
    let basic_message = compile(fields);
    let (message_encoded, len) = basic_message.encode();

    to_hex(&crop_bits(message_encoded, len as isize))
}

/**
 * Decodes an encoded Whiteflag message and creates a new Whiteflag base message object
 * @since 1.1
 * @param msgBuffer a buffer with the compressed binary encoded message
 * @return this message creator
 * @throws WfCoreException if the encoded message is invalid
 */
pub fn decode<T: AsRef<str>>(message: T) -> BasicMessage {
    let (buffer, bit_length) = decode_from_hexadecimal(message);
    //let buffer = from_hex(message);
    //let bit_length = buffer.len() * BYTE;

    let mut bit_cursor = 0;
    //let mut next_field = 0;

    let mut header: MessageSegment = MessageSegment::generic_header_segment();
    bit_cursor += header.decode(&buffer, bit_length, bit_cursor, 0); // header.bit_length();

    let message_type = get_message_type(&header);

    let mut body = message_type.body.clone();
    body.decode(&buffer, bit_length, bit_cursor, 0);
    //bit_cursor += header.bit_length();
    //next_field = body.fields.len();

    //body.decode(&buffer, bit_length, bit_cursor, next_field);

    BasicMessage::new(message_type, header, body)
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

fn get_message_type(header: &MessageSegment) -> MessageType {
    let message_code = match header.get(&FIELD_MESSAGETYPE) {
        Some(x) => x.chars().next(),
        _ => None,
    };

    MessageType::from_code_option(message_code.as_ref())
}
