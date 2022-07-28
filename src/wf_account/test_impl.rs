struct WhiteflagAccount {
    owned: bool,
    address: WhiteflagBuffer,
    auth_url: String,
    auth_token: WhiteflagAuthToken,
    ecdh_keypair: WfECDHKeyPair,
    ecdh_public_key: PublicKey,
    shared_key: WhiteflagEncryptionKey,
}

impl WfAccount for WhiteflagAccount {
    fn new(owned: bool) -> Self {
        WhiteflagAccount { owned: owned }
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn setAddress(&self, address: String) {
        self.address = WfBinaryBuffer.fromHexString(address);
    }

    fn getAddress(&self) -> String {
        if (self.address == null) {
            null
        }
        self.address.toHexString()
    }

    fn getBinaryAddress(&self) -> Vec<u8> {
        if (self.address == null) {
            vec![0] as Vec<u8>
        }
        self.address.toByteArray()
    }

    fn setAuthURL(&self, url: String) {
        self.authURL = url;
    }

    fn getAuthURL(&self) -> String {
        self.authURL;
    }

    fn setAuthToken(&self, token: WhiteflagAuthToken) {
        self.authToken = token;
    }

    fn getAuthToken(&self) -> WhiteflagAuthToken {
        self.authToken;
    }

    fn setSharedKey(&self, key: WfEncryptionKey) {
        self.sharedKey = key;
    }

    fn getSharedKey(&self) -> WfEncryptionKey {
        self.sharedKey;
    }

    fn setEcdhKeyPair(&self, ecdhKeyPair: WfECDHKeyPair) -> WhiteflagAccountResult<()> {
        if (!self.owned) {
            Err(WhiteflagAccountError::CantSetECDHPair)
        } else {
            self.ecdhKeyPair = ecdhKeyPair;
            self.ecdhPublicKey = ecdhKeyPair.getPublicKey();
            Ok(())
        }
    }

    fn getEcdhKeyPair(&self) -> WfECDHKeyPair {
        self.ecdhKeyPair;
    }

    fn setEcdhPublicKey(&self, ecdhPublicKey: PublicKey) -> WhiteflagAccountResult<()> {
        if (self.owned) {
            Err(WhiteflagAccountError::CantSetOwnECDHKey)
        } else {
            self.ecdhPublicKey = ecdhPublicKey;
            Ok(())
        }
    }

    fn getEcdhPublicKey(&self) -> PublicKey {
        self.ecdhPublicKey;
    }
}
