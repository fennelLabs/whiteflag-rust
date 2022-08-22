use crate::wf_crypto::{
    cipher::{WfCipher, WhiteflagCipher},
    ecdh_keypair::WhiteflagECDHKeyPair,
    wf_encryption_key::WfEncryptionKey,
};

/**
 * Whiteflag cipher test class
 */

/**
 * Tests Whiteflag encryption and decryption with pre-shared key and known test vector
 */
#[test]
fn test_cipher_1() {
    let plaintext = "23000000000088888889111111119999999a22222222aaaaaaab33333333bbbbbbbb0983098309830983118b118b118b118b1993199319931993219b219b219b219b29a329a329a329a331ab31ab31ab31a9b1b9b1b9b1b9b1b9c1c9c1c9c1c9c1c8";

    let key = WfEncryptionKey::new(
        "32676187ba7badda85ea63a69870a7133909f1999774abb2eed251073616a6e7".to_string(),
    );
    let cipher: WhiteflagCipher = WfCipher::from_key(key);

    let ciphertext = cipher.encrypt(plaintext.to_string());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
}

/**
 * Tests full Whiteflag encryption scheme and decryption with negotiated key
 */
#[test]
fn test_cipher_2() {
    let plaintext1 = "aa1bb2cc3dd4ee5ff6007008009000";

    let keypair1 = WhiteflagECDHKeyPair::default();
    let keypair2 = WhiteflagECDHKeyPair::default();

    let cipher1: WhiteflagCipher = keypair1.create_cipher(keypair2.as_ref());
    let ciphertext = cipher1.encrypt(plaintext1.to_string());

    let cipher2: WhiteflagCipher = keypair2.create_cipher(keypair1.as_ref());
    let plaintext2 = cipher2.decrypt(ciphertext.to_string());

    assert_eq!(plaintext1, plaintext2);
}
