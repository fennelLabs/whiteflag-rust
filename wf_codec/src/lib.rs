#[macro_use]
extern crate lazy_static;

pub mod binary;
pub mod encoding;
mod error;
pub mod hexadecimal;
pub mod latlong;

pub use error::{CodecError, CodecResult};

#[cfg(test)]
mod validation_tests;
