#[derive(Clone)]
pub struct WhiteflagMessage {
    pub prefix: String,
    pub message_type: String,
    pub version: String,
    pub message_code: String,

    pub duress_indictor: String,
    pub encryption_indicator: String,
    pub object_type: String,
    pub subject_code: String,

    pub reference_indicator: String,
    pub referenced_message: String,

    pub crypto_data_type: String,
    pub crypto_data: String,

    pub transaction_hash: String,
    pub originator_address: String,

    pub verification_method: String,
    pub verification_data: String,
}

impl WhiteflagMessage {
    pub fn new(message_code: String) -> WhiteflagMessage {
        WhiteflagMessage {
            prefix: "WF".to_string(),
            version: "1".to_string(),
            message_code: message_code.clone(),
            message_type: message_code.clone(),
            duress_indictor: "".to_string(),
            encryption_indicator: "".to_string(),
            object_type: "".to_string(),
            subject_code: "".to_string(),
            reference_indicator: "".to_string(),
            referenced_message: "".to_string(),
            crypto_data_type: "".to_string(),
            crypto_data: "".to_string(),
            transaction_hash: "".to_string(),
            originator_address: "".to_string(),
            verification_method: "".to_string(),
            verification_data: "".to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.message_code == self.message_type && self.prefix != "" && self.version == "1"
    }

    pub fn compile_auth_message(field_values: Vec<&str>) -> Option<WhiteflagMessage> {
        let message = WhiteflagMessage {
            prefix: field_values[0].to_string(),
            version: field_values[1].to_string(),
            message_code: field_values[4].to_string(),
            message_type: "A".to_string(),
            duress_indictor: field_values[3].to_string(),
            encryption_indicator: field_values[2].to_string(),
            object_type: "".to_string(),
            subject_code: "".to_string(),
            reference_indicator: field_values[5].to_string(),
            referenced_message: field_values[6].to_string(),
            crypto_data_type: "".to_string(),
            crypto_data: "".to_string(),
            transaction_hash: "".to_string(),
            originator_address: "".to_string(),
            verification_method: field_values[7].to_string(),
            verification_data: field_values[8].to_string(),
        };
        if message.is_valid() {
            return Some(message);
        } else {
            return None;
        }
    }

    pub fn get_encryption_indicator(&self) -> String {
        self.encryption_indicator.clone()
    }
    pub fn set_encryption_indicator(&mut self, arg: String) -> bool {
        if self.encryption_indicator == "" {
            self.encryption_indicator = arg;
            return true;
        }
        return false;
    }

    pub fn get_subject_code(&self) -> String {
        self.subject_code.clone()
    }
    pub fn set_subject_code(&mut self, arg: String) -> bool {
        if self.subject_code == "" {
            self.subject_code = arg;
            return true;
        }
        return false;
    }

    pub fn get_object_type(&self) -> String {
        self.object_type.clone()
    }
    pub fn set_object_type(&mut self, arg: String) -> bool {
        if self.object_type == "" {
            self.object_type = arg;
            return true;
        }
        return false;
    }

    pub fn get_transaction_hash(&self) -> String {
        self.transaction_hash.clone()
    }
    pub fn set_transaction_hash(&mut self, arg: String) -> Option<String> {
        if self.transaction_hash == "" {
            self.transaction_hash = arg;
            return None;
        }
        Some(self.transaction_hash.clone())
    }

    pub fn get_originator_address(&self) -> String {
        self.originator_address.clone()
    }
    pub fn set_originator_address(&mut self, arg: String) -> Option<String> {
        if self.originator_address == "" {
            self.originator_address = arg;
            return None;
        }
        Some(self.originator_address.clone())
    }

    /// Set the whiteflag message's message type.
    pub fn set_message_type(&mut self, message_type: String) {
        self.message_type = message_type;
    }

    /// Get a reference to the whiteflag message's prefix.
    pub fn prefix(&self) -> &str {
        self.prefix.as_ref()
    }

    /// Get a reference to the whiteflag message's version.
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Get a mutable reference to the whiteflag message's duress indictor.
    pub fn duress_indictor_mut(&mut self) -> &mut String {
        &mut self.duress_indictor
    }

    /// Get a mutable reference to the whiteflag message's message code.
    pub fn message_code_mut(&mut self) -> &mut String {
        &mut self.message_code
    }

    /// Get a mutable reference to the whiteflag message's reference indicator.
    pub fn reference_indicator_mut(&mut self) -> &mut String {
        &mut self.reference_indicator
    }

    /// Get a mutable reference to the whiteflag message's referenced message.
    pub fn referenced_message_mut(&mut self) -> &mut String {
        &mut self.referenced_message
    }

    /// Get a reference to the whiteflag message's crypto data type.
    pub fn crypto_data_type(&self) -> &str {
        self.crypto_data_type.as_ref()
    }

    /// Get a mutable reference to the whiteflag message's crypto data.
    pub fn crypto_data_mut(&mut self) -> &mut String {
        &mut self.crypto_data
    }

    /// Get a mutable reference to the whiteflag message's crypto data type.
    pub fn crypto_data_type_mut(&mut self) -> &mut String {
        &mut self.crypto_data_type
    }

    /// Set the whiteflag message's crypto data type.
    pub fn set_crypto_data_type(&mut self, crypto_data_type: String) {
        self.crypto_data_type = crypto_data_type;
    }

    /// Get a reference to the whiteflag message's crypto data.
    pub fn crypto_data(&self) -> &str {
        self.crypto_data.as_ref()
    }

    /// Set the whiteflag message's crypto data.
    pub fn set_crypto_data(&mut self, crypto_data: String) {
        self.crypto_data = crypto_data;
    }

    /// Get a mutable reference to the whiteflag message's prefix.
    pub fn prefix_mut(&mut self) -> &mut String {
        &mut self.prefix
    }

    /// Get a reference to the whiteflag message's message type.
    pub fn message_type(&self) -> &str {
        self.message_type.as_ref()
    }

    /// Get a mutable reference to the whiteflag message's message type.
    pub fn message_type_mut(&mut self) -> &mut String {
        &mut self.message_type
    }

    /// Get a reference to the whiteflag message's duress indictor.
    pub fn duress_indictor(&self) -> &str {
        self.duress_indictor.as_ref()
    }

    /// Get a reference to the whiteflag message's message code.
    pub fn message_code(&self) -> &str {
        self.message_code.as_ref()
    }

    /// Get a reference to the whiteflag message's reference indicator.
    pub fn reference_indicator(&self) -> &str {
        self.reference_indicator.as_ref()
    }

    /// Get a reference to the whiteflag message's referenced message.
    pub fn referenced_message(&self) -> &str {
        self.referenced_message.as_ref()
    }

    /// Get a reference to the whiteflag message's verification method.
    pub fn verification_method(&self) -> &str {
        self.verification_method.as_ref()
    }

    /// Get a reference to the whiteflag message's verification data.
    pub fn verification_data(&self) -> &str {
        self.verification_data.as_ref()
    }
}
