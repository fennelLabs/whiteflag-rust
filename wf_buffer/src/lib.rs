use wf_field::FieldDefinition;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_field;

mod common;
mod conversions;
mod decode;
mod encode;

pub struct WhiteflagBuffer {
    data: Vec<u8>,
    bit_length: usize,
}

impl WhiteflagBuffer {
    pub fn bit_length(&self) -> usize {
        self.bit_length
    }
}

pub trait BufferReader {
    fn read(&self, buffer: &WhiteflagBuffer) -> String;
}

impl BufferReader for FieldDefinition {
    /// used in the decoding process
    fn read(&self, buffer: &WhiteflagBuffer) -> String {
        buffer.extract_message_value(self, self.positions.bit_start)
    }
}
