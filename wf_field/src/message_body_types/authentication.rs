use crate::Header;
use serde::{Deserialize, Serialize};

/// 4.3.4.1 Authentication Message Fields
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    #[serde(flatten)]
    header: Header,

    /// Indicates the authentication mechanism
    ///
    /// 4.3.4.2 Verification Method Field
    /// 1 = Internet Resource
    /// 2 = Shared Token
    verification_method: String,

    /// Provides the data required for authentication
    ///
    /// 4.3.4.3 Verification Data Field
    /// e.g. https://organisation.int/whiteflag
    verification_data: String,
}

impl Authentication {
    /// Creates a new Authentication message with default values
    /// Default: Method 1 (URL) with placeholder URL
    pub fn new(header: Header) -> Self {
        Self {
            header,
            verification_method: "1".to_string(),
            verification_data: "https://organisation.int/whiteflag".to_string(),
        }
    }

    /// Creates a new Authentication message with Method 1 (URL Validation)
    ///
    /// Per Whiteflag spec 5.1.2.1: "The URL contained in the VerificationData
    /// field must point to web resource. This allows the blockchain account to
    /// be linked to a web site, social media account, or any other web resource
    /// that identifies the originator."
    ///
    /// # Arguments
    /// * `header` - Message header with MessageCode "A"
    /// * `url` - URL where JWS authentication object is posted
    pub fn new_with_url(header: Header, url: String) -> Self {
        Self {
            header,
            verification_method: "1".to_string(),
            verification_data: url,
        }
    }

    /// Creates a new Authentication message with Method 2 (Shared Token Validation)
    ///
    /// Per Whiteflag spec 5.1.2.2: "The secret token must not be used directly
    /// in a single A2(0) message. Instead, the authentication data sent in the
    /// A2(0) message must be derived from the secret token using the HKDF
    /// function defined in RFC 5869."
    ///
    /// # Arguments
    /// * `header` - Message header with MessageCode "A"
    /// * `token` - HKDF-derived token (hex-encoded or base64)
    pub fn new_with_token(header: Header, token: String) -> Self {
        Self {
            header,
            verification_method: "2".to_string(),
            verification_data: token,
        }
    }

    /// Sets the verification method
    ///
    /// # Arguments
    /// * `method` - "1" for URL Validation, "2" for Shared Token
    pub fn set_verification_method(&mut self, method: String) {
        self.verification_method = method;
    }

    /// Sets the verification data
    ///
    /// # Arguments
    /// * `data` - URL for Method 1, or token for Method 2
    pub fn set_verification_data(&mut self, data: String) {
        self.verification_data = data;
    }

    /// Gets the verification method
    pub fn get_verification_method(&self) -> &str {
        &self.verification_method
    }

    /// Gets the verification data
    pub fn get_verification_data(&self) -> &str {
        &self.verification_data
    }
}
