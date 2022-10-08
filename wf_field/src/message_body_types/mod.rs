use crate::error::Error;

mod authentication;
mod resource;

pub use {authentication::Authentication, resource::Resource};

/// 4.3 Message Body
pub enum MessageBodyType {
    GENERIC,
    AUTHENTICATION(Authentication),
    RESOURCE(Resource),
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
            MessageBodyType::RESOURCE(r) => serde_json::to_string(r)?,
        })
    }
}
