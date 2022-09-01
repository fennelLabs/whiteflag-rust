use x25519_dalek::PublicKey;

use wf_auth::WhiteflagAuthToken;

use super::error::WhiteflagAccountResult;
use wf_crypto::{ecdh_keypair::WhiteflagECDHKeyPair, wf_encryption_key::WhiteflagEncryptionKey};

/// Ports the interface found in https://github.com/fennelLabs/whiteflag-java/blob/master/src/main/java/org/whiteflagprotocol/java/WfAccount.java
pub trait WfAccount {
    fn new(owned: bool) -> Self;

    /// Checks if this account is owned by the current user.
    fn is_owned(&self) -> bool;

    /// Gets this account's public address.
    fn set_address(&mut self, address: String);
    fn get_address(&mut self) -> Option<String>;
    fn get_binary_address(&mut self) -> Vec<u8>;

    /// Gets the authentication URL sent with an A1 message used to identify the originator associated with this account
    /// Whiteflag Specification 5.1.2.1 Method 1: URL Validation
    fn get_auth_url(&mut self) -> Option<&String>;
    fn set_auth_url(&mut self, url: String);

    /// Gets the authentication token sent with an A2 message to identify the originator associated with this account
    /// Whiteflag Specification 5.1.2.2 Method 2: Shared Token Validation
    fn get_auth_token(&mut self) -> Option<&WhiteflagAuthToken>;
    fn set_auth_token(&mut self, token: WhiteflagAuthToken);

    /// Gets the shared encryption key with this participant's account
    /// Whiteflag Specification 5.2.4 Message Encryption
    fn get_shared_key(&mut self) -> Option<&WhiteflagEncryptionKey>;
    fn set_shared_key(&mut self, key: WhiteflagEncryptionKey);

    /// Gets the own ECDH key pair used to negatiate keys with other participants
    /// Whiteflag Specification 5.2.4 Message Encryption
    fn get_ecdh_keypair(&mut self) -> Option<&WhiteflagECDHKeyPair>;
    fn set_ecdh_keypair(
        &mut self,
        ecdh_keypair: WhiteflagECDHKeyPair,
    ) -> WhiteflagAccountResult<()>;

    /// Gets the other's ECDH public key used to negatioate a key with this participant's account
    /// Whiteflag Specification 5.2.4 Message Encryption
    fn get_ecdh_public_key(&mut self) -> Option<&PublicKey>;
    fn set_ecdh_public_key(&mut self, ecdh_public_key: PublicKey) -> WhiteflagAccountResult<()>;
}
