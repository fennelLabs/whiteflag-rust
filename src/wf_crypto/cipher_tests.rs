use crate::wf_crypto::{
    cipher::{WfCipher, WhiteflagCipher},
    ecdh_keypair::{generate_wfkeypair, WfECDHKeyPair},
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
    let keypair1 = generate_wfkeypair();
    let pubkey1 = hex::encode(keypair1.get_raw_public_key());

    let keypair2 = generate_wfkeypair();
    let pubkey2 = hex::encode(keypair2.get_raw_public_key());

    let key1 = WfEncryptionKey::new_key_from_ecdh_key(pubkey2, keypair1);
    let cipher1: WhiteflagCipher = WfCipher::from_key(key1);
    let ciphertext = cipher1.encrypt(plaintext1.to_string());

    let key2 = WfEncryptionKey::new_key_from_ecdh_key(pubkey1, keypair2);
    let cipher2: WhiteflagCipher = WfCipher::from_key(key2);
    let plaintext2 = cipher2.decrypt(ciphertext.to_string());

    assert_eq!(plaintext1, plaintext2);
}
