use crate::Header;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Authentication {
    #[serde(flatten)]
    header: Header,
    /// e.g. 1
    verification_method: usize,
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
