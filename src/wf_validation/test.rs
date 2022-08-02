use super::*;
use crate::wf_field::definitions;

#[test]
fn incorrect_length_1() {
    // referenced message can only be 64 bytes long
    let def = definitions::Header::REFERENCED_MESSAGE;
    let data = "000000000000000000000000000000000000000000000000000000000000000";

    // ensure we have an incorrect length
    assert!(data.len() != 64);

    // ensure error is thrown during validation
    assert!(
        matches!(
            def.validate(data),
            Err(ValidationError::InvalidLength(_, _))
        ),
        "invalid length should be thrown"
    );
}

#[test]
fn incorrect_length_2() {
    // referenced message can only be 64 bytes long
    let def = definitions::Sign::OBJECT_ORIENTATION;
    let data = "1234";

    // ensure we have an incorrect length
    assert!(data.len() != 3);

    // ensure error is thrown during validation
    assert!(
        matches!(
            def.validate(data),
            Err(ValidationError::InvalidLength(_, _))
        ),
        "invalid length should be thrown"
    );
}
