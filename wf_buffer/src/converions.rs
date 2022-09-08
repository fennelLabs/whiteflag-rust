use crate::WhiteflagBuffer;
use wf_field::Field;

impl From<(Vec<u8>, usize)> for WhiteflagBuffer {
    fn from((buffer, bit_length): (Vec<u8>, usize)) -> Self {
        WhiteflagBuffer::new(buffer, bit_length)
    }
}

impl From<WhiteflagBuffer> for (Vec<u8>, usize) {
    fn from(buffer: WhiteflagBuffer) -> Self {
        (buffer.data, buffer.bit_length)
    }
}

impl AsRef<Vec<u8>> for WhiteflagBuffer {
    fn as_ref(&self) -> &Vec<u8> {
        &self.data
    }
}

impl AsRef<[u8]> for WhiteflagBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl From<Vec<u8>> for WhiteflagBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        WhiteflagBuffer::from(buffer)
    }
}

impl Default for WhiteflagBuffer {
    fn default() -> Self {
        Self {
            data: Default::default(),
            bit_length: Default::default(),
        }
    }
}

impl From<WhiteflagBuffer> for Vec<u8> {
    fn from(buffer: WhiteflagBuffer) -> Self {
        buffer.data
    }
}

impl From<&Field> for WhiteflagBuffer {
    fn from(field: &Field) -> Self {
        let length = field.bit_length();
        WhiteflagBuffer::new(field.into(), length)
    }
}
