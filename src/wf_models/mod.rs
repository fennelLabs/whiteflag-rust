use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[cfg(test)]
mod test;

mod authentication;
mod header;

pub use authentication::AuthenticationMessage;
pub use header::MessageHeader;

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct WhiteflagMessage {
    #[serde(flatten)]
    pub header: MessageHeader,
    #[serde(flatten)]
    pub body: HashMap<String, Value>,
}

pub trait WhiteflagEncodeCompatible {
    fn to_field_values(self) -> Vec<String>;
}

impl WhiteflagMessage {
    pub fn from_json<T: AsRef<str>>(json: T) -> WhiteflagMessage {
        let wf_message: WhiteflagMessage = serde_json::from_str(json.as_ref()).unwrap();
        wf_message
    }
}

impl WhiteflagEncodeCompatible for WhiteflagMessage {
    fn to_field_values(self) -> Vec<String> {
        match &self.header.message_code {
            'A' => {
                let auth: AuthenticationMessage = self.into();
            }
            _ => {}
        };

        vec![]
    }
}
