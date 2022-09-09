use crate::byte_configuration::ByteConfiguration;

#[derive(Clone, Debug)]
pub struct CodecPositions {
    pub bytes: ByteConfiguration,
    /// some encodings will have a fixed byte length
    is_fixed: bool,
    /// (length * encoding.bit_length) unless is_fixed is true, then it is equal to encoding.bit_length
    bit_length: usize,
    pub bit_start: usize,
    pub bit_end: usize,
}

impl CodecPositions {
    pub const fn start(config: ByteConfiguration) -> Self {
        Self::new(config, 0)
    }

    pub const fn new(config: ByteConfiguration, bit_start: usize) -> Self {
        let is_fixed = config.is_fixed();
        let bit_length = config.bit_length();
        let bit_end = bit_start + bit_length;

        Self {
            bytes: config,
            is_fixed,
            bit_length,
            bit_start,
            bit_end,
        }
    }

    /// used in the deserializing process
    pub fn read_from_serialized<'a>(&self, message: &'a str) -> &'a str {
        match self.bytes.end {
            Some(e) => &message[self.bytes.start..e],
            None => &message[self.bytes.start..],
        }
    }
}
