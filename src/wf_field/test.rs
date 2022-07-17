use crate::wf_codec::encoding::{UTF8, BIN};
use crate::{wf_buffer::WhiteflagBuffer, wf_field::Field, wf_field::FieldDefinition};
use crate::wf_buffer::common::{to_hex};


const FIELDNAME: &str = "TESTFIELD";

#[test]
fn test_add_field_utf() {

    let buffer: Vec<u8> = vec![];
    let mut buffer: WhiteflagBuffer = WhiteflagBuffer::new(buffer, 0); 

    let mut field = Field::new(FIELDNAME, None, UTF8, 0, -1);
    let success = field.set("text");
    
    assert!(success.is_ok());

    buffer.append_field(&field);

    let (size, extracted_field) = buffer.extract_message_field(FieldDefinition::new(FIELDNAME, None, UTF8, 0, -1), 0);

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

#[test]
fn test_extract_field_utf() {
 
    let byte_array_1: Vec<u8> = vec![0x95, 0x74, 0x78, 0x74]; 

    let buffer: WhiteflagBuffer = WhiteflagBuffer::new(byte_array_1, 32); 
    let field = Field::new(FIELDNAME, None, UTF8, 0, -1);

    let field_definition: FieldDefinition = FieldDefinition::new(FIELDNAME, None, UTF8, 0, -1);
    let (_, extracted_field) = buffer.extract_message_field(field_definition, 8);

    assert_eq!(
        "txt",
        extracted_field.get().as_ref().unwrap(),
        "Extracted message field (UTF) should contain the correct value"
    );
    
}

#[test]
fn test_add_field_bin_1() {
    let buffer: Vec<u8> = vec![];
    let mut buffer: WhiteflagBuffer = WhiteflagBuffer::new(buffer, 0); 

    let mut field = Field::new(FIELDNAME, None, BIN, 0, -1);

    let success = field.set("01");
    assert!(success.is_ok());

    buffer.append_field(&field);

    let (size, extracted_field) = buffer.extract_message_field(FieldDefinition::new(FIELDNAME, None, BIN, 0, -1), 0);

    assert_eq!(
        field.bit_length(), 
        size,
        "Buffer bit length should be equal to field bit length"
    );

    assert_eq!(
        "40", 
        to_hex(&extracted_field.encode().expect("correct string")),
        "Message field (bin) should be correctly encoded and added"
    );

}

//@Test
//public void testAddFieldBin1() throws WfCoreException {


    /* Verify */
    //assertTrue("Should be able to set field value", field.set("01"));
    //buffer.addMessageField(field);
    //assertEquals("Binary buffer length should be equal to field length", field.bitLength(), buffer.bitLength());
    //assertTrue("Message field (bin) should be correctly encoded and added", buffer.toHexString().equalsIgnoreCase("40"));
//}