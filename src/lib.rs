#[cfg(test)]
mod tests;

mod wf_buffer;
mod wf_codec;
mod wf_core;
mod wf_json;

pub fn encode_from_json<T: AsRef<str>>(json: T) -> Result<String, String> {
    let message: wf_json::WhiteflagFieldValues =
        serde_json::from_str(json.as_ref()).expect("deserialization error");
    Ok(wf_core::creator::encode(&message.fields))
}

pub fn decode_from_hex<T: AsRef<str>>(hex: T) -> Result<String, String> {
    let message = wf_core::creator::decode(hex);
    let json = serde_json::to_string(&message).expect("serialization issue");
    Ok(json)
}
