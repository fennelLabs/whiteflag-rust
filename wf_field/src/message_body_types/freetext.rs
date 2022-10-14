use crate::Header;
use serde::{Deserialize, Serialize};

/// 4.3.3.1 Free Text Message Fields
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeText {
    #[serde(flatten)]
    header: Header,
    text: String,
}

impl FreeText {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            text: "Hello World".to_string(),
        }
    }
}
