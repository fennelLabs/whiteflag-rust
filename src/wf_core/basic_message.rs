use super::decoder::Decoder;
use super::segment::MessageSegment;
use super::FieldValue;
use crate::wf_account::test_impl::WhiteflagAccount;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_crypto::encryption_method::WhiteflagEncryptionMethod;
use crate::wf_field::Field;
use crate::wf_parser::{from_serialized, WhiteflagMessageBuilder};
use fennel_lib::FennelCipher;

const METAKEY_ORIGINATOR: &str = "originatorAddress";
const METAKEY_RECIPIENT: &str = "recipientAddress";
const FIELD_ENCRYPTIONINDICATOR: &str = "EncryptionIndicator";

pub struct BasicMessage {
    message_code: char,
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

impl BasicMessage {
    pub fn compile<T: FieldValue>(data: &[T]) -> Self {
        WhiteflagMessageBuilder::new(data).compile()
    }

    pub fn new(
        message_code: char,
        header: Vec<Field>,
        body: Vec<Field>,
        originator: Option<WhiteflagAccount>,
        recipient: Option<WhiteflagAccount>,
    ) -> BasicMessage {
        BasicMessage {
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

    pub fn deserialize(message: &str) -> BasicMessage {
        let header = crate::wf_parser::MessageHeader::from_serialized(message);
        let mut body = from_serialized(message, &header.get_body_field_definitions());

        let mut field_values = header.to_vec();
        field_values.append(body.as_mut());

        Self::compile(field_values.as_ref())
    }

    pub fn encode_and_encrypt<T: FennelCipher>(&self, cipher: T) -> Vec<u8> {
        let encryption_indicator_index = 2_usize;
        let encryption_indicator = &self.header[encryption_indicator_index]; // the encryption indicator is the 3rd index in the header

        let method = WhiteflagEncryptionMethod::from_str(&encryption_indicator.get()).unwrap();
        let encoded = self.encode();

        match method {
            WhiteflagEncryptionMethod::NoEncryption => return encoded,
            _ => (),
        };

        let buffer_encoded: WhiteflagBuffer = encoded.into();

        let position = self
            .header
            .bit_length_of_field(encryption_indicator_index as isize);

        buffer_encoded.encrypt(cipher, position).into()
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

    /// decode a hexadecimal encoded whiteflag message
    pub fn decode<T: AsRef<str>>(message: T) -> Self {
        Decoder::new(message).decode()
    }

    pub fn get_fields(&self) -> Vec<&Field> {
        let mut fields: Vec<&Field> = vec![];
        fields.extend(self.header.iter());
        fields.extend(self.body.iter());
        fields
    }
}

impl<T: FieldValue> From<&[T]> for BasicMessage {
    fn from(data: &[T]) -> Self {
        BasicMessage::compile(data)
    }
}
