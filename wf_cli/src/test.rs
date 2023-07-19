use crate::WhiteflagCLICommands;

use super::auth::{acquire_auth_lock, check_auth_lock, release_auth_lock};

#[test]
fn test_auth_lock_system() {
    acquire_auth_lock();
    assert!(check_auth_lock());
    assert!(release_auth_lock());
}

#[test]
fn test_encode_auth_message() {
    let json_string = "{
        \"prefix\": \"WF\",
        \"version\": \"1\",
        \"encryptionIndicator\": \"1\",
        \"duressIndicator\": \"0\",
        \"messageCode\": \"A\",
        \"referenceIndicator\": \"0\",
        \"referencedMessage\": \"0000000000000000000000000000000000000000000000000000000000000000\",
        \"verificationMethod\": \"1\",
        \"verificationData\": \"https://organisation.int/whiteflag\"
    }";
    assert!(WhiteflagCLICommands::encode(json_string).is_ok());
}

#[test]
fn test_decode_auth_message() {
    let hex_string = "5746313120800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380";
    assert!(WhiteflagCLICommands::decode(hex_string).is_ok());
}

#[test]
fn test_encode_message() {
    let json_string = "{\"prefix\":\"WF\",\"version\":\"1\",\"encryptionIndicator\":\"0\",\"duressIndicator\":\"1\",\"messageCode\":\"M\",\"referenceIndicator\":\"4\",\"referencedMessage\":\"3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae\",\"subjectCode\":\"80\",\"dateTime\":\"2013-08-31T04:29:15Z\",\"duration\":\"P00D00H00M\",\"objectType\":\"22\",\"objectLatitude\":\"+30.79658\",\"objectLongitude\":\"-037.82602\",\"objectSizeDim1\":\"8765\",\"objectSizeDim2\":\"3210\",\"objectOrientation\":\"042\"}";
    let result = WhiteflagCLICommands::encode(json_string);
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[test]
fn test_decode_message() {
    let hex_string = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084";
    let result = WhiteflagCLICommands::decode(hex_string);
    assert_eq!(result.unwrap(), "{\"prefix\":\"WF\",\"version\":\"1\",\"encryptionIndicator\":\"0\",\"duressIndicator\":\"1\",\"messageCode\":\"M\",\"referenceIndicator\":\"4\",\"referencedMessage\":\"3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae\",\"subjectCode\":\"80\",\"dateTime\":\"2013-08-31T04:29:15Z\",\"duration\":\"P00D00H00M\",\"objectType\":\"22\",\"objectLatitude\":\"+30.79658\",\"objectLongitude\":\"-037.82602\",\"objectSizeDim1\":\"8765\",\"objectSizeDim2\":\"3210\",\"objectOrientation\":\"042\"}");
    let result = WhiteflagCLICommands::decode(hex_string);
    assert!(result.is_ok());
}