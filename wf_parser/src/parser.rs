use wf_field::{
    create_request_fields, Field, FieldDefinitionParser, FieldDefinitionParserBase, MessageType,
};

use crate::header::Header;

pub struct Parser {
    pub code: MessageType,
    pub header: Vec<Field>,
    pub body: Vec<Field>,
}

impl Parser {
    pub fn parse<T: FieldDefinitionParser>(mut parser: T) -> Self {
        let mut header = Header::new(parser.parse_header());

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
