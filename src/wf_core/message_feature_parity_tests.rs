use crate::{
    wf_account::{account::WfAccount, test_impl::WhiteflagAccount},
    wf_buffer::{CryptMode, WhiteflagBuffer},
    wf_core::basic_message::BasicMessage,
    wf_crypto::{ecdh_keypair::WhiteflagECDHKeyPair, wf_encryption_key::WhiteflagEncryptionKey},
};
use fennel_lib::FennelCipher;

fn test(values: &[&str]) {
    let message = BasicMessage::compile(values);
    assert_eq!(values.concat(), message.serialize());
}

#[test]
fn crypto_message_compilation() {
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "K",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "11",
        "d426bbe111221675e333f30ef608b1aa6e60a47080dd33cb49e96395894ef42f",
    ];
    test(&field_values);
}

#[test]
fn auth_message_compilation() {
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "A",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "1",
        "b01218a30dd3c23d050af254bfcce31a715fecdff6a23fd59609612e6e0ef263",
    ];
    test(&field_values);
}

#[test]
fn auth_message_serialization() {
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "A",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "2",
        "b01218a30dd3c23d050af254bfcce31a715fecdff6a23fd59609612e6e0ef263",
    ];
    test(&field_values);
}

#[test]
fn auth_message_deserialization() {
    let message_serialized = "WF100A000000000000000000000000000000000000000000000000000000000000000001https://organisation.int/whiteflag";
    let message = BasicMessage::deserialize(message_serialized);
    assert_eq!(message_serialized, message.serialize());
}

#[test]
fn auth_message_decoding() {
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "A",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "1",
        "https://organisation.int/whiteflag",
    ];
    let message = BasicMessage::decode_from_hexadecimal("5746313020800000000000000000000000000000000000000000000000000000000000000000b43a3a38399d1797b7b933b0b734b9b0ba34b7b71734b73a17bbb434ba32b33630b380");
    assert_eq!(field_values.concat(), message.serialize());
}

/* #[test]
#[should_panic(expected = "WhiteflagError")]
fn testInvalidMessage() {
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "X",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "1",
        "b01218a30dd3c23d050af254bfcce31a715fecdff6a23fd59609612e6e0ef263",
    ];
    BasicMessage::compile(&field_values);
} */

#[test]
fn sign_signal_message_encoding() {
    let message_encoded = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084";
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "M",
        "4",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];
    let message = BasicMessage::compile(&field_values);
    assert_eq!(message_encoded, message.encode_as_hex());
}

#[test]
fn sign_signal_message_decoding() {
    let message_encoded = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084";
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "M",
        "4",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];
    let message = BasicMessage::decode_from_hexadecimal(message_encoded);
    assert_eq!(field_values.concat(), message.serialize());
}

#[test]
fn test_message() {
    let message_serialized = "WF101T33efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3aeM802013-08-31T04:29:15ZP00D00H00M22+30.79658-037.8260287653210042";
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "T",
        "3",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "M",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];
    let message = BasicMessage::compile(&field_values);
    let message_encoded = message.encode_as_hex();
    let message_decoded = BasicMessage::decode_from_hexadecimal(message_encoded);

    assert_eq!(message_serialized, message.serialize());
    assert_eq!(message_serialized, message_decoded.serialize());
}

#[test]
fn request_message() {
    let message_serialized = "WF101Q13efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae802013-08-31T04:29:15ZP01D00H00M22+31.79658-033.826028799321000010022003";
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "Q",
        "1",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P01D00H00M",
        "22",
        "+31.79658",
        "-033.82602",
        "8799",
        "3210",
        "000",
        "10",
        "02",
        "20",
        "03",
    ];
    let message = BasicMessage::compile(&field_values);
    let message_encoded = message.encode_as_hex();
    let message_decoded = BasicMessage::decode_from_hexadecimal(message_encoded);

    assert_eq!(message_serialized, message.serialize());
    assert_eq!(message_serialized, message_decoded.serialize());
}

#[test]
fn free_text_message() {
    let message1 = BasicMessage::deserialize("WF100F5f6c1e1ed8950b137bb9e0edcf21593d62c03a7fb39dacfd554c593f72c8942dfWhiteflag test message!");
    let message2 = BasicMessage::decode_from_hexadecimal("57463130232fb60f0f6c4a8589bddcf076e790ac9eb1601d3fd9ced67eaaa62c9fb9644a16fabb434ba32b33630b3903a32b9ba1036b2b9b9b0b3b2908");

    assert_eq!(message1.serialize(), message2.serialize());
}

