use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

/* #[derive(Serialize, Deserialize)]
pub struct HeaderSegment {
    prefix: String,
    version: String,
    encryption_indicator: String,
    duress_indicator: String,
    message_code: String,
    reference_indicator: String,
    referenced_message: String,
} */

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthenticationMessage {
    pub prefix: String,
    pub version: String,
    pub encryption_indicator: String,
    pub duress_indicator: String,
    pub message_code: String,
    pub reference_indicator: String,
    pub referenced_message: String,

    pub verification_method: String,
    pub verification_data: String,
}

impl AuthenticationMessage {
    pub fn to_field_values(self) -> Vec<String> {
        vec![
            self.prefix,
            self.version,
            self.encryption_indicator,
            self.duress_indicator,
            self.message_code,
            self.reference_indicator,
            self.referenced_message,
            self.verification_method,
            self.verification_data,
        ]
    }
}
