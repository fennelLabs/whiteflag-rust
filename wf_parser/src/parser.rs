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
        let mut header = Header::new(parser.parse_header())?;

        let mut body = Vec::new();

        // parses and adds pseudo code field to body if message type is 'T'
        if let Some(pc) = header.check_for_pseudo_code(&mut parser) {
            body.push(pc);
        }

        let code = header.code();

        let body_defs = code.definitions()?.to_vec();
        body.append(parser.parse_fields(body_defs).as_mut());

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
