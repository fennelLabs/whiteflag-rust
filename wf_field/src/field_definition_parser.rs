use crate::{definitions, Field, FieldDefinition};

pub trait FieldDefinitionParser {
    /// uses FieldDefinition to extract the associated string value from data
    fn parse(&mut self, definition: &FieldDefinition) -> String;
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
        /* if self.data.len() < field_defs.len() {
            panic!("not enough field definitions to process given values\nvalues: {:#?}\ndefinitions: {:#?}", self.data, field_defs);
        } */

        field_defs
            .into_iter()
            .map(|f| {
                let value = self.parse(&f);
                Field::new(f, value)
            })
            .collect()
    }

    fn parse_header(&mut self) -> Vec<Field> {
        let definitions = definitions::header::DEFINITIONS;
        let fields = self.parse_fields(definitions.to_vec());

        fields
    }
}
