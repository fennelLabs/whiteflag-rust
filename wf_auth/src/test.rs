use super::{AuthenticationMethod, WhiteflagAuthToken};

#[test]
fn test_auth_token() {
    /* Setup */
    let secret = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let context = vec![
        111, 219, 37, 220, 57, 77, 90, 67, 125, 136, 241, 91, 69, 148, 6, 172, 109, 184, 179, 134,
        164, 157, 191, 195, 140,
    ];
    let verification_data = vec![
        169, 81, 203, 53, 136, 30, 231, 247, 139, 5, 248, 71, 106, 33, 147, 222, 69, 86, 69, 93,
        72, 255, 207, 235, 207, 200, 147, 143, 74, 55, 167, 15,
    ];
    let token = WhiteflagAuthToken::new(secret);

    /* Verify */
    assert_eq!(
        AuthenticationMethod::PresharedToken.get_method_code(),
        token.as_ref().get_method_code(),
        "Authentication token should have the correct authentication indicator"
    );
    assert_eq!(
        verification_data,
        token.get_verification_data(context).unwrap(),
        "Authentication token should give the correct verification data"
    );
}
