use crate::{
    definitions, header::Header, request::create_request_fields, types::MessageType, Field,
    FieldDefinition, MessageHeaderOrder,
};

pub trait FieldDefinitionParser {
    fn parse(&mut self, definition: &FieldDefinition) -> String;
    /// meant to calculate remaining values (if any) for request field definitions
    fn remaining(&self) -> usize;
}

pub trait FieldDefinitionParserBase {
    fn parse_fields(&mut self, field_defs: Vec<FieldDefinition>) -> Vec<Field>;
    fn parse_header(&mut self) -> Header;
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

    fn parse_header(&mut self) -> Header {
        let definitions = definitions::header::DEFINITIONS;
        let fields = self.parse_fields(definitions.to_vec());
        let code: MessageType =
            MessageType::get_message_code(fields[MessageHeaderOrder::MessageCode.as_usize()].get());

        Header::new(fields, code)
    }
}

pub struct Parser {
    pub code: MessageType,
    pub header: Vec<Field>,
    pub body: Vec<Field>,
}

impl Parser {
    pub fn parse<T: FieldDefinitionParser>(mut parser: T) -> Self {
        let mut header = parser.parse_header();

        let mut body = Vec::new();

        // parses and adds pseudo code field to body if message type is 'T'
        if let Some(pc) = header.check_for_pseudo_code(&mut parser) {
            body.push(pc);
        }

        let code = header.code();

        let body_defs = code.definitions().to_vec();
        body.append(parser.parse_fields(body_defs).as_mut());

        if code == MessageType::Request {
            body.append(create_request_fields(&mut parser).as_mut());
        }

        Parser {
            code,
            header: header.fields(),
            body,
        }
    }
}
