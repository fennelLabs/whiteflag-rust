use std::collections::HashMap;

use crate::{wf_account::test_impl::WhiteflagAccount, wf_buffer::WhiteflagBuffer};

use super::basic_message::BasicMessage;

    const METAKEY_ORIGINATOR: &str = "originatorAddress";
    const METAKEY_RECIPIENT: &str = "recipientAddress";
    const FIELD_ENCRYPTIONINDICATOR: &str = "EncryptionIndicator";

pub struct WhiteflagMessage {
    base: BasicMessage,

    metadata: HashMap<String, String>,

    originator: Option<WhiteflagAccount>,
    recipient: Option<WhiteflagAccount>,

    initVector: Vec<u8>,

    cachedMsg: Option<WhiteflagBuffer>,
    cachedMsgStr: Option<String>,
}

impl WhiteflagMessage {    
    fn new(base: BasicMessage) -> WhiteflagMessage {
        WhiteflagMessage { base: base, metadata: HashMap::new(), originator: None, recipient: None, initVector: vec![], cachedMsg: None, cachedMsgStr: None }
    }

    fn new_from_encoded_message(base: BasicMessage, encodedMsg: WhiteflagBuffer) -> WhiteflagMessage {
        WhiteflagMessage { base: base, metadata: HashMap::new(), originator: None, recipient: None, initVector: vec![], cachedMsg: Some(encodedMsg), cachedMsgStr: None }
    }

    fn new_from_serialized_message(base: BasicMessage, serializedMsg: String) -> WhiteflagMessage {
        WhiteflagMessage { base: base, metadata: HashMap::new(), originator: None, recipient: None, initVector: vec![], cachedMsg: None, cachedMsgStr: Some(serializedMsg) }
    }

    fn create(messageCode: String) -> WhiteflagMessage {
        let base = WhiteflagMessageCreator::new().type(WhiteflagMessageType.fromCode(messageCode)).create();
        WhiteflagMessage { base: base, metadata: HashMap::new(), originator: None, recipient: None, initVector: vec![], cachedMsg: None, cachedMsgStr: None }
    }

    fn deserialize(serializedMsg: String) -> WhiteflagMessage {
        let base = WhiteflagMessageCreator::new().deserialize(serializedMsg).create();
        WhiteflagMessage { base: base, metadata: HashMap::new(), originator: None, recipient: None, initVector: vec![], cachedMsg: None, cachedMsgStr: Some(serializedMsg) }
    }

    fn deserializeJson(jsonMessage: String) -> WhiteflagMessage {
        let jsonMsg = WfJsonMessage.create(jsonMessage);
            let base = WhiteflagMessageCreator::new().map(jsonMsg.getHeader(), jsonMsg.getBody()).create();
        let message = WhiteflagMessage::new(base);
        message.setMetadata(jsonMsg.getMetadata());
        return message;
    }

    fn decode(String hexMessage) -> WhiteflagMessage {
        return decode(WhiteflagBuffer.fromHexString(hexMessage));
    }

    fn decode(byte[] binMessage) -> WhiteflagMessage {
        return decode(WhiteflagBuffer.fromByteArray(binMessage));
    }

    fn decode(WhiteflagBuffer encodedMsg) -> WhiteflagMessage {
        BasicMessage base;
        try {
            base = new WhiteflagMessageCreator().decode(encodedMsg).create();
        } catch (WfCoreException e) {
            throw new WfException("Cannot decode message: " + e.getMessage(), e, WF_FORMAT_ERROR);
        }
        return new WhiteflagMessage(base, encodedMsg);
    }

    fn decrypt(String encryptedMsg, WfAccount originator, WfAccount recipient, String initVector) -> WhiteflagMessage {
        return decrypt(WhiteflagBuffer.fromHexString(encryptedMsg), originator, recipient, WhiteflagBuffer.convertToByteArray(initVector));
    }

