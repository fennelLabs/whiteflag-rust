use super::{
    account::WfAccount,
    error::{WhiteflagAccountError, WhiteflagAccountResult},
};
use wf_auth::WhiteflagAuthToken;
use wf_crypto::{ecdh_keypair::WhiteflagECDHKeyPair, wf_encryption_key::WhiteflagEncryptionKey};
use x25519_dalek::PublicKey;

#[derive(Clone)]
pub struct WhiteflagAccount {
    owned: bool,
    address: Option<Vec<u8>>,
    auth_url: Option<String>,
    auth_token: Option<WhiteflagAuthToken>,
    ecdh_keypair: Option<WhiteflagECDHKeyPair>,
    ecdh_public_key: Option<PublicKey>,
    shared_key: Option<WhiteflagEncryptionKey>,
}

impl WfAccount for WhiteflagAccount {
    fn new(owned: bool) -> Self {
        WhiteflagAccount {
            owned: owned,
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

    fn set_address(&mut self, address: String) {
        self.address = Some(hex::decode(address).unwrap());
    }

    fn get_address(&mut self) -> Option<String> {
        if self.address.is_none() {
            None
        } else {
            Some(hex::encode(self.address.as_ref().unwrap()))
        }
    }

    fn get_binary_address(&mut self) -> Vec<u8> {
        if self.address.is_none() {
            vec![0 as u8]
        } else {
            self.address.as_ref().unwrap().to_vec()
        }
    }

    fn get_auth_url(&mut self) -> Option<&String> {
        self.auth_url.as_ref()
    }

    fn set_auth_url(&mut self, url: String) {
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
            self.ecdh_public_key = Some(ecdh_keypair.as_ref().clone());
            Ok(())
        }
    }

    fn get_ecdh_public_key(&mut self) -> Option<&PublicKey> {
        self.ecdh_public_key.as_ref()
    }

    fn set_ecdh_public_key(&mut self, ecdh_public_key: PublicKey) -> WhiteflagAccountResult<()> {
        if self.owned {
            Err(WhiteflagAccountError::CantSetOwnECDHKey)
        } else {
            self.ecdh_public_key = Some(ecdh_public_key);
            Ok(())
        }
    }
}
