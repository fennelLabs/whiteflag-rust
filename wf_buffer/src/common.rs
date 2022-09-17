use crate::WhiteflagBuffer;
use wf_common::common::{append_bits, crop_bits, extract_bits};

impl WhiteflagBuffer {
    pub fn extract_bits(&self, start: usize, end: usize) -> WhiteflagBuffer {
        WhiteflagBuffer::new(
            extract_bits(&self.data, self.bit_length, start, end),
            end - start,
        )
    }

    pub fn extract_bits_from(&self, start: usize) -> WhiteflagBuffer {
        WhiteflagBuffer::new(
            extract_bits(&self.data, self.bit_length, start, self.bit_length),
            self.bit_length - start,
        )
    }

    pub fn append(&mut self, mut buffer: WhiteflagBuffer, bits: Option<usize>) {
        let bit_length_to_extract = bits.unwrap_or(buffer.bit_length);
        let (buffer, length) = append_bits(
            &self.data,
            self.bit_length,
            buffer.to_byte_array(),
            bit_length_to_extract,
        );

        self.data = buffer;
        self.bit_length = length;
    }

    pub fn crop(&mut self) {
        crop_bits(self.data.as_mut(), self.bit_length);
    }

    /// Returns the Whiteflag encoded message as a byte array
    /// @return a byte array with an encoded message
    pub fn to_byte_array(&mut self) -> &[u8] {
        crop_bits(self.data.as_mut(), self.bit_length);
        &self.data
    }
}