    fn decrypt(byte[] encryptedMsg, WfAccount originator, WfAccount recipient, byte[] initVector -> WhiteflagMessage) {
        return decrypt(WhiteflagBuffer.fromByteArray(encryptedMsg), originator, recipient, initVector);
    }

    WhiteflagMessage decrypt(WhiteflagBuffer encryptedMsg, WfAccount originator, WfAccount recipient, byte[] initVector) {
        WhiteflagMessageSegment header;
        WhiteflagMessageCreator creator = new WhiteflagMessageCreator();
        try {
            header = creator.getUnencryptedHeader(encryptedMsg);
        } catch (WfCoreException e) {
            throw new WfException("Cannot decode unencrypted message header: " + e.getMessage(), e, WF_FORMAT_ERROR);
        }
        WhiteflagBuffer encodedMsg = decrypt(encryptedMsg, header, originator, recipient, initVector);

        BasicMessage base;
        try {
            base = creator.decode(encodedMsg).create();
        } catch (WfCoreException e) {
            throw new WfException("Cannot decode message: " + e.getMessage(), e, WF_FORMAT_ERROR);
        }
        WhiteflagMessage message = new WhiteflagMessage(base, encryptedMsg);
        message.setOriginator(originator);
        message.setRecipient(recipient);
        message.setInitVector(initVector);
        return message;
    }

    WhiteflagMessage compile(String[] fieldValues) {
        BasicMessage base;
        try {
            base = new WhiteflagMessageCreator().compile(fieldValues).create();
        } catch (WfCoreException e) {
            throw new WfException("Cannot compile message: " + e.getMessage(), e, WF_FORMAT_ERROR);
        }
        return new WhiteflagMessage(base);
    }

    WhiteflagMessageType getType() {
        return base.type;
    }

    boolean isValid() {
        return base.isValid();
    }

    boolean isValid(String fieldname) {
        return base.isValid(fieldname);
    }

    boolean isValid(String fieldname, String data) {
        return base.isValid(fieldname, data);
    }

    int getNoFields() {
        return base.getNoFields();
    }

    Set<String> getFieldNames() {
        return base.getFieldNames();
    }

    String get(String fieldname) {
        return base.get(fieldname);
    }

    boolean set(String fieldname, String data) {
        return base.set(fieldname, data);
    }

    String addMetadata(String key, String value) {
        return metadata.putIfAbsent(key, value);
    }

    String getMetadata(String key) {
        return metadata.get(key);
    }

    Set<String> getMetadataKeys() {
        return metadata.keySet();
    }

    WhiteflagMessage copy() {
        return new WhiteflagMessage(this.base);
    }

    String setOriginator(WfAccount originator) {
        this.originator = originator;
        return metadata.put(METAKEY_ORIGINATOR, originator.getAddress());
    }

    WfAccount getOriginator() {
        return this.originator;
    }

    String setRecipient(WfAccount recipient) {
        this.recipient = recipient;
        return metadata.put(METAKEY_RECIPIENT, recipient.getAddress());
    }

    WfAccount getRecipient() {
        return this.recipient;
    }

    byte[] setInitVector(String initVector) {
        if (this.initVector.length == 0) {
            this.initVector = WfCryptoUtil.convertToByteArray(initVector);
        }
        return this.initVector;
    }

    byte[] setInitVector(byte[] initVector) {
        if (this.initVector.length == 0) {
            this.initVector = Arrays.copyOf(initVector, initVector.length);
        }
        return this.initVector;
    }

    byte[] getInitVector() {
        return Arrays.copyOf(this.initVector, this.initVector.length);
    }

    String toString() {
        return base.toString(); 
    }

    String serialize() {
        if (this.cachedMsgStr == null) {
            try {
                this.cachedMsgStr = base.serialize();
            } catch (WfCoreException e) {
                throw new WfException("Could not serialize message: " + e.getMessage(), e, WF_FORMAT_ERROR);
            }
        }
        return this.cachedMsgStr;
    }

