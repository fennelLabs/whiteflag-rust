use super::common::{remove_hexadecimal_prefix, remove_hexadecimal_prefix_mut};
use hex::FromHexError;

pub struct HexadecimalString {
    hex: String,
}

impl HexadecimalString {
    pub fn new(buffer: &[u8]) -> HexadecimalString {
        buffer.into()
    }

    pub fn from_string<T: Into<String>>(hex: T) -> HexadecimalString {
        HexadecimalString { hex: hex.into() }
    }

    pub fn decode<T: AsRef<str>>(value: T) -> Result<Vec<u8>, FromHexError> {
        hex::decode(remove_hexadecimal_prefix(value.as_ref()))
    }

    pub fn remove_prefix(&mut self) {
        remove_hexadecimal_prefix_mut(&self.hex);
    }
}

impl Into<String> for HexadecimalString {
    fn into(self) -> String {
        self.hex
    }
}

impl AsRef<str> for HexadecimalString {
    fn as_ref(&self) -> &str {
        &self.hex
    }
}

impl From<&[u8]> for HexadecimalString {
    fn from(buffer: &[u8]) -> Self {
        HexadecimalString {
            hex: hex::encode(buffer),
        }
    }
}

impl TryFrom<HexadecimalString> for Vec<u8> {
    type Error = FromHexError;

    fn try_from(value: HexadecimalString) -> Result<Self, Self::Error> {
        HexadecimalString::decode(value.as_ref())
    }
}