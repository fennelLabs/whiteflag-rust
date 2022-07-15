use super::WhiteflagBuffer;
use crate::wf_field::{Field, FieldDefinition};

impl WhiteflagBuffer {
    /**
     * Decodes this message segment from the provided encoded message
     * @since 1.1
     * @param buffer the binary buffer with the binary encoded message
     * @param startBit the bit position where this segment starts in the encoded message
     * @param fieldIndex the index of the next field to be decoded
     * @throws WfCoreException if the message cannot be decoded
     */
    pub fn decode(
        &self,
        field_defs: Vec<FieldDefinition>,
        start_bit: usize,
    ) -> (usize, Vec<Field>) {
        if field_defs.len() < 1 {
            panic!("field definition vector should not be empty")
        }

        let mut bit_cursor = start_bit;
        let mut byte_cursor = field_defs[0].start_byte;

        let fields = field_defs
            .into_iter()
            .map(|f| {
                if f.start_byte != byte_cursor {
                    panic!("start byte should match byte cursor");
                    //throw new WfCoreException("Invalid field order while decoding: did not expect field " + fields[index].debugInfo() + " at byte " + byteCursor, null);
                }
                /*
                try {
                    buffer.extractMessageField(fields[index], bitCursor);
                } catch (WfCoreException e) {
                    throw new WfCoreException("Could not decode field at bit " + bitCursor + " of buffer: " + buffer.toHexString(), e);
                } */

                let (bit_length, field) = self.extract_message_field(f, bit_cursor);

                bit_cursor += bit_length; //field.bit_length();
                byte_cursor = field.definition.end_byte as usize;

                field
            })
            .collect();

        (bit_cursor - start_bit, fields)
    }
}
