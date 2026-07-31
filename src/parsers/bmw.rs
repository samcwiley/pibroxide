#![allow(unused_imports)]
use crate::ir::internal_representation::{
    Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune, TuneType,
};
use crate::utils::error::{NoteParseError, PitchParseError};
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

#[must_use]
pub fn process_bmw_bar(line: &str) -> Result<Measure, NoteParseError> {
    let line = line.replace("!", ""); // need to figure out how to do multi replace with strings
    let mut notes = Vec::new();
    let mut embellishment = None;
    for token in line.split_ascii_whitespace() {
        let token = token.split('_').collect::<Vec<_>>();
        if token.len() == 2 {
            let note = process_bmw_note(token, embellishment)?;
            notes.push(note);
            embellishment = None;
        } else if let Some(found_embellishment) = process_bmw_embellishment(token[0])? {
            embellishment = Some(found_embellishment);
        } else if token[0].starts_with('\'')
            && let Some(last) = notes.last_mut()
        {
            match last.duration.add_dot() {
                Ok(_) => continue,
                Err(err) => eprintln!("{err}"),
            }
        }
    }

    Ok(Measure {
        notes,
        time_signature: TimeSignature::SixEight,
    })
}

fn process_bmw_embellishment(embellishment: &str) -> Result<Option<Embellishment>, NoteParseError> {
    let bmw_embellishment = match embellishment {
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
            return Err(NoteParseError);
        }
    };
    Ok(Some(bmw_embellishment))
}

fn process_bmw_note(
    note: Vec<&str>,
    embellishment: Option<Embellishment>,
) -> Result<Note, NoteParseError> {
    let pitch = process_bmw_pitch(&note[0].replace(['l', 'r'], ""))?;
    let duration = process_bmw_duration(note[1])?;
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
        _ => return Err(NoteParseError),
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
            return Err(NoteParseError);
        }
    };
    Ok(bmw)
}
