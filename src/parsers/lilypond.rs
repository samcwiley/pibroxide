use crate::ir::internal_representation::{
    Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune, TuneType,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq)]
pub enum Group {
    Note,
    Embellishment,
}

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub fn process_lily() -> Result<Tune, std::io::Error> {
    let f = File::open("music/inputs/atholl_highlanders.ly")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();

    // skip everything until the actual tune starts
    for meta in lines.by_ref() {
        if meta? == "atholl_highlanders = {" {
            break;
        }
    }

    // getting time signature as a string
    let _time_signature = {
        if let Some(line) = lines.next() {
            let time_line = line?.trim().to_owned();
            if let Some(sig) = time_line.strip_prefix("\\time ") {
                sig.to_owned()
            } else {
                eprintln!("Could not parse time signature");
                String::new()
            }
        } else {
            eprintln!("Empty file");
            String::new()
        }
    };

    let mut parts = Vec::new();
    let mut bars = Vec::new();
    for line in lines {
        let line = line?;
        let line = line.trim();
        // start new part
        if line == "\\repeat volta 2 {" {
            bars = Vec::new();
        // end part
        } else if line == "}" && !bars.is_empty() {
            parts.push(Part { bars: bars.clone() });
            bars = Vec::new();
        // end tune
        } else if line == "}" {
            break;
        // music time
        } else if line == "\\break" {
        } else {
            bars.push(process_lily_bar(line));
        }
    }

    let tune = Tune {
        name: String::from("Atholl Highlanders"),
        parts,
        time_signature: TimeSignature::SixEight,
        tune_type: TuneType::Jig,
        composer: String::from("Trad."),
    };
    Ok(tune)
}

/// Processes a string slice into a lilypond bar
/// todo: get `time_signature` from tune
#[must_use]
pub fn process_lily_bar(line: &str) -> Measure {
    let line = line.replace(['[', ']', '|'], "");
    let mut notes = Vec::new();
    let mut embellishment = None;
    for token in line.split_ascii_whitespace() {
        if token.starts_with('\\') {
            embellishment = process_lily_embellishment(token);
        } else {
            let note = process_lily_note(token, embellishment);
            notes.push(note);
            embellishment = None;
        }
    }
    Measure {
        notes,
        time_signature: TimeSignature::SixEight,
    }
}

fn process_lily_embellishment(embellishment: &str) -> Option<Embellishment> {
    let parsed_embellishment = if let Some(grace_note_pitch) = embellishment.strip_prefix("\\gr")
        && grace_note_pitch.len() == 1
    {
        Embellishment::GraceNote(process_lily_pitch(grace_note_pitch.as_bytes()[0]))
    } else if let Some(doubling_pitch) = embellishment.strip_prefix("\\dbl") {
        Embellishment::Doubling(process_lily_pitch(doubling_pitch.as_bytes()[0]))
    } else if let Some(half_doubling_pitch) = embellishment.strip_prefix("\\hdbl") {
        Embellishment::HalfDoubling(process_lily_pitch(half_doubling_pitch.as_bytes()[0]))
    } else if let Some(thumb_doubling_pitch) = embellishment.strip_prefix("\\tdbl") {
        Embellishment::ThumbDoubling(process_lily_pitch(thumb_doubling_pitch.as_bytes()[0]))
    } else if let Some(slur_pitch) = embellishment.strip_prefix("\\slur") {
        Embellishment::Slur(process_lily_pitch(slur_pitch.as_bytes()[0]))
    } else if let Some(hornpipe_shake_pitch) = embellishment.strip_prefix("\\shake") {
        Embellishment::HornpipeShake(process_lily_pitch(hornpipe_shake_pitch.as_bytes()[0]))
    } else {
        match embellishment {
            "\\grip" => Embellishment::Grip,
            "\\bgrip" => Embellishment::BGrip,
            "\\taor" => Embellishment::Taorluath,
            "\\btaor" => Embellishment::BTaorluath,
            "\\Gtaor" => Embellishment::LGTaorluath,
            "\\thrwd" => Embellishment::ThrowD,
            "\\crun" => Embellishment::Crunluath,
            "\\dcrun" => Embellishment::BCrunluath,
            "\\Gcrun" => Embellishment::LGCrunluath,
            "\\dre" => Embellishment::Edre,
            "\\dare" => Embellishment::Dare,
            "\\dari" => Embellishment::Chedari,
            "\\bari" => Embellishment::Embari,
            "\\wbirl" => Embellishment::Birl,
            "\\birl" => Embellishment::Abirl,
            "\\gbirl" => Embellishment::Gbirl,
            "\\darodo" => Embellishment::Darodo,
            "\\catchc" => Embellishment::Hodro,
            "\\catchb" => Embellishment::Hiotro,
            _ => {
                eprintln!("Unrecognized embellishment {embellishment}");
                return None;
            }
        }
    };
    Some(parsed_embellishment)
}

fn process_lily_note(note: &str, embellishment: Option<Embellishment>) -> Note {
    let pitch = process_lily_pitch(note.as_bytes()[0]);
    let duration = match &note.as_bytes()[1..] {
        b"8" => Duration::Eighth,
        b"4" => Duration::Quarter,
        b"4." => Duration::DottedQuarter,
        b"8." => Duration::DottedEighth,
        b"16" => Duration::Sixteenth,
        b"16." => Duration::DottedSixteenth,
        b"32" => Duration::ThirtySecond,
        b"2" => Duration::Half,
        b"2." => Duration::DottedHalf,
        b"1" => Duration::Whole,
        b"1." => Duration::DottedWhole,
        _ => panic!("Unrecognized duration in note: {note}"),
    };
    Note {
        pitch,
        duration,
        embellishment,
    }
}

fn process_lily_pitch(pitch: u8) -> Pitch {
    match pitch {
        b'G' => Pitch::LowG,
        b'a' => Pitch::LowA,
        b'b' => Pitch::B,
        b'c' => Pitch::C,
        b'd' => Pitch::D,
        b'e' => Pitch::E,
        b'f' => Pitch::F,
        b'g' => Pitch::HighG,
        b'A' => Pitch::HighA,
        _ => panic!("Invalid Pitch found {pitch}"),
    }
}
