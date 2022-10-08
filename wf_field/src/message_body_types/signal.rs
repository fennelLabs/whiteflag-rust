use crate::Header;
use serde::{Deserialize, Serialize};

/// 4.3.1.1 Signs & Signals Message Fields
#[derive(Serialize, Deserialize)]
pub struct Signal {
    #[serde(flatten)]
    header: Header,
    /// Indicates the sign/signal type with the value defined in Subject Code Field
    subject_code: String,
    /// Indicates when the sign/signal is valid, using an ISO 8601/ RFC 3339 timestamp
    datetime: String,
    /// Indicates how long the sign/signal will be valid, using the ISO 8601 format
    duration: String,
    /// Specifies the type of object the sign/signal refers to
    object_type: String,
    /// Specifies the object location in decimal degrees latitude i.a.w. ISO 6709
    object_latitude: String,
    /// Specifies the object location in decimal degrees longitude i.a.w. ISO 6709
    object_longitude: String,
    /// Specifies the size of the object’s first dimension in meters
    object_size_dim1: String,
    /// Specifies the size of the object’s second dimension in meters
    object_size_dim2: String,
    /// Specifies the object’s orientation in degrees
    object_orientation: String,
}

impl Signal {
    pub fn new(header: Header) -> Self {
        Self {
            header,
            subject_code: "80".to_string(),
            datetime: "2013-08-31T04:29:15Z".to_string(),
            duration: "P00D00H00M".to_string(),
            object_type: "22".to_string(),
            object_latitude: "+30.79658".to_string(),
            object_longitude: "-037.82602".to_string(),
            object_size_dim1: "8765".to_string(),
            object_size_dim2: "3210".to_string(),
            object_orientation: "042".to_string(),
        }
    }
}
