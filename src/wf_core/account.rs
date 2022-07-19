pub trait WhiteflagAccount {
    /// Checks if this account is owned by the current user.
    pub fn is_self() -> bool;

    /// Gets this account's public address.
    pub fn get_address() -> &str;

    /// Gets the authentication URL sent with an A1 message used to identify the originator associated with this account
    /// Whiteflag Specification 5.1.2.1 Method 1: URL Validation
    pub fn get_auth_url() -> Option<&str>;

    /// Gets the authentication token sent with an A2 message to identify the originator associated with this account
    /// Whiteflag Specification 5.1.2.2 Method 2: Shared Token Validation
    pub fn get_auth_token() -> WhiteflagAuthToken;

    /// Gets the shared encryption key with this participant's account
    /// Whiteflag Specification 5.2.4 Message Encryption
    pub fn get_shared_key() -> WhiteflagEncryptionKey;

    /// Gets the own ECDH key pair used to negatiate keys with other participants
    /// Whiteflag Specification 5.2.4 Message Encryption
    pub fn get_ecdh_keypair() -> WhiteflagECDHKeyPair;

    /// Gets the other's ECDH public key used to negatioate a key with this participant's account
    /// Whiteflag Specification 5.2.4 Message Encryption
    pub fn get_ecdh_public_key() -> ECPublicKey;
}
