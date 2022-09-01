use std::ops::{Deref, DerefMut};
use wf_buffer::WhiteflagBuffer;
use wf_field::Field;

#[derive(Clone)]
pub struct MessageSegment {
    fields: Vec<Field>,
}

impl MessageSegment {
    /// encodes the array of fields contained in the message segment
    /// returns a binary buffer with the binary encoded message segment and its bit length
    pub fn encode(&self) -> (Vec<u8>, usize) {
        let mut buffer: WhiteflagBuffer = Default::default();

        //let cursor = self.fields[0].start_byte;
        for field in &self.fields {
            /* if (field.startByte != byteCursor) {
                throw new WfCoreException("Invalid field order while encoding: did not expect field " + field.debugInfo() + " at byte " + byteCursor, null);
            } */

            buffer.append_field(field);

            //byteCursor = field.endByte;
        }

        buffer.into()
    }

    /// Returns the bit length of this segment, excluding the last variable length field if not set
    /// returns the bit length of this segment
    pub fn bit_length(&self) -> usize {
        self.bit_length_of_field(self.len() as isize)
    }

    /// Returns the bit length up to and including the specified field, excluding the last variable length field if not set
    /// field_index the index of the field up to which the segment length is calculated; negative index counts back from last field
    /// returns the bit length of this segment up to and including the specified field, or 0 if the field does not exist
    pub fn bit_length_of_field(&self, field_index: isize) -> usize {
        /* converts potential negative index into a positive */
        let selected_field_index = match self.get_absolute_index(field_index) {
            Ok(index) => index,
            Err(_) => return 0,
        };

        /* Calculate segment bit length */
        let mut bit_length = 0;
        for index in 0..=selected_field_index {
            bit_length += self[index].bit_length();
        }

        bit_length
    }

    /// Gets the absolute field index and check if index is within bounds
    /// index the absolute index of the requested field; negative index counts back from last field
    /// returns an error if the index is out of bounds
    fn get_absolute_index(&self, index: isize) -> Result<usize, &str> {
        let length = self.len() as isize;

        if index >= 0 && index < length {
            return Ok(index as usize);
        }

        if index < 0 && (length + index) >= 0 {
            return Ok((length + index) as usize);
        }

        Err("index is out of bounds")
    }
}

impl Deref for MessageSegment {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl DerefMut for MessageSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}

impl From<Vec<Field>> for MessageSegment {
    fn from(fields: Vec<Field>) -> Self {
        MessageSegment { fields }
    }
}
