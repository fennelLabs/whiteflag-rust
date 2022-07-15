use super::segment::MessageSegment;
use super::wf_buffer::common::{append_bits, crop_bits};
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_field::{get_field_value_from_array, Field};

pub struct BasicMessage {
    message_code: char,
    header: MessageSegment,
    body: MessageSegment,
}

impl BasicMessage {
    pub fn new(message_code: char, header: MessageSegment, body: MessageSegment) -> BasicMessage {
        BasicMessage {
            message_code,
            header,
            body,
        }
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut buffer = WhiteflagBuffer::default();

        buffer.encode(&mut self.header.fields);
        buffer.encode(&mut self.body.fields);

        buffer.crop()
    }

    /**
     * Gets the value of the specified field
     * @param fieldname the name of the requested field
     * @return the field value, or NULL if field does not exist
     */
    pub fn get<T: AsRef<str>>(&self, fieldname: T) -> String {
        self.get_option(fieldname)
            .expect("no value found")
            .to_string()
    }

    /**
     * Gets the value of the specified field
     * @param fieldname the name of the requested field
     * @return the field value, or NULL if field does not exist
     */
    pub fn get_option<T: AsRef<str>>(&self, fieldname: T) -> Option<&String> {
        get_field_value_from_array(&self.header.fields, fieldname.as_ref())
            .or(get_field_value_from_array(
                &self.body.fields,
                fieldname.as_ref(),
            ))
            .or(None)
    }

    pub fn get_fields(&self) -> Vec<&Field> {
        let mut fields: Vec<&Field> = vec![];
        fields.extend(&self.header.fields);
        fields.extend(&self.body.fields);
        fields
    }
}
