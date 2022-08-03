use std::collections::HashMap;

use fennel_lib::wf_core::types::{MessageType, MessageTypeEnum};

use crate::{wf_buffer::WhiteflagBuffer, wf_field::Field, wf_core::{basic_message::BasicMessage, segment::MessageSegment}, error::{WhiteflagResult, WhiteflagCreatorError}};

const PREFIX: &str = "WF";
const PROTOCOL_VERSION: &str = "1";
const FIELD_PREFIX: &str = "Prefix";
const FIELD_VERSION: &str = "Version";
const FIELD_MESSAGETYPE: &str = "MessageCode";
const FIELD_TESTMESSAGETYPE: &str = "PseudoMessageCode";

struct WhiteflagMessageCreator {
    messageType: MessageType,
    header: Vec<Field>,
    body: Vec<Field>,
}

impl WhiteflagMessageCreator {
    fn new() -> WhiteflagMessageCreator {
        WhiteflagMessageCreator {
            messageType: MessageType::from_code_option(None),
            header: vec![],
            body: vec![],
        }
    }

    fn create(&self) -> BasicMessage {
        return BasicMessage::new(' ', self.header, self.body);
    }

    fn message_type(&self, messageType: MessageType) -> &WhiteflagMessageCreator {
        self.messageType = messageType;
        self.header = MessageSegment::new(messageType.getHeaderFields());
        self.body = MessageSegment::new(messageType.getBodyFields());

        self.header.set(FIELD_PREFIX, PREFIX);
        self.header.set(FIELD_VERSION, PROTOCOL_VERSION);
        self.header.set(FIELD_MESSAGETYPE, messageType.getCode());

        self
    }

    fn map(
        &self,
        headerValues: HashMap<String, String>,
        bodyValues: HashMap<String, String>,
    ) -> WhiteflagResult<&WhiteflagMessageCreator> {
        self.header = self.messageType.getHeaderFields();
        if !self.header.setAll(headerValues) {
            return Err(WhiteflagCreatorError::InvalidHeaderField);
        }
        self.messageType = MessageType::from_code(self.header.get(FIELD_MESSAGETYPE));

        self.body = self.messageType.getBodyFields();
        match (self.messageType) {
            MessageTypeEnum::Test => {
                let pseudoMessageType =
                    MessageType::from_code(bodyValues.get(FIELD_TESTMESSAGETYPE));
                self.body
                    .append(pseudoMessageType.getBodyFields());
            }
            MessageTypeEnum::Request => {
                let nRequestObjects = (bodyValues.size() - body.getNoFields()) / 2;
                self.body.append(
                    self.messageType.createRequestFields(nRequestObjects),
                );
            }
            _ => {}
        }
        if !self.body.setAll(bodyValues) {
            return Err(WhiteflagCreatorError::InvalidBodyField);
        }
        Ok(self)
    }

    fn deserialize(&self, serializedMsg: String) -> &WhiteflagMessageCreator {
        let nextField = 0;

        self.header = self.messageType.getHeaderFields();
        self.header.deserialize(serializedMsg, nextField);
        self.messageType = MessageType::from_code(self.header.get(FIELD_MESSAGETYPE));

        self.body = self.messageType.getBodyFields();
        self.body.deserialize(serializedMsg, nextField);
        nextField = self.body.getNoFields();
        match (self.messageType) {
            MessageTypeEnum::Test => {
                let pseudoMessageType = MessageType::from_code(self.body.get(FIELD_TESTMESSAGETYPE));
                self.body
                    .append(pseudoMessageType.getBodyFields());
            }
            MessageTypeEnum::Request => {
                let lastFieldByte = self.body.getField(-1).endByte;
                let nRequestObjects = (serializedMsg.length() - lastFieldByte) / 4; // One request object requires 2 fields of 2 bytes
                self.body.append(
                    self.messageType.createRequestFields(nRequestObjects),
                );
            }
            _ => {}
        }
        self.body.deserialize(serializedMsg, nextField);
        self
    }

    fn decode(&self, msgBuffer: WhiteflagBuffer) -> &WhiteflagMessageCreator {
        let bitCursor = 0;
        let nextField = 0;

        self.header = self.messageType.getHeaderFields();
        self.header.decode(msgBuffer, bitCursor, nextField);
        bitCursor += self.header.bitLength();
        self.messageType = MessageType::from_code(self.header.get(FIELD_MESSAGETYPE));

        self.body = self.messageType.getBodyFields();
        self.body.decode(msgBuffer, bitCursor, nextField);
        nextField = self.body.getNoFields();
        bitCursor += self.body.bitLength();
        match (self.messageType) {
            MessageTypeEnum::Test => {
                let pseudoMessageType =
                    MessageType::from_code(self.body.get(FIELD_TESTMESSAGETYPE));
                self.body
                    .append(pseudoMessageType.getBodyFields());
            }
            MessageTypeEnum::Request => {
                let nRequestObjects = (msgBuffer.bitLength() - bitCursor) / 16; // One request object requires 2 fields of 8 bits
                self.body.append(
                    self.messageType.createRequestFields(nRequestObjects),
                );
            }
            _ => {}
        }
        self.body.decode(msgBuffer, bitCursor, nextField);
        self
    }

    fn getUnencryptedHeader(&self, msgBuffer: WhiteflagBuffer) -> Vec<Field> {
        self.header = MessageType::ANY.getUnencryptedHeaderFields();
        self.header.decode(msgBuffer, 0, 0);
        return self.header;
    }

    fn compile(&self, fieldValues: Vec<String>) -> &WhiteflagMessageCreator {
        self.header = self.messageType.getHeaderFields();
        self.header.setAll(fieldValues, 0);
        self.messageType = MessageType::from_code(self.header.get(FIELD_MESSAGETYPE));

        let bodyStartIndex = self.header.getNoFields();
        self.body = self.messageType.getBodyFields();

        match (self.messageType) {
            MessageTypeEnum::Test => {
                let pseudoMessageType = MessageType::from_code(fieldValues[bodyStartIndex]);
                self.body
                    .append(pseudoMessageType.getBodyFields());
            }
            MessageTypeEnum::Request => {
                let nRequestObjects = (fieldValues.length
                    - (self.header.getNoFields() + self.body.getNoFields()))
                    / 2; // One request object requires 2 fields
                self.body.append(
                    self.messageType.createRequestFields(nRequestObjects),
                );
            }
            _ => {}
        }
        self.body.setAll(fieldValues, bodyStartIndex);
        self
    }
}
