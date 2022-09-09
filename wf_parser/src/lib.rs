use wf_field::{definitions, Field, FieldDefinition};

#[cfg(test)]
mod test;

pub fn convert_definitions<F>(
    defs: &'static [FieldDefinition],
    convert: F,
) -> impl Iterator<Item = Field>
where
    F: Fn((usize, &'static FieldDefinition)) -> Field,
{
    defs.iter().enumerate().map(convert)
}

pub fn convert_header_definitions<F>(convert: F) -> impl Iterator<Item = Field>
where
    F: Fn((usize, &'static FieldDefinition)) -> Field,
{
    convert_definitions(definitions::header::DEFINITIONS, convert)
}