    WhiteflagBuffer encode() {
        if (Boolean.TRUE.equals(cachedMsg.isComplete())) return this.cachedMsg.copy();

        WhiteflagBuffer encodedMsg;
        try {
            encodedMsg = base.encode();
        } catch (WfCoreException e) {
            throw new WfException("Could not encode message: " + e.getMessage(), e, WF_FORMAT_ERROR);
        }
        this.cachedMsg = encrypt(encodedMsg).markComplete();
        return this.cachedMsg.copy();
    }

    WhiteflagBuffer encrypt() {
        return this.encode();
    }

    byte[] toByteArray() {
        return this.encode().toByteArray();
    }

    String toHexString() {
        return this.encode().toHexString();
    }

    String toJson() {
        String jsonMsgStr;
        try {
            jsonMsgStr = new WfJsonMessage(metadata, base.header.toMap(), base.body.toMap()).toJson();
        } catch (WfUtilException e) {
            throw new WfException("Could not serialize message into JSON string: " + e.getMessage(), e, WF_FORMAT_ERROR);
        }
        return jsonMsgStr;
    }

    protected void setMetadata(Map<String, String> metadata) {
        metadata.forEach(this.metadata::put);
    }

    WhiteflagBuffer encrypt(WhiteflagBuffer encodedMsg) {
        WfEncryptionMethod method = getEncryptionMethod(base.header.get(FIELD_ENCRYPTIONINDICATOR));
        if (method == WfEncryptionMethod.NO_ENCRYPTION) return encodedMsg;

        if (recipient == null) throw new IllegalStateException("Cannot determine encryption key if recipient is unknown");
        if (originator == null) throw new IllegalStateException("Cannot set context if originator is unknown");

        WfCipher cipher = createCipher(method, originator, recipient);
        if (this.initVector.length == 0) {
            try {
                this.initVector = setInitVector(cipher.setInitVector());
            } catch (WfCryptoException e) {
                throw new WfException("Could not create random initialisation vector: " + e.getMessage(), e, WF_CRYPTO_ERROR);
            }
        } else {
            cipher.setInitVector(this.initVector);
        }
        int unencryptedBitPosition = base.header.bitLength(FIELD_ENCRYPTIONINDICATOR);
        WhiteflagBuffer encryptedMsg = WhiteflagBuffer.create();
        try {
            encryptedMsg.appendBits(encodedMsg.extractBits(0, unencryptedBitPosition));
            encryptedMsg.appendBits(cipher.encrypt(encodedMsg.extractBits(unencryptedBitPosition)));
            cipher.destroy();
        } catch (WfCryptoException e) {
            throw new WfException("Could not encrypt message: " + e.getMessage(), e, WF_CRYPTO_ERROR);
        } catch (DestroyFailedException e) {
            throw new WfException("Could not destroy the cipher: " + e.getMessage(), e, WF_CRYPTO_ERROR);
        }
        return encryptedMsg;
    }

    WhiteflagBuffer decrypt(WhiteflagBuffer encryptedMsg, WhiteflagMessageSegment header, WfAccount originator, WfAccount recipient, byte[] initVector) {
        WfEncryptionMethod method = getEncryptionMethod(header.get(FIELD_ENCRYPTIONINDICATOR));
        if (method == WfEncryptionMethod.NO_ENCRYPTION) return encryptedMsg;

        WfCipher cipher = createCipher(method, originator, recipient);
        cipher.setInitVector(initVector);

        int unencryptedBitPosition = header.bitLength(FIELD_ENCRYPTIONINDICATOR);
        WhiteflagBuffer encodedMsg = WhiteflagBuffer.create();
        try {
            encodedMsg.appendBits(encryptedMsg.extractBits(0, unencryptedBitPosition));
            encodedMsg.appendBits(cipher.decrypt(encryptedMsg.extractBits(unencryptedBitPosition)));
            cipher.destroy();
        } catch (WfCryptoException e) {
            throw new WfException("Could not decrypt message: " + e.getMessage(), e, WF_CRYPTO_ERROR);
        } catch (DestroyFailedException e) {
            throw new WfException("Could not destroy the cipher: " + e.getMessage(), e, WF_CRYPTO_ERROR);
        }
        return encodedMsg;
    }

