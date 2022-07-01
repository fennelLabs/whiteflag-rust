use super::field::Field;
use super::segment::MessageSegment;
use super::wf_buffer::common::append_bits;

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

    pub fn encode(&self) -> (Vec<u8>, usize) {
        let mut buffer: Vec<u8> = vec![];
        let mut len = 0;

        let (header_buffer, header_len) = self.header.encode();
        let (body_buffer, body_len) = self.body.encode();

        (buffer, len) = append_bits(&buffer, len, &header_buffer, header_len);
        (buffer, len) = append_bits(&buffer, len, &body_buffer, body_len);

        (buffer, len)
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
        self.header
            .get(fieldname.as_ref())
            .or(self.body.get(fieldname.as_ref()))
            .or(None)
    }

    pub fn get_fields(&self) -> Vec<&Field> {
        let mut fields: Vec<&Field> = vec![];
        fields.extend(&self.header.fields);
        fields.extend(&self.body.fields);
        fields
    }
}
