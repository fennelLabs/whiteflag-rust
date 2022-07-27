use self::error::AuthenticationError;
use crate::{
    wf_buffer::{constants::*, HexadecimalString},
    wf_crypto::hkdf,
};
use lazy_static::lazy_static;

mod error;
mod test;

lazy_static! {
    static ref TOKEN_PRESHARED: WhiteflagAuthMethod = WhiteflagAuthMethod {
        value: "2".to_string(),
        length: 32,
        hkdf_salt: "420abc48f5d69328c457d61725d3fd7af2883cad8460976167e375b9f2c14081".into()
    };
}

struct WhiteflagAuthMethod {
    value: String,
    length: usize,
    hkdf_salt: Vec<u8>,
}

impl WhiteflagAuthMethod {
    pub fn new(value: String, length: usize, hkdf_salt: HexadecimalString) -> WhiteflagAuthMethod {
        WhiteflagAuthMethod {
            value,
            length,
            hkdf_salt: hkdf_salt.try_into().map_err(|e| panic!("{}", e)).unwrap(),
        }
    }
}

struct WhiteflagAuthToken {
    token: Vec<u8>,
    method: WhiteflagAuthMethod,
}

impl WhiteflagAuthToken {
    pub fn new(secret: HexadecimalString) -> WhiteflagAuthToken {
        let token = secret.try_into().map_err(|e| panic!("{}", e)).unwrap();
        println!("{:?}", &token);
        WhiteflagAuthToken {
            token,
            method: WhiteflagAuthMethod {
                value: "2".into(),
                length: 32,
                hkdf_salt: "420abc48f5d69328c457d61725d3fd7af2883cad8460976167e375b9f2c14081"
                    .into(),
            }, /* method: WhiteflagAuthMethod {
                   value: TOKEN_PRESHARED.value.clone(),
                   length: TOKEN_PRESHARED.length,
                   hkdf_salt: TOKEN_PRESHARED.hkdf_salt.clone(),
               }, */
        }
    }

    pub fn get_verification_data<T: AsRef<str>>(
        &self,
        context: T,
    ) -> Result<String, AuthenticationError> {
        let info = hex::decode(context.as_ref()).map_err(|e| AuthenticationError::HexDecode(e))?;
        let result = hkdf(
            &self.token,
            &self.method.hkdf_salt,
            &info,
            self.method.length,
        )
        .map_err(|_| AuthenticationError::General("hkdf error"))?;
        Ok(hex::encode(result))
    }
}
