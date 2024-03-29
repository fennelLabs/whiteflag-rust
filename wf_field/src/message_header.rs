use crate::{
    message_body_types::{Authentication, FreeText, MessageBodyType, Resource, Signal},
    Error, Field, MessageCodeType,
};
use serde::{Deserialize, Serialize};

const EMPTY_MESSAGE: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// 4.2.1.1 Generic Message Header Fields
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    prefix: String,
    version: String,
    encryption_indicator: String,
    duress_indicator: String,
    message_code: String,
    /// 4.2.1.7 Reference Indicator Field
    reference_indicator: String,
    referenced_message: String,
}

impl Header {
    pub fn new(code: String) -> Self {
        Self {
            prefix: "WF".to_string(),
            version: "1".to_string(),
            encryption_indicator: "0".to_string(),
            duress_indicator: "0".to_string(),
            message_code: code,
            reference_indicator: "0".to_string(),
            referenced_message: EMPTY_MESSAGE.to_string(),
        }
    }

    pub fn new_discontinue(code: String, reference_indicator: String) -> Self {
        Self {
            prefix: "WF".to_string(),
            version: "1".to_string(),
            encryption_indicator: "0".to_string(),
            duress_indicator: "0".to_string(),
            message_code: code,
            reference_indicator: reference_indicator,
            referenced_message: EMPTY_MESSAGE.to_string(),
        }
    }

    pub fn code(&self) -> Result<MessageCodeType, Error> {
        MessageCodeType::get_message_code(&self.message_code)
    }

    pub fn to_body(self) -> Result<MessageBodyType, Error> {
        match &self.code()? {
            MessageCodeType::Authentication => {
                Ok(MessageBodyType::Authentication(Authentication::new(self)))
            }
            MessageCodeType::Resource => Ok(MessageBodyType::Resource(Resource::new(self))),
            MessageCodeType::FreeText => Ok(MessageBodyType::Text(FreeText::new(self))),
            MessageCodeType::Protective
            | MessageCodeType::Emergency
            | MessageCodeType::Danger
            | MessageCodeType::Status
            | MessageCodeType::Infrastructure
            | MessageCodeType::Mission
            | MessageCodeType::Request => Ok(MessageBodyType::Signal(Signal::new(self))),
            _ => Ok(MessageBodyType::Generic),
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
