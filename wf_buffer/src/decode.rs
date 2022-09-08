use super::WhiteflagBuffer;
use wf_common::common::extract_bits;
use wf_field::{Field, FieldDefinition};

impl WhiteflagBuffer {
    /// Decodes current buffer using their [`FieldDefinition`] into [`Field`]
    ///
    /// # Arguments
    /// * `field_defs` - field definitions required to decode the buffer
    /// * `start_bit` - the bit position where this segment starts in the encoded buffer
    pub fn decode(&self, field_defs: &[FieldDefinition], start_bit: usize) -> (usize, Vec<Field>) {
        if field_defs.len() < 1 {
            panic!("field definition vector should not be empty")
        }

        let mut bit_cursor = start_bit;

        let fields = field_defs
            .into_iter()
            .map(|f| {
                let field = self.extract_message_field(f, bit_cursor);
                bit_cursor += field.bit_length();

                field
            })
            .collect();

        (bit_cursor, fields)
    }

    pub fn extract_message_field(&self, definition: &FieldDefinition, start_bit: usize) -> Field {
        let value = self.extract_message_value(definition, start_bit);
        Field::new(definition.clone(), value)
    }

    pub fn extract_message_value(&self, definition: &FieldDefinition, start_bit: usize) -> String {
        let field_bit_length = definition.bit_length();
        let bit_length = if field_bit_length >= 1 {
            field_bit_length
        } else {
            let mut bit_length = self.bit_length - start_bit;
            bit_length -= bit_length % &definition.bytes.encoding.bit_length;
            bit_length
        };

        let field_buffer: Vec<u8> =
            extract_bits(&self.data, self.bit_length, start_bit, bit_length);

        definition.decode(&field_buffer)
    }
}
