use super::WhiteflagBuffer;
use wf_field::Field;

impl WhiteflagBuffer {
    /// encodes given array of [`Field`] and appends them into the buffer
    ///
    /// # Arguments
    ///
    /// * `fields` - array of fields to append and encode into the buffer
    pub fn encode(&mut self, fields: &[Field]) {
        fields.into_iter().for_each(|f| self.append_field(f));
    }

    pub fn append_field(&mut self, field: &Field) {
        self.append(field.into(), None);
    }
}
