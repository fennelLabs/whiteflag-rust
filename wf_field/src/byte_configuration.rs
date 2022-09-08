use crate::codec_positions::CodecPositions;
use wf_codec::encoding::{ByteLength, Encoding};

#[derive(Clone, Copy, Debug)]
pub struct ByteConfiguration {
    pub start: usize,
    /// most fields will have an end byte, but some are unbounded
    pub end: Option<usize>,
    encoding: Encoding,
    /// (end - start) unless the encoding has a fixed byte length or it is the last field and isn't bounded (end = 0 or None)
    pub length: ByteLength,
}

impl ByteConfiguration {
    pub const fn new(start: usize, end: usize, encoding: Encoding) -> Self {
        let (end, length) = if end == 0 {
            (None, ByteLength::new(encoding.byte_length.length()))
        } else {
            (Some(end), ByteLength::new(end - start))
        };

        Self {
            start,
            end,
            encoding,
            length,
        }
    }

    pub const fn is_fixed(&self) -> bool {
        self.encoding.byte_length.is_fixed()
    }

    pub const fn bit_length(&self) -> usize {
        if self.is_fixed() {
            self.encoding.bit_length
        } else {
            self.length.as_usize() * self.encoding.bit_length
        }
    }

    pub const fn to_position(self, bit_start: usize) -> CodecPositions {
        CodecPositions::new(self, bit_start)
    }
}
