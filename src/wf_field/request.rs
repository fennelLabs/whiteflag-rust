use super::{definitions::*, Field, FieldDefinition};
use crate::wf_parser::FieldDefinitionParser;
use std::ops::Mul;

const OBJECT_TYPE: FieldDefinition = Request::OBJECT_TYPE;
const OBJECT_TYPE_QUANT: FieldDefinition = Request::OBJECT_TYPE_QUANT;

/// there can be any amount of request field pairs at the end of the message
/// this function takes n number of request objects and parsers out the remaining request fields
/// 
/// wf spec 4.3.1.9 Object Request Fields
pub fn create_request_fields<T: FieldDefinitionParser>(n: usize, parser: &mut T) -> Vec<Field> {
    let ot_size = OBJECT_TYPE
        .expected_byte_length()
        .expect("request::OBJECT_TYPE is misconfigured: must have a start and end byte");
    let ot_quant_size = OBJECT_TYPE_QUANT
        .expected_byte_length()
        .expect("request::OBJECT_TYPE_QUANT is misconfigured: must have a start and end byte");
    let name = OBJECT_TYPE
        .get_name()
        .expect("request::OBJECT_TYPE is misconfigured: should have a name");

    let mut start_byte = OBJECT_TYPE.start_byte;

    (0..(n.mul(2)))
        .step_by(2)
        .map(|i| {
            let n_field = (i / 2) + 1;
            let byte_start = start_byte;
            let byte_split = byte_start + ot_size;
            let byte_end = byte_split + ot_quant_size;

            start_byte = byte_end;

            let ot = FieldDefinition::new_without_name(
                OBJECT_TYPE.encoding.kind.get_encoding(),
                byte_start,
                byte_split,
            );

            let oq = FieldDefinition::new_without_name(
                OBJECT_TYPE_QUANT.encoding.kind.get_encoding(),
                byte_split,
                byte_end,
            );

            [
                Field::new_with_name(parser.parse(&ot), format!("{}{}", name, n_field), ot),
                Field::new_with_name(parser.parse(&oq), format!("{}{}Quant", name, n_field), oq),
            ]
        })
        .flatten()
        .collect()
}
