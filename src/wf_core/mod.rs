#[cfg(test)]
mod message_tests;

#[cfg(test)]
mod message_feature_parity_tests;

#[cfg(test)]
mod edge_case_test;

mod crypted_buffer;
pub mod error;
pub mod message;
mod segment;
mod wf_message_builder;

use crate::error::WhiteflagError;
use message::Message;
use wf_field::FieldValue;

/// encode an array of values, ordered according to the WF specification, into a hexadecimal string
pub fn encode<T: FieldValue>(fields: &[T]) -> Result<String, WhiteflagError> {
    let basic_message: Message = Message::compile(fields)?;
    let message_encoded = basic_message.encode();
    Ok(hex::encode(message_encoded))
}

/// decode a hexadecimal encoded whiteflag message
pub fn decode<T: AsRef<str>>(message: T) -> Result<Message, WhiteflagError> {
    match Message::decode_from_hexadecimal(message) {
        Ok(message) => Ok(message),
        Err(error) => Err(error),
    }
}
