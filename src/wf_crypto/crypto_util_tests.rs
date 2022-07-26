use super::crypto_util::{zeroise, SimpleWhiteflagHkdf};
use crate::wf_buffer::common::decode_from_hexadecimal;

fn assert_array_eq<T: PartialEq + std::fmt::Debug>(l: &[T], r: &[T], msg: Option<&str>) {
    let success = l.iter().eq(r.iter());
    if !success {
        println!("expected: {:?}\nwas: {:?}", l, r);
    }

    assert!(success, "{}", msg.unwrap_or(""));
}

#[test]
fn test_zeroise() {
    let mut buffer = decode_from_hexadecimal("f0f1f2f3f4f5f6f7f8f9").0;
    let zero = vec![0; buffer.len()];

    zeroise(&mut buffer);
    assert_array_eq(
        &zero,
        &buffer,
        Some("Input and output hexadecimal strings should be equal"),
    );
}

#[test]
fn test_hkdf1() {
    let ikm = hex::decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
    let salt = hex::decode("000102030405060708090a0b0c").unwrap();
    let info = hex::decode("f0f1f2f3f4f5f6f7f8f9").unwrap();
    let prk =
        hex::decode("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5").unwrap();
    let okm = hex::decode(
        "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865",
    )
    .unwrap();

    /* test extract */
    let prk_result = SimpleWhiteflagHkdf::<sha2::Sha256>::new(&ikm, &salt);
    assert_array_eq(
        &prk,
        &prk_result.as_ref(),
        Some("Should pass RFC 5869 A.1 Test Case 1 Extract"),
    );

    /* test expand */
    let hk: SimpleWhiteflagHkdf<sha2::Sha256> = prk.as_slice().try_into().unwrap();
    let okm_result = hk.expand(&info, 42).unwrap();
    assert_array_eq(
        &okm,
        &okm_result,
        Some("Should pass RFC 5869 A.1 Test Case 1 Expand"),
    );
}

#[test]
fn test_hkdf2() {
    let ikm = hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f").unwrap();
    let salt = hex::decode("606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeaf").unwrap();
    let info = hex::decode("b0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff").unwrap();
    let prk =
        hex::decode("06a6b88c5853361a06104c9ceb35b45cef760014904671014a193f40c15fc244").unwrap();
    let okm = hex::decode("b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71cc30c58179ec3e87c14c01d5c1f3434f1d87").unwrap();

    /* test extract */

    let prk_result = SimpleWhiteflagHkdf::<sha2::Sha256>::new(&ikm, &salt);
    assert_array_eq(
        &prk,
        &prk_result.as_ref(),
        Some("Should pass RFC 5869 A.2 Test Case 2 Extract"),
    );

    /* test expand */
    let hk: SimpleWhiteflagHkdf<sha2::Sha256> = prk.as_slice().try_into().unwrap();
    let okm_result = hk.expand(&info, 82).unwrap();
    assert_array_eq(
        &okm,
        &okm_result,
        Some("Should pass RFC 5869 A.2 Test Case 2 Expand"),
    );
}
