use super::WhiteflagBuffer;
use wf_field::Field;

impl WhiteflagBuffer {
    /**
     * Encodes this message segment
     * @return a binary buffer with the binary encoded message segment and its bit length
     * @throws WfCoreException if the message cannot be encoded
     */
    pub fn encode(&mut self, fields: &[Field]) {
        //let cursor = self.fields[0].start_byte;
        fields.into_iter().for_each(|f| {
            /* if (field.startByte != byteCursor) {
                throw new WfCoreException("Invalid field order while encoding: did not expect field " + field.debugInfo() + " at byte " + byteCursor, null);
            } */

            self.append_field(f);

            //byteCursor = field.endByte;
        });
    }
}
