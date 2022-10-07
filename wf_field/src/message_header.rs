use crate::{
    message_body_type::{Authentication, MessageBodyType},
    Field, MessageCodeType,
};
use serde::{Deserialize, Serialize};

const EMPTY_MESSAGE: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Serialize, Deserialize)]
pub struct Header {
    prefix: String,
    version: String,
    encryption_indicator: usize,
    duress_indicator: usize,
    message_code: String,
    reference_indicator: usize,
    referenced_message: String,
}

impl Header {
    pub fn new(code: String) -> Self {
        Self {
            prefix: "WF".to_string(),
            version: "1".to_string(),
            encryption_indicator: 0,
            duress_indicator: 0,
            message_code: code,
            reference_indicator: 0,
            referenced_message: EMPTY_MESSAGE.to_string(),
        }
    }

    pub fn code(&self) -> MessageCodeType {
        MessageCodeType::get_message_code(&self.message_code)
    }

    pub fn to_body(self) -> MessageBodyType {
        match &self.code() {
            MessageCodeType::Authentication => {
                MessageBodyType::AUTHENTICATION(Authentication::new(self))
            }
            _ => MessageBodyType::GENERIC,
        }
    }
}

pub trait MessageHeader {
    type Target: ?Sized;

    fn prefix(&self) -> &Self::Target;
    fn version(&self) -> &Self::Target;
    fn encryption_indicator(&self) -> &Self::Target;
    fn duress_indicator(&self) -> &Self::Target;
    fn message_code(&self) -> &Self::Target;
    fn reference_indicator(&self) -> &Self::Target;
    fn referenced_message(&self) -> &Self::Target;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MessageHeaderOrder {
    Prefix = 0,
    Version = 1,
    EncryptionIndicator = 2,
    DuressIndicator = 3,
    MessageCode = 4,
    ReferenceIndicator = 5,
    ReferencedMessage = 6,
}

impl<'a> MessageHeaderOrder {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn get<'b>(&'a self, fields: &'b [Field]) -> &'b Field {
        &fields[self.as_usize()]
    }
}
