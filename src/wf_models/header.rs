use super::WhiteflagEncodeCompatible;
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

impl WhiteflagEncodeCompatible for MessageHeader {
    fn to_field_values(self) -> Vec<String> {
        vec![
            self.prefix,
            self.version,
            self.encryption_indicator,
            self.duress_indicator,
            self.message_code.to_string(),
            self.reference_indicator,
            self.referenced_message,
        ]
    }
}
