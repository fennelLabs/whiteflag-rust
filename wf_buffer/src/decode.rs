use super::WhiteflagBuffer;
use wf_field::{Field, FieldDefinition};

impl WhiteflagBuffer {
    /**
     * Decodes this message segment from the provided encoded message
     * @since 1.1
     * @param buffer the binary buffer with the binary encoded message
     * @param startBit the bit position where this segment starts in the encoded message
     * @param fieldIndex the index of the next field to be decoded
     * @throws WfCoreException if the message cannot be decoded
     */
    pub fn decode(&self, field_defs: &[FieldDefinition], start_bit: usize) -> (usize, Vec<Field>) {
        if field_defs.len() < 1 {
            panic!("field definition vector should not be empty")
        }

        let mut bit_cursor = start_bit;

        // the byte cursor only ensures definitions are in their proper order relative to each other
        let mut byte_cursor = field_defs[0].positions.bytes.start;

        let fields = field_defs
            .into_iter()
            .map(|f| {
                if f.positions.bytes.start != byte_cursor {
                    panic!(
                        "\nstart byte should match byte cursor\n\tcursor: {}\n\tfield: {:#?}",
                        byte_cursor, f
                    );
                }

                let field = self.extract_message_field(f, bit_cursor);

                bit_cursor += field.bit_length();
                byte_cursor = field.definition.positions.bytes.end.unwrap_or(0);

                field
            })
            .collect();

        (bit_cursor, fields)
    }
}
