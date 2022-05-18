use super::definitions::generic_header_fields;
use super::field::Field;

#[derive(Clone)]
pub struct MessageSegment {
    pub fields: Vec<Field>,
}

impl MessageSegment {
    pub fn from(fields: Vec<Field>) -> MessageSegment {
        MessageSegment { fields }
    }

    pub fn generic_header_segment() -> MessageSegment {
        MessageSegment::from(generic_header_fields().to_vec())
    }

    /*
     * Sets all field values of this segment with values from an array
     * @since 1.1
     * @param data array with the data to be set as the field values
     * @param startIndex starting position in the array
     * @return TRUE if the data was valid and all field values are set
     * @throws WfCoreException if the provided data is invalid
     */
    pub fn set_all<T: AsRef<str> + Into<String>>(&mut self, data: &[T], start_index: usize) {
        /* int nItems = data.length - startIndex;
        if (nItems < fields.length) {
            throw new WfCoreException("Message segment has " + fields.length + " fields, but received " + nItems + " items in array", null);
        } */
        let mut index = start_index;
        for field in &mut self.fields {
            /* if (Boolean.FALSE.equals(field.set(data[index]))) {
                throw new WfCoreException("Field " + field.debugInfo() + " already set or array item " + index + " contains invalid data: " + data[index], null);
            } */
            let value = &data[index];
            field.set(value.as_ref()).unwrap();
            index += 1;
        }

        //return this.isValid();
    }

    /**
     * Gets the value of the field specified by name
     * @param fieldname the name of the requested field
     * @return the field value, or NULL if field does not exist
     */
    pub fn get<T: AsRef<str>>(&self, field_name: T) -> Option<&String> {
        let value = self
            .fields
            .iter()
            .find(|f| f.name == field_name.as_ref())?
            .get();

        value.as_ref()
    }

    pub fn get_number_of_fields(&self) -> usize {
        self.fields.len()
    }

    /**
     * Encodes this message segment
     * @return a binary buffer with the binary encoded message segment and its bit length
     * @throws WfCoreException if the message cannot be encoded
     */
    pub fn encode(&self) -> (Vec<u8>, usize) {
        let mut buffer: Vec<u8> = vec![];
        let mut len = buffer.len();
        //let cursor = self.fields[0].start_byte;
        for field in &self.fields {
            /* if (field.startByte != byteCursor) {
                throw new WfCoreException("Invalid field order while encoding: did not expect field " + field.debugInfo() + " at byte " + byteCursor, null);
            } */
            let field_length = field.bit_length();
            //buffer.appendBits(field.encode(), field.bitLength());
            buffer = super::wf_codec::common::concatinate_bits(
                &buffer,
                len,
                &field.encode().expect("field had no value"),
                field_length,
            );

            len += field_length;
            //byteCursor = field.endByte;
        }

        (buffer, len)
    }

    /**
     * Decodes this message segment from the provided encoded message
     * @since 1.1
     * @param buffer the binary buffer with the binary encoded message
     * @param startBit the bit position where this segment starts in the encoded message
     * @param fieldIndex the index of the next field to be decoded
     * @throws WfCoreException if the message cannot be decoded
     */
    pub fn decode(
        &mut self,
        message_buffer: &[u8],
        message_buffer_bit_length: usize,
        start_bit: usize,
        field_index: usize,
    ) -> usize {
        /* Check if all fields already processed */
        if field_index > self.fields.len() {
            return 0;
        }

        let mut bit_cursor = start_bit;
        let mut byte_cursor = self.fields[field_index].start_byte;
        for field in &mut self.fields[field_index..] {
            if field.start_byte != byte_cursor {
                panic!("start byte should match byte cursor");
                //throw new WfCoreException("Invalid field order while decoding: did not expect field " + fields[index].debugInfo() + " at byte " + byteCursor, null);
            }
            /*
            try {
                buffer.extractMessageField(fields[index], bitCursor);
            } catch (WfCoreException e) {
                throw new WfCoreException("Could not decode field at bit " + bitCursor + " of buffer: " + buffer.toHexString(), e);
            } */

            let bit_length =
                field.extract_message_field(message_buffer, message_buffer_bit_length, bit_cursor);

            bit_cursor += bit_length; //field.bit_length();
            byte_cursor = field.end_byte as usize;
        }

        bit_cursor - start_bit
    }

    /**
     * Returns the bit length of this segment, excluding the last variable length field if not set
     * @return the bit length of this segment
     */
    pub fn bit_length(&self) -> usize {
        self.bit_length_of_field(self.fields.len() as isize)
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
            bit_length += self.fields[index].bit_length();
        }

        bit_length
    }

    /**
     * Gets the absolute field index and check if index is within bounds
     * @param index the absolute index of the requested field; negative index counts back from last field
     * @return the absolute field index or -1 if index out of bounds
     */
    fn get_absolute_index(&self, index: isize) -> isize {
        let length = self.fields.len() as isize;

        if index >= 0 && index < length {
            return index;
        }

        if index < 0 && (length + index) >= 0 {
            return length + index;
        }

        return -1;
    }
}
