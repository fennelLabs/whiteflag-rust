use super::decoder::Decoder;
use super::error::{WhiteflagError, WhiteflagResult};
use super::segment::MessageSegment;
use super::FieldValue;
use crate::wf_account::account::WfAccount;
use crate::wf_account::test_impl::WhiteflagAccount;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_crypto::cipher::{WfCipher, WhiteflagCipher};
use crate::wf_crypto::encryption_method::WhiteflagEncryptionMethod;
use crate::wf_crypto::wf_encryption_key::WhiteflagEncryptionKey;
use crate::wf_field::{get_field_value_from_array, Field};
use crate::wf_parser::{from_serialized, MessageHeader, WhiteflagMessageBuilder};

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

    /* pub fn encrypt(&self, encoded_message: WhiteflagBuffer) -> WhiteflagBuffer {
        let method =
            BasicMessage::get_encryption_method(self.header.get(FIELD_ENCRYPTIONINDICATOR));
        if method
            == (WhiteflagEncryptionMethod::NoEncryption {
                field_value: "0".to_string(),
                algorithm_name: "NONE".to_string(),
                operation_mode: "NONE".to_string(),
                padding_scheme: "NoPadding".to_string(),
                key_length: 0,
                hkdf_salt: "".to_string(),
            })
        {
            return encoded_message;
        }
        let cipher = self.create_cipher(method, self.originator.unwrap(), self.recipient.unwrap());
        let unencrypted_bit_position = self.header.bit_length_of_field(FIELD_ENCRYPTIONINDICATOR);
        let encrypted_message = WhiteflagBuffer::new(vec![], 0);

        encrypted_message.append(
            encoded_message.extract_bits(0, unencrypted_bit_position),
            None,
        );
        encrypted_message.append(
            encoded_message.extract_bits_from(unencrypted_bit_position),
            None,
        );
        encrypted_message
    }

    pub fn decrypt(&self, message: WhiteflagBuffer) -> BasicMessage {
        let method =
            BasicMessage::get_encryption_method(self.header.get(FIELD_ENCRYPTIONINDICATOR));
        if method
            == (WhiteflagEncryptionMethod::NoEncryption {
                field_value: "0".to_string(),
                algorithm_name: "NONE".to_string(),
                operation_mode: "NONE".to_string(),
                padding_scheme: "NoPadding".to_string(),
                key_length: 0,
                hkdf_salt: "".to_string(),
            })
        {
            return message;
        }

        let cipher = self.create_cipher(method, self.originator.unwrap(), self.recipient.unwrap());
        let unencrypted_bit_position = self.header.bit_length_of_field(FIELD_ENCRYPTIONINDICATOR);
        let encoded_message = WhiteflagBuffer::new(vec![], 0);

        encoded_message.append(message.extract_bits(0, unencrypted_bit_position), None);
        encoded_message.append(message.extract_bits_from(unencrypted_bit_position), None);
        encoded_message
    }

    pub fn get_encryption_method(indicator: String) -> WhiteflagEncryptionMethod {
        encryption_method_from_field_value(indicator).unwrap()
    }

    pub fn create_cipher(
        &self,
        method: WhiteflagEncryptionMethod,
        originator: WhiteflagAccount,
        recipient: WhiteflagAccount,
    ) -> WhiteflagCipher {
        let key = self.get_encryption_key(method, originator, recipient);
        let cipher = WhiteflagCipher::from_key(key.unwrap());
        let address = originator.get_binary_address();
        cipher.set_context(hex::encode(address));
        cipher
    }

    pub fn get_encryption_key(
        &self,
        method: WhiteflagEncryptionMethod,
        originator: WhiteflagAccount,
        recipient: WhiteflagAccount,
    ) -> WhiteflagResult<WhiteflagEncryptionKey> {
        match method {
            WhiteflagEncryptionMethod::Aes512IegEcdh {
                field_value,
                algorithm_name,
                operation_mode,
                padding_scheme,
                key_length,
                hkdf_salt,
            } => Ok(self.generated_negotiated_key(originator, recipient)),
            WhiteflagEncryptionMethod::Aes512IegPsk {
                field_value,
                algorithm_name,
                operation_mode,
                padding_scheme,
                key_length,
                hkdf_salt,
            } => Ok(self.get_shared_key(recipient)),
            _ => Err(WhiteflagError::CannotRetrieveKey),
        }
    }

    pub fn get_shared_key(recipient: WhiteflagAccount) -> WhiteflagEncryptionKey {
        recipient.get_shared_key().unwrap()
    }

    pub fn generate_negotiated_key(
        originator: WhiteflagAccount,
        recipient: WhiteflagAccount,
    ) -> WhiteflagEncryptionKey {
    } */

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
