use super::segment::MessageSegment;
use super::FieldValue;
use crate::wf_buffer::WhiteflagBuffer;
use crate::wf_field::{generic_header_fields, get_field_value_from_array, Field, FieldDefinition};
use crate::wf_parser::MessageCodeParser;

pub struct BasicMessage {
    message_code: char,
    header: MessageSegment,
    body: MessageSegment,
}

impl MessageSegment {
    pub fn serialize(&self) -> String {
        let mut serial: String = String::new();
        for f in self.iter() {
            let value: &str = &f.get();
            serial.push_str(value);
        }

        serial
    }
}

impl BasicMessage {
    pub fn compile<T: FieldValue>(data: &[T]) -> Self {
        let header = convert_values_to_fields(generic_header_fields().to_vec(), data.as_ref(), 0);

        let body_start_index = header.len();

        //need switch statement here
        let parser = MessageCodeParser::parse_for_encode(data);

        let body = convert_values_to_fields(
            parser.get_field_definitions_for_encode(),
            data.as_ref(),
            body_start_index,
        );

        BasicMessage::new(parser.code, header, body)
    }

    pub fn new(message_code: char, header: Vec<Field>, body: Vec<Field>) -> BasicMessage {
        BasicMessage {
            message_code,
            header: header.into(),
            body: body.into(),
        }
    }

    pub fn serialize(&self) -> String {
        let mut serial = String::new();
        serial.push_str(&self.header.serialize());
        serial.push_str(&self.body.serialize());

        serial
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut buffer = WhiteflagBuffer::default();

        buffer.encode(&mut self.header);
        buffer.encode(&mut self.body);

        buffer.crop()
    }

    pub fn encode_as_hex(&mut self) -> String {
        hex::encode(self.encode())
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
    fn get_option<T: AsRef<str>>(&self, fieldname: T) -> Option<&String> {
        get_field_value_from_array(&self.header, fieldname.as_ref())
            .or(get_field_value_from_array(&self.body, fieldname.as_ref()))
            .or(None)
    }

    pub fn get_fields(&self) -> Vec<&Field> {
        let mut fields: Vec<&Field> = vec![];
        fields.extend(self.header.iter());
        fields.extend(self.body.iter());
        fields
    }
}

impl<T: FieldValue> From<&[T]> for BasicMessage {
    fn from(data: &[T]) -> Self {
        BasicMessage::compile(data)
    }
}

/// converts string values to their respective fields relative to their position and the corresponding field definition
fn convert_values_to_fields<T: FieldValue>(
    field_defs: Vec<FieldDefinition>,
    data: &[T],
    start_index: usize,
) -> Vec<Field> {
    if (data.len() - start_index) < field_defs.len() {
        panic!("not enough field definitions to process given values\nvalues: {:#?}\ndefinitions: {:#?}", data, field_defs);
    }

    let mut index = start_index;
    field_defs
        .into_iter()
        .map(|f| {
            /* if (Boolean.FALSE.equals(field.set(data[index]))) {
                throw new WfCoreException("Field " + field.debugInfo() + " already set or array item " + index + " contains invalid data: " + data[index], null);
            } */
            let value = &data[index];
            let field = match f.set(value.as_ref()) {
                Ok(field) => field,
                Err(e) => panic!(
                    "{} error while converting array of strings into fields\n{0:?}",
                    e
                ),
            };
            index += 1;
            field
        })
        .collect()
    //return this.isValid();
}