    WfEncryptionMethod getEncryptionMethod(String encryptionIndicator) {
        if (encryptionIndicator == null || encryptionIndicator.equals("")) {
            throw new WfException("The " + FIELD_ENCRYPTIONINDICATOR + " message field does not exist or is not set", null, WF_FORMAT_ERROR);
        }
        try {
            return WfEncryptionMethod.fromFieldValue(encryptionIndicator);
        } catch (WfCryptoException e) {
            throw new WfException("No valid encryption method associated with the " + FIELD_ENCRYPTIONINDICATOR + " message field value: " + encryptionIndicator, e, WF_FORMAT_ERROR);
        }
    }

    WfCipher createCipher(WfEncryptionMethod method, WfAccount originator, WfAccount recipient) {
        WfCipher cipher;
        try {
            WfEncryptionKey key = getEncryptionKey(method, originator, recipient);
            cipher = WfCipher.fromKey(key);
        } catch (WfCryptoException e) {
            throw new WfException("Could not initialize cipher to encrypt message: " + e.getMessage(), e, WF_CRYPTO_ERROR);
        }
        byte[] address = originator.getBinaryAddress();
        if (address.length == 0) {
            throw new WfException("No originator blockchain address available", null, WF_METADATA_ERROR);
        }
        return cipher.setContext(address);
    }

    WfEncryptionKey getEncryptionKey(WfEncryptionMethod method, WfAccount originator, WfAccount recipient) {
        switch (method) {
            case AES_256_CTR_ECDH:
                return generateNegotiatedKey(originator, recipient);
            case AES_256_CTR_PSK:
                return getSharedKey(recipient);
            default:
                throw new WfException("Cannot retrieve encryption key for encryption method " + method.fieldValue + "(" + method.cipherName + ")", null, WF_CRYPTO_ERROR);
        }
    }

    WfEncryptionKey getSharedKey(WfAccount recipient) {
        WfEncryptionKey key = recipient.getSharedKey();
        if (key == null) throw new WfException("Missing pre-shared key with recipient", null, WF_METADATA_ERROR);
        return key;
    }

    WfEncryptionKey generateNegotiatedKey(WfAccount originator, WfAccount recipient) {
        WfECDHKeyPair ecdhKeypair;
        ECPublicKey ecdhPublicKey;
        if (originator.isSelf()) {
            ecdhKeypair = originator.getEcdhKeyPair();
            if (ecdhKeypair == null) throw new WfException("Missing own ECDH key pair", null, WF_METADATA_ERROR);
            ecdhPublicKey = recipient.getEcdhPublicKey();
            if (ecdhPublicKey == null) throw new WfException("Missing recipient's ECDH key", null, WF_METADATA_ERROR);
        } else if (recipient.isSelf()) {
            ecdhKeypair = recipient.getEcdhKeyPair();
            if (ecdhKeypair == null) throw new WfException("Missing recipients ECDH key pair", null, WF_METADATA_ERROR);
            ecdhPublicKey = originator.getEcdhPublicKey();
            if (ecdhPublicKey == null) throw new WfException("Missing originator's ECDH key", null, WF_METADATA_ERROR);
        } else {
            throw new WfException("Cannot encrypt or decrypt message if not the originator or recipient", null, WF_CRYPTO_ERROR);
        }
        try {
            return new WfEncryptionKey(ecdhPublicKey, ecdhKeypair);
        } catch (WfCryptoException e) {
            throw new WfException("Could not generate negotiated encryption key: " + e.getMessage(), e, WF_CRYPTO_ERROR);
        }
    }
}

