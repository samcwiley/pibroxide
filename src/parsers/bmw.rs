#![allow(unused_imports)]
use crate::ir::internal_representation::{
    Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune, TuneType,
};
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

#[must_use]
pub fn process_bar(line: &str) -> Measure {
    let mut notes = Vec::new();
    //let mut embellishment = None;

    Measure {
        notes,
        time_signature: TimeSignature::SixEight,
    }
}

fn process_bmw_embellishment(embellishment: &str) -> Result<Option<Embellishment>, std::io::Error> {
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
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid Pitch: {embellishment}"),
            ));
        }
    };
    Ok(Some(bmw_embellishment))
}

fn process_bmw_pitch(pitch: &str) -> Result<Pitch, std::io::Error> {
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
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid Pitch: {pitch}"),
            ));
        }
    };
    Ok(bmw)
}
