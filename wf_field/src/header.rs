use crate::{definitions, types::MessageType, Field, FieldDefinition, FieldDefinitionParser};

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MessageHeaderOrder {
    Prefix = 0,
    Version = 1,
    EncryptionIndicator = 2,
    DuressIndicator = 3,
    MessageCode = 4,
    ReferenceIndicator = 5,
    ReferencedMessage = 6,
}

impl<'a> MessageHeaderOrder {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn get<'b>(&'a self, fields: &'b [Field]) -> &'b Field {
        &fields[self.as_usize()]
    }
}

pub struct Header {
    fields: Vec<Field>,
    code: MessageType,
    psuedo_code: Option<MessageType>,
}

impl Header {
    pub fn new(fields: Vec<Field>, code: MessageType) -> Self {
        Self {
            fields,
            code,
            psuedo_code: None,
        }
    }

    /// if the pseudo code is present, that means self.code = 'T'
    /// in which case, the pseudo_code is the "real" message code
    /// which defines the body definitions
    pub fn code(&self) -> MessageType {
        self.psuedo_code.unwrap_or(self.code)
    }

    pub fn check_for_pseudo_code<T: FieldDefinitionParser>(
        &mut self,
        parser: &mut T,
    ) -> Option<Field> {
        // if this is a test message, then we need to parse the pseudo code
        if &self.code == &MessageType::Test {
            let def = definitions::test::PSEUDO_MESSAGE_CODE;
            let pseudo_code = parser.parse(&def);
            self.psuedo_code = Some(MessageType::get_message_code(&pseudo_code));
            Some(Field::new(def, pseudo_code))
        } else {
            None
        }
    }

    pub fn fields(self) -> Vec<Field> {
        self.fields
    }
}
