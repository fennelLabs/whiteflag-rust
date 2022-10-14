mod auth;
mod error;

use crate::auth::UserAuthenticationState;
pub use fennel_whiteflag::WhiteflagMessage;

pub struct WhiteflagCLICommands;
pub type WhiteflagCLIResult<T> = Result<T, error::WhiteflagCLIError>;

impl WhiteflagCLICommands {
    pub fn encode<T: AsRef<str>>(json: T) -> WhiteflagCLIResult<String> {
        Ok(fennel_whiteflag::encode_from_json(json)?)
    }

    pub fn decode<T: AsRef<str>>(hex: T) -> WhiteflagCLIResult<String> {
        Ok(fennel_whiteflag::decode_from_hex(hex)?)
    }

    pub fn auth(logout: bool) -> WhiteflagCLIResult<String> {
        Ok(if logout {
            UserAuthenticationState::logout()
        } else {
            UserAuthenticationState::login()
        }
        .to_string())
    }

    pub fn message(code: String) -> WhiteflagCLIResult<WhiteflagMessage> {
        if !UserAuthenticationState::is_authenticated() {
            Err(error::WhiteflagCLIError::AuthenticationRequired)
        } else {
            Ok(WhiteflagMessage::new(code)?)
        }
    }

    pub fn message_with_reference(code: String, reference_code: String) -> WhiteflagCLIResult<WhiteflagMessage> {
        if !UserAuthenticationState::is_authenticated() {
            Err(error::WhiteflagCLIError::AuthenticationRequired)
        } else {
            Ok(WhiteflagMessage::new_with_reference(code, reference_code)?)
        }
    }
}
