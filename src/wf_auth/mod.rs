use crate::{error::WhiteflagResult, wf_buffer::HexadecimalString, wf_crypto::hkdf};

mod test;

const preshared_secret: &'static str =
    "420abc48f5d69328c457d61725d3fd7af2883cad8460976167e375b9f2c14081";

struct WhiteflagAuthMethod {
    value: AuthenticationMethod,
    length: usize,
    hkdf_salt: HexadecimalString,
}

impl WhiteflagAuthMethod {
    pub fn new(
        value: AuthenticationMethod,
        length: usize,
        hkdf_salt: HexadecimalString,
    ) -> WhiteflagAuthMethod {
        WhiteflagAuthMethod {
            value,
            length,
            hkdf_salt,
        }
    }

    fn get_preshared_token() -> Self {
        WhiteflagAuthMethod {
            value: AuthenticationMethod::PRESHARED_TOKEN,
            length: 32,
            hkdf_salt: preshared_secret.to_string().into(),
        }
    }
}

struct WhiteflagAuthToken {
    token: Vec<u8>,
    method: WhiteflagAuthMethod,
}

/// Whiteflag specifies two methods for authentication
/// (4.3.4.2 https://standard.whiteflagprotocol.org/v1/)
/// - (1) internet resource
/// - (2) pre shared token
enum AuthenticationMethod {
    INTERNET_RESOURCE,
    PRESHARED_TOKEN,
}

impl AuthenticationMethod {
    pub fn get_method_code(self) -> u8 {
        match self {
            AuthenticationMethod::INTERNET_RESOURCE => 1,
            AuthenticationMethod::PRESHARED_TOKEN => 2,
        }
    }
}

impl WhiteflagAuthToken {
    pub fn new(secret: HexadecimalString) -> WhiteflagAuthToken {
        let token: Vec<u8> = secret.try_into().map_err(|e| panic!("{}", e)).unwrap();
        WhiteflagAuthToken {
            token,
            method: WhiteflagAuthMethod::get_preshared_token(),
        }
    }

    pub fn get_verification_data<T: AsRef<str>>(&self, context: T) -> WhiteflagResult<String> {
        let info = hex::decode(context.as_ref())?;
        let salt: Vec<u8> = (&self.method.hkdf_salt).try_into()?;
        let result = hkdf(&self.token, &salt, &info, self.method.length).unwrap();
        Ok(hex::encode(result))
    }
}
