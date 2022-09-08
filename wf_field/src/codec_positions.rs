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

    pub const fn next(&self, config: ByteConfiguration) -> Self {
        Self::new(config, self.bit_end)
    }

    /* pub const fn next(&mut self, start:usize, end:usize) {
        self.bit_start = Some(start);
        self.bit_end = Some(end);
    } */
}
