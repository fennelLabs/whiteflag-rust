use crate::{
    definitions::*,
    message_body_types::{Authentication, MessageBodyType},
    FieldDefinition,
};
use std::str::FromStr;

impl MessageCodeType {
    pub fn from_code(code: char) -> Self {
        match code {
            'A' => MessageCodeType::Authentication,
            'K' => MessageCodeType::Cryptographic,
            'T' => MessageCodeType::Test,
            'R' => MessageCodeType::Resource,
            'F' => MessageCodeType::FreeText,
            'P' => MessageCodeType::Protective,
            'E' => MessageCodeType::Emergency,
            'D' => MessageCodeType::Danger,
            'S' => MessageCodeType::Status,
            'I' => MessageCodeType::Infrastructure,
            'M' => MessageCodeType::Mission,
            'Q' => MessageCodeType::Request,
            _ => MessageCodeType::Any,
        }
    }

    pub fn definitions(&self) -> &'static [FieldDefinition] {
        match &self {
            MessageCodeType::Any => panic!("no definition fields for undefined message type"),
            MessageCodeType::Authentication => authentication::DEFINITIONS,
            MessageCodeType::Cryptographic => crypto::DEFINITIONS,
            MessageCodeType::Test => test::DEFINITIONS,
            MessageCodeType::Resource => resource::DEFINITIONS,
            MessageCodeType::FreeText => freetext::DEFINITIONS,
            MessageCodeType::Protective
            | MessageCodeType::Emergency
            | MessageCodeType::Danger
            | MessageCodeType::Status
            | MessageCodeType::Infrastructure
            | MessageCodeType::Mission
            | MessageCodeType::Request => sign::DEFINITIONS,
        }
    }

    pub fn get_message_code(code: &str) -> Self {
        Self::from_code(
            code.chars()
                .next()
                .unwrap_or_else(|| panic!("invalid message code: {}", code)),
        )
    }

    pub fn to_body(&self) -> MessageBodyType {
        match &self {
            MessageCodeType::Authentication => {
                MessageBodyType::AUTHENTICATION(Authentication::default())
            }
            _ => MessageBodyType::GENERIC,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageCodeType {
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

impl FromStr for MessageCodeType {
    type Err = super::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = Self::get_message_code(s); /* match s {
                                               "A" => MessageCodeType::Authentication,
                                               "K" => MessageCodeType::Cryptographic,
                                               "T" => MessageCodeType::Test,
                                               "R" => MessageCodeType::Resource,
                                               "F" => MessageCodeType::FreeText,
                                               "P" => MessageCodeType::Protective,
                                               "E" => MessageCodeType::Emergency,
                                               "D" => MessageCodeType::Danger,
                                               "S" => MessageCodeType::Status,
                                               "I" => MessageCodeType::Infrastructure,
                                               "M" => MessageCodeType::Mission,
                                               "Q" => MessageCodeType::Request,
                                               _ => MessageCodeType::Any,
                                           }; */

        Ok(t)
    }
}

impl ToString for MessageCodeType {
    fn to_string(&self) -> String {
        match &self {
            MessageCodeType::Any => "_",
            MessageCodeType::Authentication => "A",
            MessageCodeType::Cryptographic => "K",
            MessageCodeType::Test => "T",
            MessageCodeType::Resource => "R",
            MessageCodeType::FreeText => "F",
            MessageCodeType::Protective => "P",
            MessageCodeType::Emergency => "E",
            MessageCodeType::Danger => "D",
            MessageCodeType::Status => "S",
            MessageCodeType::Infrastructure => "I",
            MessageCodeType::Mission => "M",
            MessageCodeType::Request => "Q",
        }
        .to_string()
    }
}
