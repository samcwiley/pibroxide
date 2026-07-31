use std::fmt::Display;

#[derive(Debug)]
pub enum NoteParseError {
    UnrecognizedEmbellishment(String),
    UnrecognizedPitch(String),
    InvalidDuration(String),
    Custom(String),
}

impl Display for NoteParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteParseError::UnrecognizedEmbellishment(embellishment) => {
                write!(f, "Unrecognized embellishment: {embellishment}")
            }
            NoteParseError::UnrecognizedPitch(pitch) => write!(f, "Unrecognized pitch: {pitch}"),
            NoteParseError::InvalidDuration(duration) => {
                write!(f, "Unrecognized duration: {duration}")
            }
            NoteParseError::Custom(message) => write!(f, "{message}"),
        }
    }
}

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

pub fn write_measure_parse_errors(measure_num: usize, errors: &[NoteParseError]) {
    let errors = errors
        .iter()
        .map(|error| format!("{error}"))
        .fold(String::new(), |a, b| a + ", " + &b);
    eprintln!("Parsing errors found in measure {measure_num}: {errors}");
}
