#[cfg(test)]
mod test;

#[allow(dead_code)]
mod message_code_parser;
#[allow(dead_code)]
mod message_header_parser;
mod parsed_field_definition;

pub use message_code_parser::MessageCodeParser;
//pub use message_header_parser::MessageHeaderParser;

/// fields that are codes are single characters
pub fn convert_value_to_code(value: &str) -> char {
    value
        .chars()
        .nth(0)
        .unwrap_or_else(|| panic!("invalid message code"))
}
