use fennel_lib::{get_session_public_key, get_session_secret, get_shared_secret};

use super::ecdh_keypair::{
    generate_wfkeypair, generate_wfkeypair_from_key, WfECDHKeyPair, WhiteflagECDHKeyPair,
};

fn assert_array_eq<T: PartialEq + std::fmt::Debug>(l: &[T], r: &[T], msg: Option<&str>) {
    let success = l.iter().eq(r.iter());
    if !success {
        println!("expected: {:?}\nwas: {:?}", l, r);
    }

    assert!(success, "{}", msg.unwrap_or(""));
}

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
    let result = pair.negotiate_key_from_bytes(public_key_two.to_bytes());
    assert_eq!(shared_secret.to_bytes().to_vec(), result)
}

#[test]
fn test_get_keypair_with_secret() {
    let static_secret = get_session_secret();
    let pair = WhiteflagECDHKeyPair::_get_keypair_with_secret(static_secret.clone());
    assert_eq!(
        get_session_public_key(&static_secret).to_bytes(),
        pair.get_public_key().to_bytes()
    )
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

#[test]
fn test_negotiate_key_one() {
    for _ in 0..5 {
        let mut keypair1 = generate_wfkeypair();
        let mut keypair2 = generate_wfkeypair();
        let pubkey1 = keypair1.get_raw_public_key();
        let pubkey2 = keypair2.get_raw_public_key();

        let shared_secret1 = keypair1.negotiate_key_from_bytes(pubkey2);
        let shared_secret2 = keypair2.negotiate_key_from_bytes(pubkey1);
        assert_array_eq(
            &shared_secret1,
            &shared_secret2,
            Some("Shared secrets should be indentical"),
        );
    }
}

#[test]
fn test_negotiate_key_two() {
    let mut keypair1 = generate_wfkeypair();
    let mut keypair2 = generate_wfkeypair();
    let pubkey1 = keypair1.get_public_key();
    let pubkey2 = keypair2.get_public_key();

    let shared_secret1 = keypair1.negotiate_key(pubkey2);
    let shared_secret2 = keypair2.negotiate_key(pubkey1);
    assert_array_eq(
        &shared_secret1,
        &shared_secret2,
        Some("Shared secrets should be indentical"),
    );
}
