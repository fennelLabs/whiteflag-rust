use crate::{wf_buffer::WhiteflagBuffer, wf_field::Field};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct MessageSegment {
    fields: Vec<Field>,
}

impl MessageSegment {
    /**
     * Encodes this message segment
     * @return a binary buffer with the binary encoded message segment and its bit length
     * @throws WfCoreException if the message cannot be encoded
     */
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

    /**
     * Returns the bit length of this segment, excluding the last variable length field if not set
     * @return the bit length of this segment
     */
    pub fn bit_length(&self) -> usize {
        self.bit_length_of_field(self.len() as isize)
    }

    /**
     * Returns the bit length up to and including the specified field, excluding the last variable length field if not set
     * @param fieldIndex the index of the field up to which the segment length is calculated; negative index counts back from last field
     * @return the bit length of this segment up to and including the specified field, or 0 if the field does not exist
     */
    pub fn bit_length_of_field(&self, field_index: isize) -> usize {
        /* Check provided index */
        let last_field_index = self.get_absolute_index(field_index);
        if last_field_index < 0 {
            return 0;
        }

        /* Calculate segment bit length */
        let mut bit_length = 0;
        for index in 0..last_field_index as usize {
            bit_length += self[index].bit_length();
        }

        bit_length
    }

    /**
     * Gets the absolute field index and check if index is within bounds
     * @param index the absolute index of the requested field; negative index counts back from last field
     * @return the absolute field index or -1 if index out of bounds
     */
    fn get_absolute_index(&self, index: isize) -> isize {
        let length = self.len() as isize;

        if index >= 0 && index < length {
            return index;
        }

        if index < 0 && (length + index) >= 0 {
            return length + index;
        }

        return -1;
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
