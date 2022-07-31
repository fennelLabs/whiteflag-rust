#[cfg(test)]
mod test;

mod deserialize;
mod serialize;

pub use deserialize::WhiteflagFieldValues;

struct WhiteflagJsonMessage {}

impl WhiteflagJsonMessage {}