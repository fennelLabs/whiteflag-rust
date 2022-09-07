use crate::{definitions, FieldDefinition, FieldValue};

pub struct ParsedFieldDefinition {
    definition: &'static FieldDefinition,
    pub start_bit: usize,
    pub end_bit: usize,
    index: usize,
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
        ParsedFieldDefinition::new(self.index, self.end_bit, next)
    }

    pub fn new(index: usize, previous_end_bit: usize, current: &'static FieldDefinition) -> Self {
        let start_bit = previous_end_bit;
        let end_bit = start_bit + current.bit_length();
        ParsedFieldDefinition {
            definition: current,
            start_bit,
            end_bit,
            index,
        }
    }

    pub fn parse(defs: &'static [FieldDefinition], start: usize) -> Vec<ParsedFieldDefinition> {
        let mut previous = start;
        defs.iter()
            .enumerate()
            .map(|(i, d)| {
                let p = ParsedFieldDefinition::new(i, previous, d);
                previous = p.end_bit;
                p
            })
            .collect()
    }

    pub fn header() -> Vec<ParsedFieldDefinition> {
        Self::parse(definitions::header::DEFINITIONS, 0)
    }

    /// used in the compiling process
    pub fn read_from_values<'a, T: FieldValue>(&self, values: &'a [T]) -> &'a str {
        values[self.index].as_ref()
    }

    /// used in the deserializing process
    pub fn read_from_serialized<'a>(&self, message: &'a str) -> &'a str {
        match self.positions.bytes.end {
            Some(e) => &message[self.positions.bytes.start..e],
            None => &message[self.positions.bytes.start..],
        }
    }

    pub fn to_definition(self) -> FieldDefinition {
        self.definition.to_owned()
    }
}
