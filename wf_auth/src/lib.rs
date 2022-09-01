#[cfg(test)]
mod test;

use wf_crypto::{hkdf, CryptoResult};

/// Whiteflag specifies two methods for authentication
/// (4.3.4.2 https://standard.whiteflagprotocol.org/v1/)
/// - (1) internet resource
/// - (2) pre shared token
#[derive(Clone, Copy)]
pub enum AuthenticationMethod {
    InternetResource,
    PresharedToken,
}

impl AuthenticationMethod {
    pub fn get_method_code(self) -> u8 {
        match self {
            AuthenticationMethod::InternetResource => 1,
            AuthenticationMethod::PresharedToken => 2,
        }
    }
}

/// hex = 420abc48f5d69328c457d61725d3fd7af2883cad8460976167e375b9f2c14081
const PRESHARED_SECRET: [u8; 32] = [
    66, 10, 188, 72, 245, 214, 147, 40, 196, 87, 214, 23, 37, 211, 253, 122, 242, 136, 60, 173,
    132, 96, 151, 97, 103, 227, 117, 185, 242, 193, 64, 129,
];

#[derive(Clone)]
struct WhiteflagAuthMethod {
    value: AuthenticationMethod,
    length: usize,
    hkdf_salt: Vec<u8>,
}

impl WhiteflagAuthMethod {
    pub fn new(
        value: AuthenticationMethod,
        length: usize,
        hkdf_salt: Vec<u8>,
    ) -> WhiteflagAuthMethod {
        WhiteflagAuthMethod {
            value,
            length,
            hkdf_salt,
        }
    }

    pub fn get_preshared_token() -> Self {
        WhiteflagAuthMethod {
            value: AuthenticationMethod::PresharedToken,
            length: 32,
            hkdf_salt: PRESHARED_SECRET.to_vec(),
        }
    }
}

#[derive(Clone)]
pub struct WhiteflagAuthToken {
    token: Vec<u8>,
    method: WhiteflagAuthMethod,
}

impl WhiteflagAuthToken {
    pub fn new<T: Into<Vec<u8>>>(secret: T) -> WhiteflagAuthToken {
        WhiteflagAuthToken {
            token: secret.into(),
            method: WhiteflagAuthMethod::get_preshared_token(),
        }
    }

    pub fn get_verification_data<T: AsRef<[u8]>>(&self, context: T) -> CryptoResult<String> {
        let result = hkdf(
            &self.token,
            &self.method.hkdf_salt,
            context.as_ref(),
            self.method.length,
        )?;
        Ok(hex::encode(result))
    }
}

impl AsRef<AuthenticationMethod> for WhiteflagAuthToken {
    fn as_ref(&self) -> &AuthenticationMethod {
        &self.method.value
    }
}
