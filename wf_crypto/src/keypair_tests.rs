use super::ecdh_keypair::WhiteflagECDHKeyPair;
use fennel_lib::{get_session_public_key, get_session_secret, get_shared_secret};

fn assert_array_eq<T: PartialEq + std::fmt::Debug>(l: &[T], r: &[T], msg: Option<&str>) {
    let success = l.iter().eq(r.iter());
    if !success {
        println!("expected: {:?}\nwas: {:?}", l, r);
    }

    assert!(success, "{}", msg.unwrap_or(""));
}

#[test]
fn test_generate_keypair_from_key() {
    let static_secret = get_session_secret();
    let public_key = get_session_public_key(&static_secret);
    let keypair = WhiteflagECDHKeyPair::from_secret(static_secret);
    assert_eq!(public_key.as_bytes(), keypair.as_ref().as_bytes());
}

#[test]
fn test_negotiate_key() {
    let static_secret = get_session_secret();
    let static_secret_two = get_session_secret();

    let public_key_two = get_session_public_key(&static_secret_two);
    let shared_secret = get_shared_secret(static_secret.clone(), &public_key_two.clone());

    let pair = WhiteflagECDHKeyPair::from_secret(static_secret);
    let result = pair.negotiate(WhiteflagECDHKeyPair::from_secret(static_secret_two).as_ref());

    assert_eq!(shared_secret.to_bytes().to_vec(), result)
}

#[test]
fn test_get_keypair_with_secret() {
    let static_secret = get_session_secret();
    let pair = WhiteflagECDHKeyPair::from_secret(static_secret.clone());
    assert_eq!(
        get_session_public_key(&static_secret).to_bytes(),
        pair.as_ref().to_bytes()
    )
}

#[test]
fn negotiate_key() {
    let keypair1 = WhiteflagECDHKeyPair::default();
    let keypair2 = WhiteflagECDHKeyPair::default();

    let shared_secret1 = keypair1.negotiate(keypair2.as_ref());
    let shared_secret2 = keypair2.negotiate(keypair1.as_ref());
    assert_array_eq(
        &shared_secret1,
        &shared_secret2,
        Some("Shared secrets should be identical"),
    );
}
