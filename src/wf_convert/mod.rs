use crate::wf_core::basic_message::BasicMessage;
use crate::wf_core::segment::MessageSegment;
use crate::wf_field::{generic_header_fields, get_message_body, Field, FieldDefinition};

pub trait FieldValue: AsRef<str> + Into<String> {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> {}

fn compile<T: FieldValue>(data: &[T]) -> BasicMessage {
    let header = set_all(generic_header_fields().to_vec(), data.as_ref(), 0);

    let (body_field_defs, code) = get_message_body(&header);
    let body_start_index = header.len();

    //need switch statement here

    let body = set_all(body_field_defs, data.as_ref(), body_start_index);

    BasicMessage::new(
        code,
        MessageSegment::from(header),
        MessageSegment::from(body),
    )
}

impl<T: FieldValue> From<&[T]> for BasicMessage {
    fn from(data: &[T]) -> Self {
        compile(data)
    }
}

/*
 * Sets all field values of this segment with values from an array
 * @since 1.1
 * @param data array with the data to be set as the field values
 * @param startIndex starting position in the array
 * @return TRUE if the data was valid and all field values are set
 * @throws WfCoreException if the provided data is invalid
 */
fn set_all<T: FieldValue>(
    field_defs: Vec<FieldDefinition>,
    data: &[T],
    start_index: usize,
) -> Vec<Field> {
    /* int nItems = data.length - startIndex;
    if (nItems < fields.length) {
        throw new WfCoreException("Message segment has " + fields.length + " fields, but received " + nItems + " items in array", null);
    } */
    let mut index = start_index;
    field_defs
        .into_iter()
        .map(|f| {
            /* if (Boolean.FALSE.equals(field.set(data[index]))) {
                throw new WfCoreException("Field " + field.debugInfo() + " already set or array item " + index + " contains invalid data: " + data[index], null);
            } */
            let value = &data[index];
            let field = match f.set(value.as_ref()) {
                Ok(field) => {
                    println!("Message field set successfully.");
                    field
                }
                Err(e) => panic!("{:?}", e),
            };
            index += 1;
            field
        })
        .collect()
    //return this.isValid();
}
