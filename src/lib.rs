use error::WhiteflagError;

mod error;
#[allow(dead_code)]
mod wf_core;
mod wf_json;

pub fn encode_from_json<T: AsRef<str>>(json: T) -> Result<String, WhiteflagError> {
    let message: wf_json::WhiteflagFieldValues =
        serde_json::from_str(json.as_ref()).map_err(WhiteflagError::Serde)?;

    Ok(wf_core::encode(&message.fields))
}

pub fn decode_from_hex<T: AsRef<str>>(hex: T) -> Result<String, WhiteflagError> {
    let message = wf_core::decode(hex);
    let json = serde_json::to_string(&message).map_err(WhiteflagError::Serde)?;

    Ok(json)
}
