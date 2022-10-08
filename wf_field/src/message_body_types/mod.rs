use crate::error::Error;

mod authentication;
mod crypto;
mod resource;
mod signal;

pub use {authentication::Authentication, crypto::Crypto, resource::Resource, signal::Signal};

/// 4.3 Message Body
pub enum MessageBodyType {
    GENERIC,
    AUTHENTICATION(Authentication),
    CRYPTO(Crypto),
    TEXT,
    // TEST,
    RESOURCE(Resource),
    SIGNAL(Signal),
    //REQUEST,
}

impl MessageBodyType {
    pub fn to_string(&self) -> Result<String, Error> {
        Ok(match &self {
            MessageBodyType::GENERIC => "".to_string(),
            MessageBodyType::AUTHENTICATION(a) => serde_json::to_string(a)?,
            MessageBodyType::CRYPTO(c) => serde_json::to_string(c)?,
            MessageBodyType::RESOURCE(r) => serde_json::to_string(r)?,
            MessageBodyType::SIGNAL(s) => serde_json::to_string(s)?,
            _ => todo!(),
        })
    }
}

/*
    2.4.2.2 Management Messages
    Authentication, Crypto and Test are considered "management messages"

    2.4.2.1 Functional Messages
    Everything else are "functional" messages
*/
