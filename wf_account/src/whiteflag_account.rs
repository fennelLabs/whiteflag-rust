use super::{
    account::WfAccount,
    error::{WhiteflagAccountError, WhiteflagAccountResult},
};
use wf_auth::WhiteflagAuthToken;
use wf_crypto::{ecdh_keypair::WhiteflagECDHKeyPair, wf_encryption_key::WhiteflagEncryptionKey};

#[derive(Clone)]
pub struct WhiteflagAccount {
    owned: bool,
    address: Option<Vec<u8>>,
    auth_url: Option<Vec<u8>>,
    auth_token: Option<WhiteflagAuthToken>,
    ecdh_keypair: Option<WhiteflagECDHKeyPair>,
    ecdh_public_key: Option<Vec<u8>>, // SEC1 compressed public key (33 bytes)
    shared_key: Option<WhiteflagEncryptionKey>,
}

impl WfAccount for WhiteflagAccount {
    fn new(owned: bool) -> Self {
        WhiteflagAccount {
            owned,
            address: None,
            auth_url: None,
            auth_token: None,
            ecdh_keypair: None,
            ecdh_public_key: None,
            shared_key: None,
        }
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn set_address(&mut self, address: Vec<u8>) {
        self.address = Some(address);
    }

    fn get_address(&mut self) -> Option<&Vec<u8>> {
        if self.address.is_none() {
            None
        } else {
            self.address.as_ref()
        }
    }

    fn get_auth_url(&mut self) -> Option<&Vec<u8>> {
        self.auth_url.as_ref()
    }

    fn set_auth_url(&mut self, url: Vec<u8>) {
        self.auth_url = Some(url);
    }

    fn get_auth_token(&mut self) -> Option<&WhiteflagAuthToken> {
        self.auth_token.as_ref()
    }

    fn set_auth_token(&mut self, token: WhiteflagAuthToken) {
        self.auth_token = Some(token)
    }

    fn get_shared_key(&mut self) -> Option<&WhiteflagEncryptionKey> {
        self.shared_key.as_ref()
    }

    fn set_shared_key(&mut self, key: WhiteflagEncryptionKey) {
        self.shared_key = Some(key);
    }

    fn get_ecdh_keypair(&mut self) -> Option<&WhiteflagECDHKeyPair> {
        self.ecdh_keypair.as_ref()
    }

    fn set_ecdh_keypair(
        &mut self,
        ecdh_keypair: WhiteflagECDHKeyPair,
    ) -> WhiteflagAccountResult<()> {
        if !self.owned {
            Err(WhiteflagAccountError::CantSetECDHPair)
        } else {
            self.ecdh_keypair = Some(ecdh_keypair.clone());
            self.ecdh_public_key = Some(ecdh_keypair.public_key_compressed());
            Ok(())
        }
    }

    fn get_ecdh_public_key(&mut self) -> Option<&[u8]> {
        self.ecdh_public_key.as_ref().map(|v| v.as_slice())
    }

    fn set_ecdh_public_key(&mut self, ecdh_public_key: Vec<u8>) -> WhiteflagAccountResult<()> {
        if self.owned {
            Err(WhiteflagAccountError::CantSetOwnECDHKey)
        } else {
            self.ecdh_public_key = Some(ecdh_public_key);
            Ok(())
        }
    }
}

/// Additional methods for Whiteflag A(0) initial authentication
impl WhiteflagAccount {
    /// Generates an A(0) initial authentication message JSON for this account
    /// 
    /// Per Whiteflag spec 5.1.1: "Each account should be identified by sending 
    /// an A(0) initial authentication message, before sending any other message."
    /// 
    /// This creates a JSON string representing the A(0) authentication message.
    /// The message will use the account's auth_url (Method 1) or auth_token (Method 2)
    /// if configured, otherwise it will use a default placeholder URL.
    /// 
    /// # Returns
    /// - JSON string with verification data from the account
    /// 
    /// # Example
    /// ```
    /// let mut account = WhiteflagAccount::new(true);
    /// account.set_auth_url(b"https://myorg.org/whiteflag.json".to_vec());
    /// let a0_json = account.generate_initial_authentication_json();
    /// // Returns: {"prefix":"WF","version":"1",...,"verificationMethod":"1","verificationData":"https://myorg.org/whiteflag.json"}
    /// ```
    pub fn generate_initial_authentication_json(&self) -> String {
        // Determine verification method and data based on what's available
        let (method, data) = if let Some(url) = &self.auth_url {
            // Method 1: URL Validation
            let url_str = String::from_utf8_lossy(url).to_string();
            ("1", url_str)
        } else if let Some(_token) = &self.auth_token {
            // Method 2: Shared Token
            // Note: In full implementation, we'd extract verification data from token
            // For now, using placeholder - this should call token.get_verification_data()
            ("2", "token_placeholder".to_string())
        } else {
            // No authentication data set - use default URL
            ("1", "https://organisation.int/whiteflag".to_string())
        };
        
        // Create A(0) message JSON manually
        // ReferenceIndicator = "0" for initial authentication per spec 5.1.1
        format!(
            r#"{{"prefix":"WF","version":"1","encryptionIndicator":"0","duressIndicator":"0","messageCode":"A","referenceIndicator":"0","referencedMessage":"0000000000000000000000000000000000000000000000000000000000000000","verificationMethod":"{}","verificationData":"{}"}}"#,
            method, data
        )
    }
    
    /// Checks if this account has authentication data configured
    /// 
    /// # Returns
    /// - `true` if auth_url or auth_token is set
    /// - `false` if neither is configured
    pub fn has_authentication_data(&self) -> bool {
        self.auth_url.is_some() || self.auth_token.is_some()
    }
}
