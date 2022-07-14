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
            let json_name =
                name_map(&f.definition.name).map_err(|e| serde::ser::Error::custom(e))?;
            state.serialize_field(json_name, f.get())?;
        }

        state.end()
    }
}

fn name_map(name: &str) -> Result<&'static str, String> {
    let json_name = match name {
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
        /* crypto */
        "CryptoDataType" => "cryptoDataType",
        "CryptoData" => "cryptoData",
        /* free text */
        "Text" => "text",
        /* resource */
        "ResourceMethod" => "resourceMethod",
        "ResourceData" => "resourceData",
        /* test */
        "PseudoMessageCode" => "pseudoMessageCode",
        /* sign signal */
        "SubjectCode" => "subjectCode",
        "DateTime" => "dateTime",
        "Duration" => "duration",
        "ObjectType" => "objectType",
        "ObjectLatitude" => "objectLatitude",
        "ObjectLongitude" => "objectLongitude",
        "ObjectSizeDim1" => "objectSizeDim1",
        "ObjectSizeDim2" => "objectSizeDim2",
        "ObjectOrientation" => "objectOrientation",
        /* request */
        "ObjectTypeQuant" => "objectTypeQuant",
        _ => return Err(format!("missing support for field name: {}", name)),
    };

    Ok(json_name)
}
