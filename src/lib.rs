#[cfg(test)]
mod tests;

mod wf_buffer;
mod wf_codec;
mod wf_core;
mod wf_models;

pub fn encode_from_json<T: AsRef<str>>(json: T) -> Result<String, String> {
    let message: wf_models::WhiteflagMessage =
        serde_json::from_str(json.as_ref()).expect("deserialization error");
    let values: Vec<String> = message.try_into()?;
    Ok(wf_core::creator::encode(&values))
}

pub fn decode_from_hex<T: AsRef<str>>(hex: T) -> Result<String, String> {
    let message = wf_core::creator::decode(hex);
    let json = serde_json::to_string(&message).expect("serialization issue");
    Ok(json)
}
