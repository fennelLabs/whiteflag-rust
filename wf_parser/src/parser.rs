use wf_field::{
    create_request_fields, Field, FieldDefinitionParser, FieldDefinitionParserBase, MessageCodeType,
};

use crate::header::Header;

pub struct Parser {
    pub code: MessageCodeType,
    pub header: Vec<Field>,
    pub body: Vec<Field>,
}

impl Parser {
    pub fn parse<T: FieldDefinitionParser>(mut parser: T) -> Result<Self, wf_field::Error> {
        let parsed_header = parser.parse_header();

        if parsed_header.is_err() {
            return Err(wf_field::Error::Other(Box::new(
                parsed_header.err().unwrap(),
            )));
        }

        let parsed_header_vec = parsed_header.unwrap();

        let mut header = Header::new(parsed_header_vec)?;

        let mut body = Vec::new();

        // parses and adds pseudo code field to body if message type is 'T'
        if let Some(pc) = header.check_for_pseudo_code(&mut parser) {
            body.push(pc);
        }

        let code = header.code();

        let body_defs = code.definitions()?.to_vec();

        let body_defs_parser_result = parser.parse_fields(body_defs);
        if body_defs_parser_result.is_err() {
            return Err(wf_field::Error::Other(Box::new(
                body_defs_parser_result.err().unwrap(),
            )));
        }

        let mut body_defs_parser = body_defs_parser_result.unwrap();

        body.append(body_defs_parser.as_mut());

        if code == MessageCodeType::Request {
            body.append(create_request_fields(&mut parser).as_mut());
        }

        Ok(Parser {
            code,
            header: header.fields(),
            body,
        })
    }
}
