#[macro_use]
extern crate lazy_static;

pub mod binary;
pub mod encoding;
mod error;
pub mod hexadecimal;
pub mod latlong;

#[cfg(test)]
mod validation_tests;
