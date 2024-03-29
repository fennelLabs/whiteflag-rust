use wf_field::MessageHeaderOrder;
use wf_field::{definitions, Field, FieldDefinitionParser, MessageCodeType};

pub struct Header {
    fields: Vec<Field>,
    code: MessageCodeType,
    psuedo_code: Option<MessageCodeType>,
}

impl Header {
    pub fn new(fields: Vec<Field>) -> Result<Self, wf_field::Error> {
        let code: MessageCodeType = MessageCodeType::get_message_code(
            fields[MessageHeaderOrder::MessageCode.as_usize()].get(),
        )?;

        Ok(Self {
            fields,
            code,
            psuedo_code: None,
        })
    }

    /// if the pseudo code is present, that means self.code = 'T'
    /// in which case, the pseudo_code is the "real" message code
    /// which defines the body definitions
    pub fn code(&self) -> MessageCodeType {
        self.psuedo_code.unwrap_or(self.code)
    }

    pub fn check_for_pseudo_code<T: FieldDefinitionParser>(
        &mut self,
        parser: &mut T,
    ) -> Option<Field> {
        // if this is a test message, then we need to parse the pseudo code
        if self.code == MessageCodeType::Test {
            let def = definitions::test::PSEUDO_MESSAGE_CODE;
            let pseudo_code = match parser.parse(&def) {
                Ok(code) => code,
                Err(_) => return None,
            };
            self.psuedo_code = match MessageCodeType::get_message_code(&pseudo_code) {
                Ok(code) => Some(code),
                Err(_) => None,
            };
            Some(Field::new(def, pseudo_code))
        } else {
            None
        }
    }

    pub fn fields(self) -> Vec<Field> {
        self.fields
    }
}
