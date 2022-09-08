use crate::{definitions::*, get_body_from_code_char, Field, FieldDefinition};

/* struct MessageType {
    pub message_code: char,
    pub definitions: Vec<FieldDefinition>,
}

impl MessageType {
    pub fn from_code_option(code: Option<&char>) -> MessageType {
        let c = code.unwrap_or(&' ');
        Self::from_code(c)
    }

    pub fn from_code(code: &char) -> MessageType {
        MessageType {
            message_code: *code,
            definitions: get_body_from_code_char(code),
        }
    }
} */

/* pub enum FieldKind {
    GENERIC,
    AUTHENTICATION,
    CRYPTO,
    TEXT,
    RESOURCE,
    TEST,
    SIGNAL,
    REQUEST,
} */

/*
match code {
        'A' => authentication::DEFINITIONS,
        'K' => crypto::DEFINITIONS,
        'T' => test::DEFINITIONS,
        'R' => resource::DEFINITIONS,
        'F' => freetext::DEFINITIONS,
        'P' | 'E' | 'D' | 'S' | 'I' | 'M' | 'Q' => sign::DEFINITIONS,
        _ => panic!("'{}' is not a valid message code", code),
    } */

impl MessageType {
    pub fn from_code(code: char) -> Self {
        match code {
            'A' => MessageType::Authentication,
            'K' => MessageType::Cryptographic,
            'T' => MessageType::Test,
            'R' => MessageType::Resource,
            'F' => MessageType::FreeText,
            'P' => MessageType::Protective,
            'E' => MessageType::Emergency,
            'D' => MessageType::Danger,
            'S' => MessageType::Status,
            'I' => MessageType::Infrastructure,
            'M' => MessageType::Mission,
            'Q' => MessageType::Request,
            _ => MessageType::Any,
        }
    }

    pub fn definitions(&self) -> &'static [FieldDefinition] {
        match &self {
            MessageType::Any => panic!("no definition fields for undefined message type"),
            MessageType::Authentication => authentication::DEFINITIONS,
            MessageType::Cryptographic => crypto::DEFINITIONS,
            MessageType::Test => test::DEFINITIONS,
            MessageType::Resource => resource::DEFINITIONS,
            MessageType::FreeText => freetext::DEFINITIONS,
            MessageType::Protective
            | MessageType::Emergency
            | MessageType::Danger
            | MessageType::Status
            | MessageType::Infrastructure
            | MessageType::Mission
            | MessageType::Request => sign::DEFINITIONS,
        }
    }
}

enum MessageType {
    /**
     * Undefined message type
     */
    Any, //UNDEFINED

    /**
     * Authentication message type
     * <p> Message introducing the sender on the network with the sender’s authentication data
     * @wfref 4.3.4 Management Messages: Authentication
     */
    Authentication, //("A", authenticationBodyFields),

    /**
     * Cryptographic message type
     * <p> Message for management of keys and parameters of cryptographic functions
     * @wfref 4.3.5 Management Messages: Cryptographic Support
     */
    Cryptographic, //("K", cryptoBodyFields),

    /**
     * Test message type
     * <p> Message that can be used for testing Whiteflag functionality by applications
     * @wfref 4.3.6 Management Messages: Test
     */
    Test, //("T", testBodyFields),

    /**
     * Resource message type
     * <p> Message to point to an internet resource
     * @wfref 4.3.2 Functional Messages: Resource
     */
    Resource, //("R", resourceBodyFields),

    /**
     * Free Text message type
     * <p> Message to send a free text string
     * @wfref 4.3.3 Functional Messages: Free Text
     */
    FreeText, //("F", freetextBodyFields),

    /**
     * Protective Sign message type
     * <p> Sign to mark objects under the protection of international law
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.1 Protective Signs
     */
    Protective, //("P", signsignalBodyFields),

    /**
     * Emergency Signal message type
     * <p> Signal to send an emergency signal when in need of assistance
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.2 Emergency Signals
     */
    Emergency, //("E", signsignalBodyFields),

    /**
     * Danger Sign message type
     * <p> Sign to mark a location or area of imminent danger, e.g. an area under attack, land mines, disaster, etc.
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.3 Danger and Disaster Signs
     */
    Danger, //("D", signsignalBodyFields),

    /**
     * Status Signal message type
     * <p> Signal to provide the status of an object, or specifically for persons: give a proof of life
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.4 Status Signals
     */
    Status, //("S", signsignalBodyFields),

    /**
     * Infrastructure Sign message type
     * <p> Sign to mark critical infrastructure, e.g. roads, utilities, water treatment, hospitals, power plants etc.
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.5 Infrastructure Signs
     */
    Infrastructure, //("I", signsignalBodyFields),

    /**
     * Mission Signal message type
     * <p> Signal to provide information on activities undertaken during a mission
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.6 Mission Signals
     */
    Mission, //("M", signsignalBodyFields),

    /**
     * Request Signal message type
     * <p> Signal to perform requests to other parties
     * @wfref 4.3.1 Functional Messages: Signs/Signals
     * @wfref 4.3.1.2.7 Request Signals
     */
    Request, //Q("Q", signsignalBodyFields);
}
