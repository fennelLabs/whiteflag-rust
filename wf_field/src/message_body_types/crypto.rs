use crate::Header;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crypto {
    #[serde(flatten)]
    header: Header,

    /// Indicates the type of data in this message
    ///
    /// 4.3.5.1 Cryptographic Support Message Fields
    crypto_data_type: String,

    /// Contains the cryptographic data
    ///
    /// 4.3.5.2 Cryptographic Data Type Field
    crypto_data: String,
}

impl Crypto {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            crypto_data_type: "01".to_string(),
            crypto_data: "HDExtPubKey".to_string(),
        }
    }
}
