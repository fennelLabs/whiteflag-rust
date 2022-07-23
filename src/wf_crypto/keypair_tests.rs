use fennel_lib::{get_session_public_key, get_session_secret, get_shared_secret};

use super::ecdh_keypair::{
    generate_wfkeypair, generate_wfkeypair_from_key, WfECDHKeyPair, WhiteflagECDHKeyPair,
};

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
fn test_negotiate_key() {
    let static_secret = get_session_secret();
    let static_secret_two = get_session_secret();
    let public_key_two = get_session_public_key(&static_secret_two);
    let shared_secret = get_shared_secret(static_secret.clone(), &public_key_two.clone());
    let mut pair = generate_wfkeypair_from_key(static_secret);
    pair.negotiate_key(public_key_two.to_bytes());
    assert_eq!(shared_secret.to_bytes(), pair.get_shared_secret().unwrap().to_bytes())
}

#[test]
fn test_get_keypair_with_secret() {
    let static_secret = get_session_secret();
    let pair = WhiteflagECDHKeyPair::_get_keypair_with_secret(static_secret.clone());
    assert_eq!(get_session_public_key(&static_secret).to_bytes(), pair.get_public_key().to_bytes())
}

#[test]
fn test_create_public_key() {
    let static_secret = get_session_secret();
    let public_key = get_session_public_key(&static_secret);
    assert_eq!(
        public_key.as_bytes(),
        WhiteflagECDHKeyPair::create_public_key_from_raw(public_key.to_bytes()).as_bytes()
    );
}

#[test]
fn test_create_private_key() {
    let static_secret = get_session_secret();
    assert_eq!(
        static_secret.to_bytes(),
        WhiteflagECDHKeyPair::create_private_key_from_raw(static_secret.to_bytes()).to_bytes()
    );
}
