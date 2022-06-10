use super::{MessageHeader, WhiteflagMessage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthenticationMessage {
    #[serde(flatten)]
    pub header: MessageHeader,
    pub verification_method: String,
    pub verification_data: String,
}

impl TryFrom<WhiteflagMessage> for AuthenticationMessage {
    type Error = String;

    fn try_from(value: WhiteflagMessage) -> Result<Self, Self::Error> {
        if value.header.message_code != 'A' {
            return Err(format!(
                "not possible to convert message code {} to authentication message",
                &value.header.message_code
            ));
        }

        Ok(AuthenticationMessage {
            header: value.header,
            verification_method: value
                .body
                .get("verificationMethod")
                .expect("missing verificationMethod")
                .as_str()
                .expect("unable to convert to string")
                .to_string(),
            verification_data: value
                .body
                .get("verificationData")
                .expect("missing verificationData")
                .as_str()
                .expect("unable to convert to string")
                .to_string(),
        })
    }
}

impl From<AuthenticationMessage> for Vec<String> {
    fn from(message: AuthenticationMessage) -> Self {
        let auth_values = vec![message.verification_method, message.verification_data];
        let mut values: Vec<String> = message.header.into();
        values.extend(auth_values.into_iter());
        values
    }
}
