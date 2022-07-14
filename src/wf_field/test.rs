use crate::wf_core::field::Field;
use crate::wf_codec::encoding::UTF8;

const FIELDNAME: &str = "TESTFIELD";

// all "add field" tests should be placed here

#[test]
fn test_add_field_utf() {

    let mut buffer: Vec<Field> = vec![];
    let mut field = Field::new(FIELDNAME, None, UTF8, 0, -1);
    
    let success = field.set("text");
    assert!(success.is_ok());

    let result = field.clone();

    buffer.push(field);

    //This really isn't the right test here
    assert_eq!(
        buffer.len(), //This will be 1 because it is just 1 item in Vec
        result.bit_length(), //this will be 32 because that's how big a Field is
        "Buffer bit length should be equal to field bit length"
    );

    /*
    buffer.addMessageField(field);
    assertEquals("Binary buffer length should be equal to field length", field.bitLength(), buffer.bitLength());
    assertTrue("Message field (UTF) should be correctly encoded and added", buffer.toHexString().equalsIgnoreCase("74657874"));
    */
}