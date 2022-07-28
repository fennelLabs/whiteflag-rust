use x25519_dalek::PublicKey;

use crate::{wf_crypto::{wf_encryption_key::WhiteflagEncryptionKey, ecdh_keypair::WhiteflagECDHKeyPair}, wf_auth::WhiteflagAuthToken};

/// Ports the interface found in https://github.com/fennelLabs/whiteflag-java/blob/master/src/main/java/org/whiteflagprotocol/java/WfAccount.java
trait WfAccount {
    fn new(owned: bool) -> Self;

    /// Checks if this account is owned by the current user.
    fn is_owned() -> bool;

    /// Gets this account's public address.
    fn get_address() -> String;

    /// Gets the authentication URL sent with an A1 message used to identify the originator associated with this account
    /// Whiteflag Specification 5.1.2.1 Method 1: URL Validation
    fn get_auth_url() -> Option<String>;

    /// Gets the authentication token sent with an A2 message to identify the originator associated with this account
    /// Whiteflag Specification 5.1.2.2 Method 2: Shared Token Validation
    fn get_auth_token() -> WhiteflagAuthToken;

    /// Gets the shared encryption key with this participant's account
    /// Whiteflag Specification 5.2.4 Message Encryption
    fn get_shared_key() -> WhiteflagEncryptionKey;

    /// Gets the own ECDH key pair used to negatiate keys with other participants
    /// Whiteflag Specification 5.2.4 Message Encryption
    fn get_ecdh_keypair() -> WhiteflagECDHKeyPair;

    /// Gets the other's ECDH public key used to negatioate a key with this participant's account
    /// Whiteflag Specification 5.2.4 Message Encryption
    fn get_ecdh_public_key() -> PublicKey;
}