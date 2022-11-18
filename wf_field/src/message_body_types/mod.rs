use crate::error::Error;

mod authentication;
mod crypto;
mod freetext;
mod resource;
mod signal;
mod test;

pub use {
    authentication::Authentication, crypto::Crypto, freetext::FreeText, resource::Resource,
    signal::Signal, test::Test,
};

/// 4.3 Message Body
pub enum MessageBodyType {
    Generic,
    Authentication(Authentication),
    Crypto(Crypto),
    Text(FreeText),
    Test(Test),
    Resource(Resource),
    Signal(Signal),
    //REQUEST,
}

impl MessageBodyType {
    pub fn to_string(&self) -> Result<String, Error> {
        Ok(match &self {
            MessageBodyType::Generic => "".to_string(),
            MessageBodyType::Authentication(a) => serde_json::to_string(a)?,
            MessageBodyType::Crypto(c) => serde_json::to_string(c)?,
            MessageBodyType::Resource(r) => serde_json::to_string(r)?,
            MessageBodyType::Signal(s) => serde_json::to_string(s)?,
            MessageBodyType::Text(s) => serde_json::to_string(s)?,
            MessageBodyType::Test(s) => serde_json::to_string(s)?,
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
