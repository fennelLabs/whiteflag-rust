use crate::{ecdh_keypair::WhiteflagECDHKeyPair, wf_encryption_key::WhiteflagEncryptionKey};
use aes_tools::FennelCipher;

/// Tests Whiteflag encryption and decryption with pre-shared key and known test vector
#[test]
fn test_cipher_1() {
    let plaintext = "23000000000088888889111111119999999a22222222aaaaaaab33333333bbbbbbbb0983098309830983118b118b118b118b1993199319931993219b219b219b219b29a329a329a329a331ab31ab31ab31a9b1b9b1b9b1b9b1b9c1c9c1c9c1c9c1c8";
    let key = WhiteflagEncryptionKey::from_preshared_key(
        "32676187ba7badda85ea63a69870a7133909f1999774abb2eed251073616a6e7",
    );

    let cipher = key.aes_cipher();
    let ciphertext = cipher.encrypt(plaintext);

    assert_eq!(
        plaintext,
        String::from_utf8_lossy(&cipher.decrypt(ciphertext))
    );
}

/// Tests full Whiteflag encryption scheme and decryption with negotiated key
#[test]
fn test_cipher_2() {
    let plaintext1 = "aa1bb2cc3dd4ee5ff6007008009000";

    let keypair1 = WhiteflagECDHKeyPair::default();
    let keypair2 = WhiteflagECDHKeyPair::default();

    let cipher1 = keypair1.create_aes_cipher(keypair2.as_ref());
    let ciphertext = cipher1.encrypt(plaintext1);

    let cipher2 = keypair2.create_aes_cipher(keypair1.as_ref());
    let plaintext2 = cipher2.decrypt(ciphertext);

    assert_eq!(plaintext1, String::from_utf8_lossy(&plaintext2));
}
