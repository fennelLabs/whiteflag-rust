#[cfg(test)]
mod test;

mod deserialize;
mod serialize;

pub use deserialize::WhiteflagFieldValues;

use crate::{error::WhiteflagError, wf_core::basic_message::BasicMessage};

impl BasicMessage {
    pub fn deserialize_from_json<T: AsRef<str>>(json: T) -> Result<Self, WhiteflagError> {
        let message: WhiteflagFieldValues =
            serde_json::from_str(json.as_ref()).map_err(|e| WhiteflagError::Serde(e))?;
        Ok(BasicMessage::compile(message.fields.as_ref()))
    }
}
