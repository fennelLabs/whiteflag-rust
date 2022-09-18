use super::{
    crypted_buffer::{CryptMode, CryptedBuffer},
    segment::MessageSegment,
    wf_message_builder::{
        builder_from_encoded, builder_from_field_values, builder_from_serialized,
    },
    FieldValue,
};
use aes_tools::FennelCipher;
use wf_account::whiteflag_account::WhiteflagAccount;
use wf_buffer::WhiteflagBuffer;
use wf_crypto::encryption_method::WhiteflagEncryptionMethod;
use wf_field::{Field, MessageType};
use wf_parser::Parser;

const METAKEY_ORIGINATOR: &str = "originatorAddress";
const METAKEY_RECIPIENT: &str = "recipientAddress";
const FIELD_ENCRYPTIONINDICATOR: &str = "EncryptionIndicator";

pub struct Message {
    message_code: MessageType,
    header: MessageSegment,
    body: MessageSegment,

    originator: Option<WhiteflagAccount>,
    recipient: Option<WhiteflagAccount>,
}

impl MessageSegment {
    pub fn serialize(&self) -> String {
        let mut serial: String = String::new();
        for f in self.iter() {
            let value: &str = &f.get();
            serial.push_str(value);
        }

        serial
    }
}

impl Message {
    pub fn new(
        message_code: MessageType,
        header: Vec<Field>,
        body: Vec<Field>,
        originator: Option<WhiteflagAccount>,
        recipient: Option<WhiteflagAccount>,
    ) -> Message {
        Message {
            message_code,
            header: header.into(),
            body: body.into(),
            originator,
            recipient,
        }
    }

    pub fn serialize(&self) -> String {
        let mut serial = String::new();
        serial.push_str(&self.header.serialize());
        serial.push_str(&self.body.serialize());

        serial
    }

    pub fn from_parser(parsed_message: Parser) -> Self {
        Message::new(
            parsed_message.code,
            parsed_message.header,
            parsed_message.body,
            None,
            None,
        )
    }

    /// construct Message from an array of field strings
    pub fn compile<T: FieldValue>(data: &[T]) -> Self {
        Self::from_parser(builder_from_field_values(data))
    }

    /// construct Message from a serialized string
    pub fn deserialize(message: &str) -> Message {
        Self::from_parser(builder_from_serialized(message))
    }

    /// decode a hexadecimal encoded whiteflag message
    pub fn decode_from_hexadecimal<T: AsRef<str>>(message: T) -> Self {
        let buffer = match WhiteflagBuffer::decode_from_hexadecimal(message) {
            Ok(buffer) => buffer,
            Err(e) => panic!("{}", e),
        };

        Self::from_parser(builder_from_encoded(buffer))
    }

    /// decode a hexadecimal encoded whiteflag message
    pub fn decode_from_buffer(message: WhiteflagBuffer) -> Self {
        Self::from_parser(builder_from_encoded(message))
    }

    pub fn encode_and_crypt<T: FennelCipher>(
        &self,
        cipher: &T,
        mode: CryptMode,
    ) -> WhiteflagBuffer {
        let encryption_indicator_index = 2_usize;
        let encryption_indicator = &self.header[encryption_indicator_index]; // the encryption indicator is the 3rd index in the header

        let method = WhiteflagEncryptionMethod::from_str(encryption_indicator.get()).unwrap();
        let encoded: WhiteflagBuffer = self.encode().into();

        match method {
            WhiteflagEncryptionMethod::NoEncryption => return encoded,
            _ => (),
        };

        let position = self
            .header
            .bit_length_of_field(encryption_indicator_index as isize);

        mode.crypt(cipher, encoded, position)
    }

    /// decode a hexadecimal encoded and encrypted whiteflag message
    pub fn decode_and_crypt<T: FennelCipher>(message: WhiteflagBuffer, cipher: &T) -> Self {
        let buffer = CryptedBuffer::new(message).crypt(cipher, CryptMode::Decrypt);

        Self::decode_from_buffer(buffer)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = WhiteflagBuffer::default();

        buffer.encode(&self.header);
        buffer.encode(&self.body);

        buffer.crop();
        buffer.into()
    }

    pub fn encode_as_hex(&self) -> String {
        hex::encode(self.encode())
    }

    pub fn get_fields(&self) -> Vec<&Field> {
        let mut fields: Vec<&Field> = vec![];
        fields.extend(self.header.iter());
        fields.extend(self.body.iter());
        fields
    }
}

impl<T: FieldValue> From<&[T]> for Message {
    fn from(data: &[T]) -> Self {
        Message::compile(data)
    }
}
