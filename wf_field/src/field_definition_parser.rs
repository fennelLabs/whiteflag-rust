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
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Result<Vec<Field>, CodecError>;
    /// parses header definitions into an array of Fields
    fn parse_header(&mut self) -> Result<Vec<Field>, CodecError>;
}

impl<T: FieldDefinitionParser> FieldDefinitionParserBase for T {
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Result<Vec<Field>, CodecError> {
        let mut fields: Vec<Field> = Vec::new();
        for f in field_defs.into_iter() {
            let value: String = self.parse(&f)?;
            let field = Field::new(f, value);
            fields.push(field);
        }
        Ok(fields)
    }

    fn parse_header(&mut self) -> Result<Vec<Field>, CodecError> {
        self.parse_fields(definitions::header::DEFINITIONS.to_vec())
    }
}
