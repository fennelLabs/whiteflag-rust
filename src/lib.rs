pub use error::WhiteflagError;
use wf_field::{Header, MessageBodyType};

mod error;
#[allow(dead_code)]
mod wf_core;
mod wf_json;

/// encode whiteflag json message into a hexadecimal string
///
/// # Example
/// ```
/// let json = serde_json::json!({
///    "prefix": "WF",
///    "version": "1",
///    "encryptionIndicator": "0",
///    "duressIndicator": "0",
///    "messageCode": 'A',
///    "referenceIndicator": "0",
///    "referencedMessage": "0000000000000000000000000000000000000000000000000000000000000000",
///    "verificationMethod": "1",
///    "verificationData": "https://organisation.int/whiteflag"
/// }).to_string();
/// let wf_message = fennel_whiteflag::encode_from_json(json).unwrap();
/// let hex = "5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380";
///
/// assert_eq!(hex, wf_message);
/// ```
pub fn encode_from_json<T: AsRef<str>>(json: T) -> Result<String, WhiteflagError> {
    let message: wf_json::WhiteflagFieldValues =
        serde_json::from_str(json.as_ref()).map_err(WhiteflagError::Serde)?;

    Ok(wf_core::encode(&message.fields)?)
}

/// decode hexadecimal encoded whiteflag message into a json message
///
/// # Example
/// ```
/// let hex = "5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380";
/// let wf_message = fennel_whiteflag::decode_from_hex(hex).unwrap();
///
/// let json = serde_json::json!({
///    "prefix": "WF",
///    "version": "1",
///    "encryptionIndicator": "0",
///    "duressIndicator": "0",
///    "messageCode": 'A',
///    "referenceIndicator": "0",
///    "referencedMessage": "0000000000000000000000000000000000000000000000000000000000000000",
///    "verificationMethod": "1",
///    "verificationData": "https://organisation.int/whiteflag"
/// });
///
/// assert_eq!(json.as_object().unwrap(), &serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&wf_message).unwrap());
/// ```
pub fn decode_from_hex<T: AsRef<str>>(hex: T) -> Result<String, WhiteflagError> {
    let message = wf_core::decode(hex)?;
    let json = serde_json::to_string(&message).map_err(WhiteflagError::Serde)?;

    Ok(json)
}

pub struct WhiteflagMessage {
    body: MessageBodyType,
    json: String,
}

impl WhiteflagMessage {
    pub fn new(code: String) -> Result<Self, WhiteflagError> {
        let header = Header::new(code);
        let body = header.to_body()?;
        Ok(Self {
            json: body.to_string()?,
            body,
        })
    }

    pub fn new_with_reference(
        code: String,
        reference_indicator: String,
    ) -> Result<Self, WhiteflagError> {
        let header = Header::new_discontinue(code, reference_indicator);
        let body = header.to_body()?;
        Ok(Self {
            json: body.to_string()?,
            body,
        })
    }

    pub fn as_json(self) -> String {
        self.json
    }

    pub fn as_hex(&self) -> Result<String, WhiteflagError> {
        encode_from_json(&self.json)
    }
}
