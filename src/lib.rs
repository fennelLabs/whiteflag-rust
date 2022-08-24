#[macro_use]
extern crate lazy_static;

use error::WhiteflagError;

mod error;
mod wf_account;
mod wf_auth;
mod wf_buffer;
mod wf_codec;
#[allow(dead_code)]
mod wf_core;
mod wf_crypto;
mod wf_field;
mod wf_json;
mod wf_parser;
mod wf_validation;

pub fn encode_from_json<T: AsRef<str>>(json: T) -> Result<String, WhiteflagError> {
    let message: wf_json::WhiteflagFieldValues =
        serde_json::from_str(json.as_ref()).map_err(|e| WhiteflagError::Serde(e))?;

    Ok(wf_core::encode(&message.fields))
}

pub fn decode_from_hex<T: AsRef<str>>(hex: T) -> Result<String, WhiteflagError> {
    let message = wf_core::decode(hex);
    let json = serde_json::to_string(&message).map_err(|e| WhiteflagError::Serde(e))?;

    Ok(json)
}
