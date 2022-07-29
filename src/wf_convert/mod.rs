use crate::wf_core::basic_message::BasicMessage;
use crate::wf_field::{generic_header_fields, get_body_from_code, Field, FieldDefinition};

pub trait FieldValue: AsRef<str> + Into<String> + std::fmt::Debug {}
impl<T> FieldValue for T where T: AsRef<str> + Into<String> + std::fmt::Debug {}

pub fn compile<T: FieldValue>(data: &[T]) -> BasicMessage {
    let header = convert_values_to_fields(generic_header_fields().to_vec(), data.as_ref(), 0);

    let body_start_index = header.len();

    //need switch statement here
    let parser = MessageCodeParser::parse(data);

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

#[derive(Debug)]
pub struct MessageCodeParser {
    code: char,
    test_code: Option<char>,
}

impl MessageCodeParser {
    /// extracts message code type from array of message values
    /// the 4th position is where the message code type resides
    /// if this is a test message (code = T) then there should be a psuedo message code to be extracted
    pub fn parse<T: FieldValue>(data: &[T]) -> Self {
        if data.len() < 6 {
            panic!(
                "a valid message must contain at least a header which is 7 values long\n{:#?}",
                data.as_ref()
            );
        }

        let code: char = convert_value_to_code(data[4].as_ref());
        let test_code = if code == 'T' {
            data.iter()
                .nth(7)
                .map(|v| convert_value_to_code(v.as_ref()))
        } else {
            None
        };

        MessageCodeParser { code, test_code }
    }

    /// collects all the field definitions based on the parsed codes
    pub fn get_field_definitions(&self) -> Vec<FieldDefinition> {
        let mut defs = get_body_from_code(&self.code);

        match &self.test_code {
            Some(c) => {
                defs.append(get_body_from_code(c).as_mut());
            }
            None => (),
        };

        defs
    }
}

/// fields that are codes are single characters
fn convert_value_to_code(value: &str) -> char {
    value
        .chars()
        .nth(0)
        .unwrap_or_else(|| panic!("invalid message code"))
}
