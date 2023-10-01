use wf_codec::CodecError;

use crate::{definitions, Field, FieldDefinition};

pub trait FieldDefinitionParser {
    /// uses FieldDefinition to extract the associated string value from data
    fn parse(&mut self, definition: &FieldDefinition) -> Result<String, CodecError>;
    /// meant to calculate remaining values (if any) for request field definitions
    fn remaining(&self) -> usize;
}

pub trait FieldDefinitionParserBase {
    /// parse multiple FieldDefinitions and extract its assoicated values and converts it into Fields
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field>;
    /// parses header definitions into an array of Fields
    fn parse_header(&mut self) -> Vec<Field>;
}

impl<T: FieldDefinitionParser> FieldDefinitionParserBase for T {
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field> {
        field_defs
            .into_iter()
            .map(|f| {
                let value = self.parse(&f).unwrap();
                Field::new(f, value)
            })
            .collect()
    }

    fn parse_header(&mut self) -> Vec<Field> {
        self.parse_fields(definitions::header::DEFINITIONS.to_vec())
    }
}
