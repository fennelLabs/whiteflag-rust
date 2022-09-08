use crate::definitions::*;
use crate::FieldDefinition;

#[test]
fn codec_positions_should_be_valid() {
    test_bit_positions(&header::PREFIX, 0, 16);
    test_bit_positions(&header::MESSAGE_CODE, 33, 41);
    test_bit_positions(&header::ENCRYPTION_INDICATOR, 24, 32);
    test_bit_positions(&header::REFERENCED_MESSAGE, 45, 301);
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

/* fn print() {
    ParsedFieldDefinition::header().iter().for_each(|f| {
        println!(
            "{}\n\tstart: {}\n\tend: {}\n",
            f.name.unwrap_or("NA"),
            f.start_bit,
            f.end_bit
        );
    });
} */

#[test]
fn test_whiteflag_fields() {
    assert_eq!(WhiteflagFields::HeaderPrefix.get_start_bit(), 0);
    assert_eq!(WhiteflagFields::HeaderMessageCode.get_start_bit(), 33);
    assert_eq!(WhiteflagFields::HeaderReferencedMessage.get_start_bit(), 45);
    assert_eq!(
        WhiteflagFields::HeaderReferencedMessage
            .get_byte_config()
            .bit_length(),
        301 - 45
    );
    assert_eq!(WhiteflagFields::TestPseudoMessageCode.get_start_bit(), 301);
}
