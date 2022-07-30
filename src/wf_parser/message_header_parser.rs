use super::parsed_field_definition::ParsedFieldDefinition;
use crate::wf_codec::encoding::*;
use crate::wf_field::{
    definitions::{message_code, test_message_code},
    FieldDefinition,
};
use regex::Regex;


pub struct MessageHeaderParser {
    prefix: ParsedFieldDefinition,
    version: ParsedFieldDefinition,
    encryption_indicator: ParsedFieldDefinition,
    duress_indicator: ParsedFieldDefinition,
    message_code: ParsedFieldDefinition,
    reference_indicator: ParsedFieldDefinition,
    referenced_message: ParsedFieldDefinition,
}

impl MessageHeaderParser {
    /// message code is a default header
    pub fn message_code(&self) -> &ParsedFieldDefinition {
        &self.message_code
    }

    /// the test message code is technically part of a test message body
    /// this field is always ordered after the header
    /// therefore, the `previous` position is the `end_bit` of the last header `ParsedFieldDefinition`
    pub fn test_message_code(&self) -> ParsedFieldDefinition {
        self.referenced_message.next(test_message_code())
    }

    pub fn to_vec(self) -> Vec<FieldDefinition> {
        vec![
            self.prefix.into(),
            self.version.into(),
            self.encryption_indicator.into(),
            self.duress_indicator.into(),
            self.message_code.into(),
            self.reference_indicator.into(),
            self.referenced_message.into(),
        ]
    }
}

impl Default for MessageHeaderParser {
    fn default() -> Self {
        let prefix = FieldDefinition::new("Prefix", Regex::new("^WF$").ok(), UTF8, 0, 2);
        let version = FieldDefinition::new("Version", Regex::new("^[A-Z0-9]{1}$").ok(), UTF8, 2, 3); //"(?=1)^[A-Z0-9]{1}$"
        let encryption_indicator = FieldDefinition::new(
            "EncryptionIndicator",
            Regex::new("^[A-Z0-9]{1}$").ok(), //"(?=0|1|2)^[A-Z0-9]{1}$"
            UTF8,
            3,
            4,
        );
        let duress_indicator =
            FieldDefinition::new("DuressIndicator", Regex::new("^[0-1]{1}$").ok(), BIN, 4, 5);
        let message_code = message_code();
        let reference_indicator = FieldDefinition::new(
            "ReferenceIndicator",
            Regex::new(
                ["^", HEX.charset, "{1}$"] //"(?=0|1|2|3|4|5|6|7|8|9)^", HEX.charset, "{1}$"
                    .concat()
                    .as_str(),
            )
            .ok(),
            HEX,
            6,
            7,
        );
        let referenced_message = FieldDefinition::new(
            "ReferencedMessage",
            Regex::new(["^", HEX.charset, "{64}$"].concat().as_str()).ok(),
            HEX,
            7,
            71,
        );

        let mut parsed_defs = ParsedFieldDefinition::parse(vec![
            prefix,
            version,
            encryption_indicator,
            duress_indicator,
            message_code,
            reference_indicator,
            referenced_message,
        ]);

        MessageHeaderParser {
            prefix: parsed_defs.remove(0),
            version: parsed_defs.remove(0),
            encryption_indicator: parsed_defs.remove(0),
            duress_indicator: parsed_defs.remove(0),
            message_code: parsed_defs.remove(0),
            reference_indicator: parsed_defs.remove(0),
            referenced_message: parsed_defs.remove(0),
        }
    }
}

/* impl std::ops::Deref for MessageHeaderParser {
    type Target = [FieldDefinition];

    fn deref(&self) -> &Self::Target {
        .as_slice()
    }
} */
