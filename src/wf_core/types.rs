use super::definitions::*;
use super::field::Field;
use super::segment::MessageSegment;

pub struct MessageType {
    message_code: char,
    pub headers: MessageSegment,
    pub body: MessageSegment,
}

impl MessageType {
    pub fn from_code_option(code: Option<&char>) -> MessageType {
        let c = code.unwrap_or(&' ');
        Self::from_code(c)
    }

    pub fn from_code(code: &char) -> MessageType {
        let body: Vec<Field> = match code {
            'A' => authentication_body_fields().to_vec(),
            'K' => crypto_body_fields().to_vec(),
            'T' => test_body_fields().to_vec(),
            'R' => resource_body_fields().to_vec(),
            'F' => freetext_body_fields().to_vec(),
            'P' | 'E' | 'D' | 'S' | 'I' | 'M' | 'Q' => sign_signal_body_fields().to_vec(),
            _ => Vec::<Field>::new(),
        };

        MessageType {
            message_code: *code,
            headers: MessageSegment::generic_header_segment(),
            body: MessageSegment::from(body),
        }
    }
}

pub enum MessageTypeEnum {
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
