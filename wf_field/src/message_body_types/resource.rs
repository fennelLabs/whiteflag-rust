use crate::{Header, MessageCodeType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Resource {
    #[serde(flatten)]
    header: Header,
    /// Indicates the mechanism for pointing to a resource
    ///
    /// 4.3.2.2 Resource Method Field
    /// 0 = Must not be used
    /// 1 = Internet Resource
    /// 2..9 = Reserved
    /// A..D = Private, not standard
    resource_method: String,

    /// Provides the data required to find the resource
    ///
    /// 4.3.2.3 Resource Data Field
    /// If the `resource_method` method indicates a reference to
    /// an internet resource (resource method 1),
    /// then the `resource_data` field must contain a valid URL
    resource_data: String,
}

impl Resource {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            resource_method: "1".to_string(),
            resource_data: "https://organisation.int/whiteflag".to_string(),
        }
    }
}
