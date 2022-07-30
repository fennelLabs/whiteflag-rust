use crate::wf_core::basic_message::BasicMessage;
use crate::wf_field::{generic_header_fields, get_body_from_code, Field, FieldDefinition};
use crate::wf_parser::MessageCodeParser;

pub trait FieldValue: AsRef<str> + Into<String> + std::fmt::Debug {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> + std::fmt::Debug {}

pub fn compile<T: FieldValue>(data: &[T]) -> BasicMessage {
    let header = convert_values_to_fields(generic_header_fields().to_vec(), data.as_ref(), 0);

    let body_start_index = header.len();

    //need switch statement here
    let parser = MessageCodeParser::parse_for_encode(data);

    let body = convert_values_to_fields(
        parser.get_field_definitions(),
        data.as_ref(),
        body_start_index,
    );

    BasicMessage::new(parser.code, header, body)
}

impl<T: FieldValue> From<&[T]> for BasicMessage {
    fn from(data: &[T]) -> Self {
        compile(data)
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
                    "error while converting array of strings into fields\n{:?}",
                    e
                ),
            };
            index += 1;
            field
        })
        .collect()
    //return this.isValid();
}
