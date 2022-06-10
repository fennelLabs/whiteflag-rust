use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[cfg(test)]
mod test;

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct WhiteflagMessage {
    #[serde(flatten)]
    pub header: MessageHeader,
    #[serde(flatten)]
    pub body: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthenticationMessage {
    #[serde(flatten)]
    pub header: MessageHeader,
    pub verification_method: String,
    pub verification_data: String,
}

impl MessageHeader {
    pub fn to_field_values(self) -> Vec<String> {
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

impl AuthenticationMessage {
    pub fn to_field_values(self) -> Vec<String> {
        [
            self.header.to_field_values(),
            vec![self.verification_method, self.verification_data],
        ]
        .concat()
    }
}

impl WhiteflagMessage {
    pub fn to_wf_message(&self) {
        match &self.header.message_code {
            'A' => {}
            _ => {}
        }
    }
}
