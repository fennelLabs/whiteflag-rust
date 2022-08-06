use std::ops::Mul;

use super::{definitions::*, FieldDefinition};

// * Returns an array with additional Whiteflag sign/signal message body request fields
// * @param n the number of request objects
// * @return an array with the request message fields
// * @wfver v1-draft.6
// * @wfref 4.3.1.9 Object Request Fields

pub fn create_request_fields(n: usize) -> Vec<FieldDefinition> {
    let object_type = Request::OBJECT_TYPE;
    let object_type_quant = Request::OBJECT_TYPE_QUANT;

    let ot_size = object_type.expected_byte_length().unwrap_or(0);
    let ot_quant_size = object_type_quant.expected_byte_length().unwrap_or(0);

    let mut start_byte = object_type.start_byte;

    (0..(n.mul(2)))
        .step_by(2)
        .map(|i| {
            let n_field = (i / 2) + 1;
            let byte_start = start_byte;
            let byte_split = byte_start + ot_size;
            let byte_end = byte_split + ot_quant_size;

            start_byte = byte_end;

            let n1 = object_type.namer(None, Some(n_field));
            let n2 = n1.new_namer(None, Some(format!("{}{}", n_field, "Quant")));

            [
                FieldDefinition::new_from_namer(
                    n1,
                    object_type.encoding.kind.get_encoding(),
                    byte_start,
                    byte_split,
                ),
                FieldDefinition::new_from_namer(
                    n2,
                    object_type_quant.encoding.kind.get_encoding(),
                    byte_split,
                    byte_end,
                ),
            ]
        })
        .flatten()
        .inspect(|f| println!("{:#?}", f))
        .collect()
}
