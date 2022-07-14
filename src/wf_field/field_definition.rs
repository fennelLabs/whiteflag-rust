use crate::wf_codec::encoding::*;
use regex::Regex;

#[derive(Clone)]
pub struct FieldDefinition {
    pub name: String,
    pattern: Option<Regex>,
    pub encoding: Encoding,
    pub start_byte: usize,
    pub end_byte: isize,
}

impl FieldDefinition {
    pub fn new(
        name: &str,
        pattern: Option<Regex>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: String::from(name),
            pattern, //: pattern.expect(&format!("invalid regular expression pattern: {}", name)),
            encoding,
            start_byte,
            end_byte,
        }
    }

    pub fn new2(
        name: &str,
        pattern: Result<Regex, regex::Error>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: String::from(name),
            pattern: pattern.ok(), //: pattern.expect(&format!("invalid regular expression pattern: {}", name)),
            encoding,
            start_byte,
            end_byte,
        }
    }

    pub fn get_minimum_starting_position(&self) -> usize {
        if self.end_byte < 0 {
            return self.start_byte;
        }

        self.end_byte as usize
    }

    /* pub fn get(&self, data: Vec<String>) -> WhiteflagResult<String> {
        if data.len() < self.get_minimum_starting_position() {
            return Err(WhiteflagError::InvalidLength);
        }

        data[self.start_byte..self.end_byte as usize]
            .first()
            .ok_or(WhiteflagError::InvalidLength)
    } */

    /**
     * Checks if the message field contains a valid value
     * @return TRUE if the field contains a valid value, else FALSE
     */
    pub fn is_valid<T: AsRef<str>>(&self, data: &Option<T>) -> bool {
        let value = match data {
            Some(x) => x,
            None => return false,
        };

        match self.pattern.as_ref() {
            Some(re) => re.is_match(value.as_ref()),
            None => true,
        }
    }
}
