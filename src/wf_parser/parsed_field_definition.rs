use crate::{
    wf_buffer::WhiteflagBuffer,
    wf_field::{definitions, FieldDefinition},
};

pub struct ParsedFieldDefinition {
    definition: &'static FieldDefinition,
    start_bit: usize,
    end_bit: usize,
    //index: usize,
}

impl std::ops::Deref for ParsedFieldDefinition {
    type Target = FieldDefinition;

    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}

impl From<ParsedFieldDefinition> for &'static FieldDefinition {
    fn from(p: ParsedFieldDefinition) -> Self {
        p.definition
    }
}

impl ParsedFieldDefinition {
    /// creates the `ParsedFieldDefinition` that is ordered after this one
    pub fn next(&self, next: &'static FieldDefinition) -> Self {
        ParsedFieldDefinition::new(self.end_bit, next)
    }

    pub fn new(previous: usize, current: &'static FieldDefinition) -> Self {
        let start_bit = previous;
        let end_bit = start_bit + current.bit_length();
        ParsedFieldDefinition {
            definition: current,
            start_bit,
            end_bit,
            //index: current.index + 1,
        }
    }

    pub fn parse(defs: &'static [FieldDefinition], start: usize) -> Vec<ParsedFieldDefinition> {
        let mut previous = start;
        defs.iter()
            .map(|d| {
                let p = ParsedFieldDefinition::new(previous, d);
                previous = p.end_bit;
                p
            })
            .collect()
    }

    pub fn header() -> Vec<ParsedFieldDefinition> {
        Self::parse(definitions::Header::DEFINITIONS, 0)
    }

    pub fn extract(&self, buffer: &WhiteflagBuffer) -> String {
        buffer.extract_message_value(&self.definition, self.start_bit)
    }
}
