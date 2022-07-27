use super::WhiteflagAuthToken;

#[test]
fn test_auth_token() {
    /* Setup */
    let secret = "000102030405060708090a0b0c";
    let context = "6fdb25dc394d5a437d88f15b459406ac6db8b386a49dbfc38c";
    let verification_data = "a951cb35881ee7f78b05f8476a2193de4556455d48ffcfebcfc8938f4a37a70f";
    let token = WhiteflagAuthToken::new(secret.into());

    /* Verify */
    //assertEquals("Authentication token should have the correct authentication indicator", TOKEN_PRESHARED.fieldValue, token.method.fieldValue);
    assert_eq!(
        verification_data,
        token.get_verification_data(context).unwrap(),
        "Authentication token should give the correct verification data"
    );
}
