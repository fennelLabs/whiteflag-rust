use crate::error::Error;

mod authentication;
mod resource;
mod signal;

pub use {authentication::Authentication, resource::Resource, signal::Signal};

/// 4.3 Message Body
pub enum MessageBodyType {
    GENERIC,
    AUTHENTICATION(Authentication),
    RESOURCE(Resource),
    SIGNAL(Signal),
    /* CRYPTO,
    TEXT,
    RESOURCE,
    TEST,

    REQUEST, */
}

impl MessageBodyType {
    pub fn to_string(&self) -> Result<String, Error> {
        Ok(match &self {
            MessageBodyType::GENERIC => "".to_string(),
            MessageBodyType::AUTHENTICATION(a) => serde_json::to_string(a)?,
            MessageBodyType::RESOURCE(r) => serde_json::to_string(r)?,
            MessageBodyType::SIGNAL(s) => serde_json::to_string(s)?,
        })
    }
}
