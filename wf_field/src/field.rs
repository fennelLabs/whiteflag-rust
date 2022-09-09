use super::field_definition::FieldDefinition;

#[derive(Clone, Debug)]
pub struct Field {
    pub definition: FieldDefinition,
    value: String,
    name: String,
}

impl Field {
    pub fn new(definition: FieldDefinition, value: String) -> Field {
        let name = definition.get_name().expect("must give Field a name");
        Field::new_with_name(value, name.to_string(), definition)
    }

    pub fn new_with_name(value: String, name: String, definition: FieldDefinition) -> Field {
        Field {
            definition,
            value,
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /* pub fn get(&self, data: Vec<String>) -> WhiteflagResult<String> {
        if data.len() < self.get_minimum_starting_position() {
            return Err(WhiteflagError::InvalidLength);
        }

        data[self.start_byte..self.end_byte as usize]
            .first()
            .ok_or(WhiteflagError::InvalidLength)
    } */

    pub fn get(&self) -> &String {
        &self.value
    }

    pub fn encode(&self) -> Vec<u8> {
        self.definition.bytes.encoding.encode(&self.value)
    }

    pub fn encode_as_hex(&self) -> String {
        hex::encode(self.encode())
    }

    pub fn decode(&mut self, data: &[u8]) -> String {
        let s = match self
            .definition
            .bytes
            .encoding
            .decode(data, self.bit_length())
        {
            Ok(r) => r,
            Err(e) => {
                panic!("error: {}\n\t{:#?}", e, &self);
            }
        };
        self.value = s.clone();
        s
    }

    /**
     * Gets the byte length of the unencoded field value
     * @return the byte length of the unencoded field value
     */
    pub fn byte_length(&self) -> usize {
        if let Some(len) = self.definition.expected_byte_length() {
            return len;
        }

        self.value.len()
    }

    /**
     * Gets the bit length of the encoded field
     * @return the bit length of the compressed encoded field value
     */
    pub fn bit_length(&self) -> usize {
        return self
            .definition
            .bytes
            .encoding
            .convert_to_bit_length(self.byte_length());
    }
}

impl AsRef<FieldDefinition> for Field {
    fn as_ref(&self) -> &FieldDefinition {
        &self.definition
    }
}

impl AsRef<String> for Field {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl Into<String> for Field {
    fn into(self) -> String {
        self.value
    }
}
