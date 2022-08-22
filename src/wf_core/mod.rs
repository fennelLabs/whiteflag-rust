#[cfg(test)]
mod message_tests;

#[cfg(test)]
mod message_feature_parity_tests;

#[cfg(test)]
mod edge_case_test;

pub mod basic_message;
mod decoder;
pub mod error;
mod segment;
mod types;

pub trait FieldValue: AsRef<str> + Into<String> + std::fmt::Debug {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> + std::fmt::Debug {}

use basic_message::BasicMessage;
use decoder::Decoder;

/// encode an array of values, ordered according to the WF specification, into a hexadecimal string
pub fn encode<T: FieldValue>(fields: &[T]) -> String {
    let basic_message: BasicMessage = fields.into();
    let message_encoded = basic_message.encode();
    hex::encode(message_encoded)
}

/// decode a hexadecimal encoded whiteflag message
pub fn decode<T: AsRef<str>>(message: T) -> BasicMessage {
    Decoder::new(message).decode()
}
