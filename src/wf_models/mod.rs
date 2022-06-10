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

impl TryFrom<WhiteflagMessage> for Vec<String> {
    type Error = String;

    fn try_from(message: WhiteflagMessage) -> Result<Self, Self::Error> {
        let values: Vec<String> = match &message.header.message_code {
            'A' => {
                let auth: AuthenticationMessage = message.try_into()?;
                auth.into()
            }
            _ => vec![],
        };

        Ok(values)
    }
}
