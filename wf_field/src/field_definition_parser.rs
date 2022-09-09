use crate::{definitions, types::MessageType, Field, FieldDefinition, MessageHeaderOrder};

pub trait FieldDefinitionParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
    /// meant to calculate remaining values (if any) for request field definitions
    fn remaining(&self) -> usize;
}

pub trait FieldDefinitionParserBase {
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field>;
    fn parse_header(&mut self) -> Vec<Field>;
}

impl<T: FieldDefinitionParser> FieldDefinitionParserBase for T {
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

    fn parse_header(&mut self) -> Vec<Field> {
        let definitions = definitions::header::DEFINITIONS;
        let fields = self.parse_fields(definitions.to_vec());

        fields
    }
}
