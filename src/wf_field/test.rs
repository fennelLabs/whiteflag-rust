use crate::wf_codec::encoding::UTF8;
use crate::{wf_buffer::WhiteflagBuffer, wf_field::Field, wf_field::FieldDefinition};
use crate::wf_buffer::common::{to_hex};

const FIELDNAME: &str = "TESTFIELD";

#[test]
fn test_add_field_utf() {

    let buffer: Vec<u8> = vec![];
    let mut wf_buffer: WhiteflagBuffer = WhiteflagBuffer::new(buffer, 0); 

    let mut field = Field::new(FIELDNAME, None, UTF8, 0, -1);
    let success = field.set("text");
    
    assert!(success.is_ok());

    wf_buffer.append_field(&field);

    let (size, extracted_field) = wf_buffer.extract_message_field(FieldDefinition::new(FIELDNAME, None, UTF8, 0, -1), 0);

    assert_eq!(
        field.bit_length(), 
        size,
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "74657874",
        to_hex(&extracted_field.encode().expect("correct string")),
        "Message field (UTF) should be correctly encoded and added"
    );
}