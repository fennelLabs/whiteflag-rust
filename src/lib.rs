use error::WhiteflagError;

#[cfg(test)]
mod tests;

mod error;
mod wf_account;
mod wf_auth;
mod wf_buffer;
mod wf_codec;
mod wf_convert;
#[allow(dead_code)]
mod wf_core;
mod wf_crypto;
mod wf_field;
mod wf_json;
mod wf_parser;

pub fn encode_from_json<T: AsRef<str>>(json: T) -> Result<String, WhiteflagError> {
    let message: wf_json::WhiteflagFieldValues =
        serde_json::from_str(json.as_ref()).map_err(|e| WhiteflagError::Serde(e))?;

    Ok(wf_core::creator::encode(&message.fields))
}

pub fn decode_from_hex<T: AsRef<str>>(hex: T) -> Result<String, WhiteflagError> {
    let message = wf_core::creator::decode(hex);
    let json = serde_json::to_string(&message).map_err(|e| WhiteflagError::Serde(e))?;

    Ok(json)
}
