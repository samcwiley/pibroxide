use std::fmt::Display;

#[derive(Debug)]
pub struct NoteParseError;

#[derive(Debug)]
pub struct PitchParseError;

#[derive(Debug)]
pub enum DotError {
    DoubleDot,
    DotThirtySecond,
}

impl Display for DotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            DotError::DoubleDot => "Double dots not currently supported",
            DotError::DotThirtySecond => "Dotted 32nd notes not currently supported",
        };
        write!(f, "{out}")
    }
}
