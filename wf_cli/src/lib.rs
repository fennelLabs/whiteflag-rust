mod auth;
mod error;

use crate::auth::UserAuthenticationState;
use wf_field::Header;

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

    pub fn message(code: String) -> WhiteflagCLIResult<String> {
        if UserAuthenticationState::is_authenticated() == false {
            Ok("error: must authenticate using `wf auth`".to_string())
        } else {
            let header = Header::new(code);
            let body = header.to_body();
            Ok(body.to_string()?)
        }
    }
}
