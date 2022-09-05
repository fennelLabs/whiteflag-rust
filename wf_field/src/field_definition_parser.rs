use crate::{Field, FieldDefinition};

pub trait FieldDefinitionParserBase {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
    /// fetch the field definitions for the body
    fn body_field_definitions(&self) -> Vec<FieldDefinition>;
    /// meant to calculate remaining values (if any) for request field definitions
    fn remaining(&self) -> usize;
}

pub trait FieldDefinitionParser {
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field>;
}

impl<T: FieldDefinitionParserBase> FieldDefinitionParser for T {
    /// parses array of field definitions from a data source into a Field
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
}

pub struct Parser<T: FieldDefinitionParserBase> {
    parser: T,
}

impl<T: FieldDefinitionParserBase> Parser<T> {
    pub fn parse(&self) {}
}
