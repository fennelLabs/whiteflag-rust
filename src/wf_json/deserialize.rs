use serde::de;
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct WhiteflagFieldValues {
    pub fields: Vec<String>,
}

impl<'de> de::Deserialize<'de> for WhiteflagFieldValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(FieldValuesVisitor {})
    }
}

struct FieldValuesVisitor;

impl<'de> de::Visitor<'de> for FieldValuesVisitor {
    type Value = WhiteflagFieldValues;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("WhiteflagFieldValues")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let mut fields: HashMap<usize, String> = HashMap::new();

        while let Some((key, value)) = map.next_entry::<String, String>()? {
            let index = name_map(&key).map_err(serde::de::Error::custom)?;
            fields.insert(index, value);
        }

        let mut sortable: Vec<_> = fields.iter().collect();
        sortable.sort_by_key(|p| p.0);

        Ok(WhiteflagFieldValues {
            fields: sortable.iter().map(|p| p.1.to_owned()).collect(),
        })
    }
}

fn name_map(name: &str) -> Result<usize, String> {
    let index = match name {
        /* headers */
        "prefix" => 0,
        "version" => 1,
        "encryptionIndicator" => 2,
        "duressIndicator" => 3,
        "messageCode" => 4,
        "referenceIndicator" => 5,
        "referencedMessage" => 6,
        /* authentication */
        "verificationMethod" => 7,
        "verificationData" => 8,
        /* crypto */
        "cryptoDataType" => 9,
        "cryptoData" => 10,
        /* free text */
        "text" => 11,
        /* resource */
        "resourceMethod" => 12,
        "resourceData" => 13,
        /* test */
        "pseudoMessageCode" => 14,
        /* sign signal */
        "subjectCode" => 15,
        "dateTime" => 16,
        "duration" => 17,
        "objectType" => 18,
        "objectLatitude" => 19,
        "objectLongitude" => 20,
        "objectSizeDim1" => 21,
        "objectSizeDim2" => 22,
        "objectOrientation" => 23,
        /* request */
        "objectTypeQuant" => 24,
        _ => return Err(format!("missing index for field: {}", &name)),
    };

    Ok(index)
}
