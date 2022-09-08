use crate::definitions::*;
use crate::{test_message_code, FieldDefinition, MessageHeaderOrder, ParsedFieldDefinition};
#[test]
fn codec_positions_should_be_valid() {
    test_bit_positions(&header::PREFIX, 0, 16);
    test_bit_positions(&header::MESSAGE_CODE, 33, 41);
    test_bit_positions(&header::ENCRYPTION_INDICATOR, 24, 32);
    test_bit_positions(&header::REFERENCED_MESSAGE, 45, 301);

    /* println!(
        "{}",
        ParsedFieldDefinition::header()[MessageHeaderOrder::ReferencedMessage.as_usize()]
            .next(test_message_code())
            .end_bit
    ); */

    test_bit_positions(&test::PSEUDO_MESSAGE_CODE, 301, 309);
}

fn test_bit_positions(def: &FieldDefinition, start: usize, end: usize) {
    let name = def.name.unwrap_or("NA");
    assert_eq!(
        def.positions.bit_start, start,
        "{} start bit is wrong",
        name
    );
    assert_eq!(def.positions.bit_end, end, "{} end bit is wrong", name);
    assert_eq!(
        end - start,
        def.positions.bytes.bit_length(),
        "{} bit_length() is wrong",
        name
    );
}

fn print() {
    ParsedFieldDefinition::header().iter().for_each(|f| {
        println!(
            "{}\n\tstart: {}\n\tend: {}\n",
            f.name.unwrap_or("NA"),
            f.start_bit,
            f.end_bit
        );
    });
}
