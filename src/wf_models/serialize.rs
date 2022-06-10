use crate::wf_core::basic_message::BasicMessage;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for BasicMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = self.get_fields();
        let length = fields.len();

        let mut state = serializer.serialize_struct("BasicMessage", length)?;

        for f in fields {
            state.serialize_field(name_map(&f.name), f.get())?;
        }

        state.end()
    }
}

fn name_map(name: &str) -> &'static str {
    match name {
        /* headers */
        "Prefix" => "prefix",
        "Version" => "version",
        "EncryptionIndicator" => "encryptionIndicator",
        "DuressIndicator" => "duressIndicator",
        "MessageCode" => "messageCode",
        "ReferenceIndicator" => "referenceIndicator",
        "ReferencedMessage" => "referencedMessage",
        /* authentication */
        "VerificationMethod" => "verificationMethod",
        "VerificationData" => "verificationData",
        _ => "",
    }
}