#[test]
fn message_encryption_1() {
    let encoded_msg = "5746313223000000000088888889111111119999999a22222222aaaaaaab33333333bbbbbbbb0983098309830983118b118b118b118b1993199319931993219b219b219b219b29a329a329a329a331ab31ab31ab31a9b1b9b1b9b1b9b1b9c1c9c1c9c1c9c1c8";
    let encrypted_msg = "574631326d7658e7d17479677a0de95076989fcd7825b709349b143f2b17644e5cb2c8ded5c7f18d77447cf9dc2115e0c1c81d717b57fadaeedf27bfef8926448ff666d3d9a65168827c94b393974ebbe6b7f0599e184bfd1ace3569117c23ae17c5640f2f2d";

    let mut key = WhiteflagEncryptionKey::from_preshared_key(
        "32676187ba7badda85ea63a69870a7133909f1999774abb2eed251073616a6e7",
    );

    let mut address = WhiteflagBuffer::decode_from_hexadecimal(
        "007a0baf6f84f0fa7402ea972686e56d50b707c9b67b108866",
    )
    .unwrap();

    key.set_context(&address.to_byte_array());

    //40aa85015d24e4601448c1ba8d7bf1aa
    let iv = vec![
        64, 170, 133, 1, 93, 36, 228, 96, 20, 72, 193, 186, 141, 123, 241, 170,
    ];
    let cipher = key.aes_256_ctr_cipher(&iv);

    let message = BasicMessage::decode_from_hexadecimal(encoded_msg);

    assert_eq!(
        encrypted_msg,
        hex::encode(message.encode_and_crypt(&cipher, CryptMode::Encrypt))
    );
}

#[test]
fn message_encryption_2() {
    let message_serialized = "WF120F5f6c1e1ed8950b137bb9e0edcf21593d62c03a7fb39dacfd554c593f72c8942dfWhiteflag test message!";

    let mut key = WhiteflagEncryptionKey::from_preshared_key(
        "b50cf705febdc9b6b2f7af10fa0955c1a5b454d6941494536d75d7810010a90d",
    );

    let mut address = WhiteflagBuffer::decode_from_hexadecimal(
        "ac000cdbe3c49955b218f8397ddfe533a32a4269658712a2f4a82e8b448e",
    )
    .unwrap();

    key.set_context(&address.to_byte_array());

    let iv = fennel_lib::generate_random_buffer(16);
    let cipher = key.aes_256_ctr_cipher(&iv);

    let message1 = BasicMessage::deserialize(message_serialized);
    let encrypted_message = message1.encode_and_crypt(&cipher, CryptMode::Encrypt);
    let message2 = BasicMessage::decode_and_crypt(encrypted_message, &cipher);

    assert_eq!(message_serialized, message2.serialize());
}

#[test]
fn message_encryption_3() {
    let message_serialized = "WF111Q13efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae802013-08-31T04:29:15ZP01D00H00M22+31.79658-033.826028799321000010022003";

    let originator = WhiteflagECDHKeyPair::default();
    let recipient = WhiteflagECDHKeyPair::default();

    let mut key = WhiteflagEncryptionKey::from_ecdh_key(originator.as_ref(), &recipient);
    let mut address = WhiteflagBuffer::decode_from_hexadecimal(
        "b77b1cdb02efe1acccf0e277021cb303117bd83c689ea8a64fc549229dba",
    )
    .unwrap();

    key.set_context(&address.to_byte_array());

    let iv = fennel_lib::generate_random_buffer(16);
    let cipher = key.aes_256_ctr_cipher(&iv);

    let message1 = BasicMessage::deserialize(message_serialized);
    assert_eq!(
        message_serialized,
        message1.serialize(),
        "failing immediately"
    );
    let encrypted_message = message1.encode_and_crypt(&cipher, CryptMode::Encrypt);
    let message2 = BasicMessage::decode_and_crypt(encrypted_message, &cipher);

    assert_eq!(message_serialized, message2.serialize());
}

/*

#[test]
#[should_panic(expected = "WhiteflagError")]
fn testMessageEncryption4() {
    let mut originator = WhiteflagAccount::new(false);
    let mut recipient = WhiteflagAccount::new(true);
    originator.set_address("b77b1cdb02efe1acccf0e277021cb303117bd83c689ea8a64fc549229dba".to_string());
    recipient.set_ecdh_keypair(generate_wfkeypair());

    let message = BasicMessage::deserialize("WF111Q13efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae802013-08-31T04:29:15ZP01D00H00M22+31.79658-033.826028799321000010022003").unwrap();
    message.set_originator(originator);
    message.set_recipient(recipient);

    message.encode();
    message.get_init_vector();
}
 */
