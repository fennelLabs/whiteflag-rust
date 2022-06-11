use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MessageHeader {
    pub prefix: String,
    pub version: String,
    pub encryption_indicator: String,
    pub duress_indicator: String,
    pub message_code: char,
    pub reference_indicator: String,
    pub referenced_message: String,
}

impl From<MessageHeader> for Vec<String> {
    fn from(message: MessageHeader) -> Self {
        vec![
            message.prefix,
            message.version,
            message.encryption_indicator,
            message.duress_indicator,
            message.message_code.to_string(),
            message.reference_indicator,
            message.referenced_message,
        ]
    }
}
