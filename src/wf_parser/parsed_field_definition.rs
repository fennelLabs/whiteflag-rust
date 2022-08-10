use crate::{wf_buffer::WhiteflagBuffer, wf_field::FieldDefinition};

pub struct ParsedFieldDefinition {
    definition: FieldDefinition,
    start_bit: usize,
    end_bit: usize,
}

impl std::ops::Deref for ParsedFieldDefinition {
    type Target = FieldDefinition;

    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}

impl From<ParsedFieldDefinition> for FieldDefinition {
    fn from(p: ParsedFieldDefinition) -> Self {
        p.definition
    }
}

impl ParsedFieldDefinition {
    /// creates the `ParsedFieldDefinition` that is ordered after this one
    pub fn next(&self, next: FieldDefinition) -> Self {
        ParsedFieldDefinition::new(self.end_bit, next)
    }

    pub fn new(previous: usize, current: FieldDefinition) -> Self {
        let start_bit = previous;
        let end_bit = start_bit + current.bit_length();
        ParsedFieldDefinition {
            definition: current,
            start_bit,
            end_bit,
        }
    }

    pub fn parse(defs: Vec<FieldDefinition>) -> Vec<ParsedFieldDefinition> {
        let mut previous = 0;
        defs.into_iter()
            .map(|d| {
                let p = ParsedFieldDefinition::new(previous, d);
                previous = p.end_bit;
                p
            })
            .collect()
    }

    pub fn extract(&self, buffer: &WhiteflagBuffer) -> String {
        buffer.extract_message_value(&self.definition, self.start_bit)
    }
}
