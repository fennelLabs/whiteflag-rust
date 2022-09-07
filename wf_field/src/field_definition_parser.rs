use crate::{request::create_request_fields, Field, FieldDefinition, MessageHeaderOrder};

pub trait FieldDefinitionParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
    /// fetch the field definitions for the body
    fn body_field_definitions(&self) -> Vec<FieldDefinition>;
    /// meant to calculate remaining values (if any) for request field definitions
    fn remaining(&self) -> usize;
}

pub trait FieldDefinitionParserBase {
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field>;
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
}

pub struct Parser {
    pub code: char,
    pub header: Vec<Field>,
    pub body: Vec<Field>,
}

impl Parser {
    pub fn parse<T: FieldDefinitionParser>(mut parser: T) -> Self {
        let header = parser.parse_fields(crate::definitions::header::DEFINITIONS.to_vec());

        let code = MessageHeaderOrder::get_code(header.as_ref()).1;
        let body_defs = parser.body_field_definitions();
        let mut body = parser.parse_fields(body_defs);

        if code == 'Q' {
            body.append(create_request_fields(&mut parser).as_mut());
        }

        Parser { code, header, body }
    }
}
