use crate::{error::Error, Header, MessageCodeType};
use serde::{Deserialize, Serialize};

pub enum MessageBodyType {
    GENERIC,
    AUTHENTICATION(Authentication),
    /* CRYPTO,
    TEXT,
    RESOURCE,
    TEST,
    SIGNAL,
    REQUEST, */
}

impl MessageBodyType {
    pub fn to_string(&self) -> Result<String, Error> {
        Ok(match &self {
            MessageBodyType::GENERIC => "".to_string(),
            MessageBodyType::AUTHENTICATION(a) => serde_json::to_string(a)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Authentication {
    #[serde(flatten)]
    header: Header,
    /// e.g. 1
    verification_method: usize,
    /// e.g. https://organisation.int/whiteflag
    verification_data: String,
}

impl Authentication {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            verification_method: 1,
            verification_data: "https://organisation.int/whiteflag".to_string(),
        }
    }
}

impl Default for Authentication {
    fn default() -> Self {
        Self::new(Header::new(MessageCodeType::Authentication.to_string()))
    }
}
