use fennel_lib::{get_session_secret, get_session_public_key};
use super::ecdh_keypair::{generate_wfkeypair, generate_wfkeypair_from_key, WfECDHKeyPair};

#[test]
fn test_generate_keypair() {
    generate_wfkeypair();
}

#[test]
fn test_generate_keypair_from_key() {
    let static_secret = get_session_secret();
    let public_key = get_session_public_key(&static_secret);
    let keypair = generate_wfkeypair_from_key(static_secret);
    assert_eq!(public_key.as_bytes(), keypair.get_public_key().as_bytes());
}

#[test]
fn test_get_public_key() {}

#[test]
fn test_get_raw_public_key() {}

#[test]
fn test_negotiate_key() {}

#[test]
fn test_create_keypair() {}

#[test]
fn test_create_keypair_from_secret() {}

#[test]
fn test_create_public_key() {}

#[test]
fn test_create_private_key() {}