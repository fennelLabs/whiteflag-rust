use crate::{definitions::*, FieldDefinition};
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

    pub fn get_message_code(code: &str) -> Self {
        Self::from_code(
            code.chars()
                .nth(0)
                .unwrap_or_else(|| panic!("invalid message code: {}", code)),
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MessageType {
    /// Undefined message type
    Any,

    /// Authentication message type
    /// <p> Message introducing the sender on the network with the sender’s authentication data
    /// @wfref 4.3.4 Management Messages: Authentication
    Authentication,

    /// Cryptographic message type
    /// <p> Message for management of keys and parameters of cryptographic functions
    /// @wfref 4.3.5 Management Messages: Cryptographic Support
    Cryptographic,

    /// Test message type
    /// <p> Message that can be used for testing Whiteflag functionality by applications
    /// @wfref 4.3.6 Management Messages: Test
    Test,

    /// Resource message type
    /// <p> Message to point to an internet resource
    /// @wfref 4.3.2 Functional Messages: Resource
    Resource,

    /// Free Text message type
    /// <p> Message to send a free text string
    /// @wfref 4.3.3 Functional Messages: Free Text
    FreeText,

    /// Protective Sign message type
    /// <p> Sign to mark objects under the protection of international law
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.1 Protective Signs
    Protective,

    /// Emergency Signal message type
    /// <p> Signal to send an emergency signal when in need of assistance
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.2 Emergency Signals
    Emergency,

    /// Danger Sign message type
    /// <p> Sign to mark a location or area of imminent danger, e.g. an area under attack, land mines, disaster, etc.
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.3 Danger and Disaster Signs
    Danger,

    /// Status Signal message type
    /// <p> Signal to provide the status of an object, or specifically for persons: give a proof of life
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.4 Status Signals
    Status,

    /// Infrastructure Sign message type
    /// <p> Sign to mark critical infrastructure, e.g. roads, utilities, water treatment, hospitals, power plants etc.
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.5 Infrastructure Signs
    Infrastructure,

    /// Mission Signal message type
    /// <p> Signal to provide information on activities undertaken during a mission
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.6 Mission Signals
    Mission,

    /// Request Signal message type
    /// <p> Signal to perform requests to other parties
    /// @wfref 4.3.1 Functional Messages: Signs/Signals
    /// @wfref 4.3.1.2.7 Request Signals
    Request,
}
