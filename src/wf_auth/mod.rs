mod test;

use crate::wf_crypto::hkdf;

/// Whiteflag specifies two methods for authentication
/// (4.3.4.2 https://standard.whiteflagprotocol.org/v1/)
/// - (1) internet resource
/// - (2) pre shared token
pub enum AuthenticationMethod {
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

/// hex = 420abc48f5d69328c457d61725d3fd7af2883cad8460976167e375b9f2c14081
const preshared_secret: [u8; 32] = [
    66, 10, 188, 72, 245, 214, 147, 40, 196, 87, 214, 23, 37, 211, 253, 122, 242, 136, 60, 173,
    132, 96, 151, 97, 103, 227, 117, 185, 242, 193, 64, 129,
];

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
            value: AuthenticationMethod::PRESHARED_TOKEN,
            length: 32,
            hkdf_salt: preshared_secret.to_vec(),
        }
    }
}

struct WhiteflagAuthToken {
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

    pub fn get_verification_data<T: AsRef<[u8]>>(&self, context: T) -> String {
        let result = hkdf(
            &self.token,
            &self.method.hkdf_salt,
            context.as_ref(),
            self.method.length,
        )
        .unwrap();
        hex::encode(result)
    }
}
