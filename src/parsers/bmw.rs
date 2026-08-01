#![allow(unused_imports)]
use crate::ir::internal_representation::{
    Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune, TuneType,
};
use crate::utils::error::{NoteParseError, write_measure_parse_errors};
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

/// Processes a bmw bar
///
/// # Errors
///
/// This function will return an error if a note is bad
#[must_use]
pub fn process_bmw_bar(line: &str) -> Measure {
    let line = line.replace("!t", "").replace('!', "");
    let mut notes = Vec::new();
    let mut embellishment = None;
    let mut errors = Vec::new();
    for token in line.split_ascii_whitespace() {
        //let token = token.split('_').collect::<Vec<_>>();
        if token.contains('_') && token.split_terminator('_').collect::<Vec<_>>().len() == 2 {
            let note_result = process_bmw_note(token, embellishment);
            match note_result {
                Ok(note) => notes.push(note),
                Err(err) => errors.push(err),
            }
            embellishment = None;
        } else if token.starts_with('\'')
            && let Some(last) = notes.last_mut()
        {
            let result = last.add_dot();
            match result {
                Ok(_) => (),
                Err(err) => errors.push(NoteParseError::DotError(err)),
            }
        } else {
            match process_bmw_embellishment(token) {
                Ok(emb) => embellishment = Some(emb),
                Err(err) => errors.push(err),
            }
        }
    }

    if !errors.is_empty() {
        write_measure_parse_errors(5, &errors);
    }

    Measure {
        notes,
        time_signature: TimeSignature::SixEight,
    }
}

fn process_bmw_embellishment(embellishment: &str) -> Result<Embellishment, NoteParseError> {
    let bmw_embellishment = if embellishment.len() == 2
        && let Some(grace_note_pitch) = embellishment.strip_suffix('g')
    {
        Embellishment::GraceNote(process_bmw_embellishment_pitch(grace_note_pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("str") {
        Embellishment::GraceNote(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("db") {
        Embellishment::Doubling(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("tdb") {
        Embellishment::ThumbDoubling(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("hdb") {
        Embellishment::HalfDoubling(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("gst") {
        Embellishment::Slur(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("tst") {
        Embellishment::ThumbSlur(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("hst") {
        Embellishment::HalfSlur(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("pel") {
        Embellishment::HornpipeShake(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("tpel") {
        Embellishment::ThumbHornpipeShake(process_bmw_embellishment_pitch(pitch)?)
    } else if let Some(pitch) = embellishment.strip_prefix("hpel") {
        Embellishment::HalfHornpipeShake(process_bmw_embellishment_pitch(pitch)?)
    } else {
        match embellishment {
            "lgstd" => Embellishment::LightDSlur,
            "ltstd" => Embellishment::LightDThumbSlur,
            "lhstd" => Embellishment::LightDHalfSlur,
            "lpeld" => Embellishment::LightDHornpipeShake,
            "tpeld" => Embellishment::LightDThumbHornpipeShake,
            "lhpeld" => Embellishment::LightDHalfHornpipeShake,
            "grp" => Embellishment::Grip,
            "grpb" => Embellishment::BGrip,
            "taor" => Embellishment::Taorluath,
            "taorb" => Embellishment::BTaorluath,
            "thrwd" => Embellishment::ThrowD,
            "crunl" => Embellishment::Crunluath,
            "crunlb" => Embellishment::BCrunluath,
            "edre" => Embellishment::Edre,
            "dare" => Embellishment::Dare,
            "ggrpc" => Embellishment::Hodro,
            "ggrpb" => Embellishment::Hiotro,
            "gbr" => Embellishment::Gbirl,
            "brl" => Embellishment::Birl,
            "abr" => Embellishment::Abirl,
            "bubly" => Embellishment::Darodo,

            _ => {
                return Err(NoteParseError::UnrecognizedEmbellishment(
                    embellishment.to_string(),
                ));
            }
        }
    };
    Ok(bmw_embellishment)
}

/// Processes a bmw note
///
/// # Errors
///
/// This function will return an error if it encounters an unrecognized pitch or
/// duration
pub fn process_bmw_note(
    note: &str,
    embellishment: Option<Embellishment>,
) -> Result<Note, NoteParseError> {
    let mut split = note.split_terminator('_');
    let pitch = if let Some(pitch_part) = split.next() {
        process_bmw_pitch(&pitch_part.replace(['l', 'r'], ""))?
    } else {
        return Err(NoteParseError::Custom(format!(
            "Could not parse {note} for a pitch"
        )));
    };
    let duration = if let Some(duration_part) = split.next() {
        process_bmw_duration(duration_part)?
    } else {
        return Err(NoteParseError::Custom(format!(
            "Could not parse {note} for duration"
        )));
    };

    Ok(Note {
        pitch,
        duration,
        embellishment,
    })
}

fn process_bmw_duration(duration: &str) -> Result<Duration, NoteParseError> {
    let value = match duration {
        "1" => Duration::Whole,
        "2" => Duration::Half,
        "4" => Duration::Quarter,
        "8" => Duration::Eighth,
        "16" => Duration::Sixteenth,
        "32" => Duration::ThirtySecond,
        _ => return Err(NoteParseError::InvalidDuration(duration.to_string())),
    };
    Ok(value)
}

fn process_bmw_pitch(pitch: &str) -> Result<Pitch, NoteParseError> {
    let bmw = match pitch {
        "LG" => Pitch::LowG,
        "LA" => Pitch::LowA,
        "B" => Pitch::B,
        "C" => Pitch::C,
        "D" => Pitch::D,
        "E" => Pitch::E,
        "F" => Pitch::F,
        "HG" => Pitch::HighG,
        "HA" => Pitch::HighA,
        _ => {
            return Err(NoteParseError::UnrecognizedPitch(pitch.to_string()));
        }
    };
    Ok(bmw)
}

fn process_bmw_embellishment_pitch(pitch: &str) -> Result<Pitch, NoteParseError> {
    let emb_pitch = match pitch {
        "lg" => Pitch::LowG,
        "a" | "la" => Pitch::LowA, // `a` used in single grace notes, since high a is `tg`
        "b" => Pitch::B,
        "c" => Pitch::C,
        "d" => Pitch::D,
        "e" => Pitch::E,
        "f" => Pitch::F,
        "g" => Pitch::HighG, // used in single grace note, since low g isnot implemented
        "ha" | "t" => Pitch::HighA, // `t` for thumb
        _ => return Err(NoteParseError::UnrecognizedPitch(String::from(pitch))),
    };
    Ok(emb_pitch)
}
