use super::{MessageHeader, WhiteflagEncodeCompatible, WhiteflagMessage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthenticationMessage {
    #[serde(flatten)]
    pub header: MessageHeader,
    pub verification_method: String,
    pub verification_data: String,
}

impl From<WhiteflagMessage> for AuthenticationMessage {
    fn from(message: WhiteflagMessage) -> Self {
        AuthenticationMessage {
            header: message.header,
            verification_method: message
                .body
                .get("verificationMethod")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
            verification_data: message
                .body
                .get("verificationData")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        }
    }
}

impl WhiteflagEncodeCompatible for AuthenticationMessage {
    fn to_field_values(self) -> Vec<String> {
        [
            self.header.to_field_values(),
            vec![self.verification_method, self.verification_data],
        ]
        .concat()
    }
}
