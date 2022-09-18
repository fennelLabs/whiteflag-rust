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
    let index = match name.to_lowercase().as_str() {
        /* headers */
        "prefix" => 0,
        "version" => 1,
        "encryptionindicator" => 2,
        "duressindicator" => 3,
        "messagecode" => 4,
        "referenceindicator" => 5,
        "referencedmessage" => 6,
        /* authentication */
        "verificationmethod" => 7,
        "verificationdata" => 8,
        /* crypto */
        "cryptodatatype" => 9,
        "cryptodata" => 10,
        /* free text */
        "text" => 11,
        /* resource */
        "resourcemethod" => 12,
        "resourcedata" => 13,
        /* test */
        "pseudomessagecode" => 14,
        /* sign signal */
        "subjectcode" => 15,
        "datetime" => 16,
        "duration" => 17,
        "objecttype" => 18,
        "objectlatitude" => 19,
        "objectlongitude" => 20,
        "objectsizedim1" => 21,
        "objectsizedim2" => 22,
        "objectorientation" => 23,
        /* request */
        "objecttypequant" => 24,
        _ => return Err(format!("missing index for field: {}", &name)),
    };

    Ok(index)
}
