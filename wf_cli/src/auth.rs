use std::{
    env,
    fs::{self, File},
    path::Path,
};

use fennel_whiteflag::WhiteflagMessage;

/// Generates an authentication lock.
pub fn acquire_auth_lock() {
    // Some good, old-fashioned semaphore action.
    match File::create(Path::new(".authlock")) {
        Err(why) => panic!("couldn't create .authlock file: {}", why),
        Ok(file) => file,
    };
}

/// Removes the authentication lock.
pub fn release_auth_lock() -> bool {
    match fs::remove_file(".authlock") {
        Err(_) => false,
        Ok(_) => true,
    }
}

/// Checks whether a lockfile exists.
pub fn check_auth_lock() -> bool {
    match File::open(Path::new(".authlock")) {
        Err(_) => false,
        Ok(_) => true,
    }
}

pub struct UserAuthenticationState;

impl UserAuthenticationState {
    pub fn is_authenticated() -> bool {
        check_auth_lock()
    }

    pub fn login() -> String {
        if check_auth_lock() {
            return "already logged in".to_owned();
        }

        acquire_auth_lock();

        // Generate A(0) initial authentication message per Whiteflag spec 5.1.1
        // "Each account should be identified by sending an A(0) initial authentication
        // message, before sending any other message."

        // Check for authentication URL from environment variable or config
        let verification_method =
            env::var("WHITEFLAG_VERIFICATION_METHOD").unwrap_or_else(|_| "1".to_string());
        let verification_data = env::var("WHITEFLAG_AUTH_URL")
            .unwrap_or_else(|_| "https://organisation.int/whiteflag".to_string());

        // Generate A(0) message with ReferenceIndicator="0" (initial authentication)
        format!(
            r#"{{"prefix":"WF","version":"1","encryptionIndicator":"0","duressIndicator":"0","messageCode":"A","referenceIndicator":"0","referencedMessage":"0000000000000000000000000000000000000000000000000000000000000000","verificationMethod":"{}","verificationData":"{}"}}"#,
            verification_method, verification_data
        )
    }

    pub fn logout() -> String {
        if check_auth_lock() {
            release_auth_lock();
            return WhiteflagMessage::new_with_reference("A".to_owned(), "4".to_owned())
                .unwrap()
                .as_json();
        }

        "error: no active session to logout from".to_owned()
    }
}
