use crate::Header;
use serde::{Deserialize, Serialize};

/// 4.3.4.1 Authentication Message Fields
#[derive(Serialize, Deserialize)]
pub struct Authentication {
    #[serde(flatten)]
    header: Header,

    /// Indicates the authentication mechanism
    ///
    /// 4.3.4.2 Verification Method Field
    /// 1 = Internet Resource
    /// 2 = Shared Token
    verification_method: usize,

    /// Provides the data required for authentication
    ///
    /// 4.3.4.3 Verification Data Field
    /// e.g. https://organisation.int/whiteflag
    verification_data: String,
}

impl Authentication {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            verification_method: 1,
            verification_data: "https://organisation.int/whiteflag".to_string(),
        }
    }
}
