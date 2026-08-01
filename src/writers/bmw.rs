use std::{fmt, io::Write};

use crate::{
    ir::internal_representation::{
        Beat, Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune,
    },
    writers::MusicWriter,
};

/// Wraps `BufWriter` with functions for writing `.bww` files
pub struct BMWWriter<W> {
    pub writer: W,
}

/// Enum for handling beam directions for beamed 8th notes, 16ths, etc.
#[derive(PartialEq, Debug)]
pub enum BeamSide {
    Left,
    Right,
}

/*
impl BeamSide {
    /// For quickly creating beams
    fn from(side: char) -> Self {
        match side {
            'l' => Self::Left,
            'r' => Self::Right,
            _ => panic!("Invalid beam side"),
        }
    }
}
*/

impl fmt::Display for BeamSide {
    /// Writes beams for use in beamed 8th notes, etc.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = match self {
            Self::Left => 'l',
            Self::Right => 'r',
        };
        write!(f, "{side}")
    }
}

struct BMWTimeSignature {
    pub time_signature: Option<TimeSignature>,
}

impl fmt::Display for BMWTimeSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match &self.time_signature {
            Some(time_signature) => match time_signature {
                TimeSignature::SixEight => "6_8",
            },
            None => "",
        };
        write!(f, "{out}")
    }
}

impl<W: Write> BMWWriter<W> {
    /// Handles writing a note with beaming or calls off to a standard, non-beamed note writer
    fn write_bmw_note(&mut self, note: Note, beam_side: Option<BeamSide>) -> std::io::Result<()> {
        if let Some(beam_side) = beam_side {
            let Note {
                pitch,
                duration,
                embellishment,
            } = note;
            let bmw_pitch = get_bmw_pitch(pitch);
            let bmw_duration = get_bmw_duration(duration, pitch);
            if let Some(embellishment) = embellishment {
                let bmw_embellishment = get_bmw_embellishment(embellishment);
                write!(
                    self.writer,
                    "{bmw_embellishment} {bmw_pitch}{beam_side}_{bmw_duration}"
                )?;
            } else {
                write!(self.writer, "{bmw_pitch}{beam_side}_{bmw_duration}")?;
            }

            Ok(())
        } else {
            self.write_note(note)
        }
    }

    fn write_key_time_signature(
        &mut self,
        time_signature: &BMWTimeSignature,
    ) -> std::io::Result<()> {
        write!(self.writer, "&  sharpf sharpc {time_signature}\t")
    }
}

#[must_use]
pub fn get_beams(beat: &Beat) -> Vec<Option<BeamSide>> {
    let num_notes = beat.notes.len();
    let mut beams = Vec::with_capacity(num_notes);
    let mut i = 0;
    let mut has_right = false;
    while i < num_notes {
        if !beat.notes[i].duration.is_beamed() {
            // if it's not a beamed note (e.g. 8th, 16th, 32, etc): none
            beams.push(None);
        } else if has_right {
            // if we already have a right beam, this one points left
            beams.push(Some(BeamSide::Left));
        } else if beat.notes.len() > i + 1 && beat.notes[i + 1].duration.is_beamed() {
            // if the next note wants a beam and this one also does, give it a
            // right, and set has_right to true
            beams.push(Some(BeamSide::Right));
            has_right = true;
        } else {
            // this means it's a flagged note but not next to another flagged
            // note, so no beam
            beams.push(None);
        }
        i += 1;
    }

    beams
}

impl<W: Write> MusicWriter for BMWWriter<W> {
    /// Writes note without beaming
    fn write_note(&mut self, note: Note) -> std::io::Result<()> {
        let Note {
            pitch,
            duration,
            embellishment,
        } = note;
        let bmw_pitch = get_bmw_pitch(pitch);
        let bmw_duration = get_bmw_duration(duration, pitch);
        if let Some(embellishment) = embellishment {
            let bmw_embellishment = get_bmw_embellishment(embellishment);
            write!(
                self.writer,
                "{bmw_embellishment} {bmw_pitch}_{bmw_duration}"
            )?;
        } else {
            write!(self.writer, "{bmw_pitch}_{bmw_duration}")?;
        }

        Ok(())
    }

    /// Writes out a full measure of notes; handles beaming logic
    fn write_measure(&mut self, measure: &Measure, _measure_number: usize) -> std::io::Result<()> {
        let beats = measure.get_beats();
        let note_beams_iter = beats.iter().map(get_beams);
        for (beat, note_beams) in beats.iter().zip(note_beams_iter) {
            for (note, beam_side) in beat.notes.iter().zip(note_beams) {
                self.write_bmw_note(*note, beam_side)?;
                write!(self.writer, " ")?;
            }
            write!(self.writer, "\t")?;
        }
        Ok(())
    }

    /// Writes a series of measures; handles barline logic
    fn write_part(
        &mut self,
        part: &Part,
        part_number: usize,
        time_signature: TimeSignature,
    ) -> std::io::Result<()> {
        for (measure_number, measure) in part.bars.iter().enumerate() {
            match measure_number {
                0 => {
                    let time_signature = if part_number == 0 {
                        Some(time_signature)
                    } else {
                        writeln!(self.writer)?;
                        None
                    };
                    let time_signature = BMWTimeSignature { time_signature };
                    self.write_key_time_signature(&time_signature)?;
                    write!(self.writer, "I!''\t")?;
                    self.write_measure(measure, measure_number)?;
                }
                4 => {
                    writeln!(self.writer)?;
                    self.write_key_time_signature(&BMWTimeSignature {
                        time_signature: None,
                    })?;
                    writeln!(self.writer)?;
                    write!(self.writer, "!\t")?;
                    self.write_measure(measure, measure_number)?;
                }
                3 => {
                    write!(self.writer, "!\t")?;
                    self.write_measure(measure, measure_number)?;
                    write!(self.writer, "!t")?;
                }
                7 => {
                    write!(self.writer, "!\t")?;
                    self.write_measure(measure, measure_number)?;
                    write!(self.writer, "''!I")?;
                }
                _ => {
                    write!(self.writer, "!\t")?;
                    self.write_measure(measure, measure_number)?;
                }
            }
            writeln!(self.writer)?;
        }
        Ok(())
    }

    /// Writes a series of parts along with requisite metadata used in BMW files
    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()> {
        for (part_number, part) in tune.parts.iter().enumerate() {
            self.write_part(part, part_number, tune.time_signature)?;
        }
        Ok(())
    }
}

/// Function for writing full bmw file
///
/// # Errors
///
/// This function returns Rsults from `write_tune` and `write!`
pub fn write_bmw_file<W: Write>(writer: &mut BMWWriter<W>, tune: &Tune) -> std::io::Result<()> {
    let pre_tune_junk = r"Bagpipe Reader:1.0

MIDINoteMappings,(54,56,58,59,61,63,64,66,68,56,58,60,61,63,65,66,68,70,55,57,59,60,62,64,65,67,69)

FrequencyMappings,(370,415,466,494,554,622,659,740,831,415,466,523,554,622,699,740,831,932,392,440,494,523,587,659,699,784,880)

InstrumentMappings,(71,71,45,33,1000,60,70)

GracenoteDurations,(20,40,30,50,100,200,800,1200,250,250,250,500,200)

FontSizes,(100,100,65,70,300)
";
    let tempo = 110usize;
    let tune_type = &tune.tune_type;
    let tune_name = &tune.name;
    let author = &tune.composer;

    let meta = format!(
        "TuneTempo,{tempo}\n\nTuneFormat,(1,1,F,L,500,500,500,500,L,0,0)\n\n\"{tune_name}\",(T,L,0,0,Times New Roman,16,700,0,0,18,0,0,0)\n\n\"{tune_type}\",(Y,C,0,0,Times New Roman,14,400,0,0,18,0,0,0)\n\n\"{author}\",(M,R,0,0,Times New Roman,14,400,0,0,18,0,0,0)\n\n\"\",(F,R,0,0,Times New Roman,10,400,0,0,18,0,0,0)\n\n"
    );
    writeln!(writer.writer, "{pre_tune_junk}")?;
    writeln!(writer.writer, "{meta}")?;
    writer.write_tune(tune)?;
    Ok(())
}

/// For getting display pitches used in notes
fn get_bmw_pitch(pitch: Pitch) -> &'static str {
    match pitch {
        Pitch::LowG => "LG",
        Pitch::LowA => "LA",
        Pitch::B => "B",
        Pitch::C => "C",
        Pitch::D => "D",
        Pitch::E => "E",
        Pitch::F => "F",
        Pitch::HighG => "HG",
        Pitch::HighA => "HA",
    }
}

/// For use in dots and embellishments
struct BMWLowercase {
    pitch: Pitch,
}

impl BMWLowercase {
    fn new(pitch: Pitch) -> Self {
        Self { pitch }
    }
}

/// for writing pitches in dots and embellishments
impl fmt::Display for BMWLowercase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_pitch = match self.pitch {
            Pitch::LowG => "lg",
            Pitch::LowA => "la",
            Pitch::B => "b",
            Pitch::C => "c",
            Pitch::D => "d",
            Pitch::E => "e",
            Pitch::F => "f",
            Pitch::HighG => "hg",
            Pitch::HighA => "ha",
        };
        write!(f, "{lowercase_pitch}")
    }
}

/// Used for writing dots in BMW files
struct Dot {
    pitch: Pitch,
}

impl Dot {
    /// Creates a dot on the given pitch
    fn new(pitch: Pitch) -> Self {
        Self { pitch }
    }
}

impl fmt::Display for Dot {
    /// Prepends a `'` symbol to the lowercase
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", BMWLowercase::new(self.pitch))
    }
}

/// For handling durations in BMW
struct BMWDuration {
    pub stem_value: u8,
    pub dot: Option<Dot>,
}

impl fmt::Display for BMWDuration {
    /// Writes a stemvalue, space, and dot on a the appropriate note
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let BMWDuration { stem_value, dot } = self;
        if let Some(dot) = dot {
            write!(f, "{stem_value} {dot}")
        } else {
            write!(f, "{stem_value}")
        }
    }
}

/// Takes in duration and pitch and returns a [`BMWDuration`] object containing a
/// stem value (2, 4, 8, etc) and an optional dot on the provided pitch
fn get_bmw_duration(duration: Duration, pitch: Pitch) -> BMWDuration {
    let (stem_value, dot) = match duration {
        Duration::ThirtySecond => (32, None),
        Duration::Sixteenth => (16, None),
        Duration::Eighth => (8, None),
        Duration::Quarter => (4, None),
        Duration::Half => (2, None),
        Duration::Whole => (1, None),
        Duration::DottedSixteenth => (16, Some(Dot::new(pitch))),
        Duration::DottedEighth => (8, Some(Dot::new(pitch))),
        Duration::DottedQuarter => (4, Some(Dot::new(pitch))),
        Duration::DottedHalf => (2, Some(Dot::new(pitch))),
        Duration::DottedWhole => (1, Some(Dot::new(pitch))),
    };
    BMWDuration { stem_value, dot }
}

/// For writing out embellishments in BMW. The embellishments marked `todo!()` I
/// could not find examples of in tunes
fn get_bmw_embellishment(embellishment: Embellishment) -> String {
    match embellishment {
        Embellishment::GraceNote(pitch) => match pitch {
            Pitch::LowG | Pitch::LowA | Pitch::C | Pitch::F => {
                format!("str{}", BMWLowercase::new(pitch))
            }
            Pitch::HighG => String::from("gg"),
            Pitch::D | Pitch::E | Pitch::B => format!("{}g", BMWLowercase::new(pitch)),
            Pitch::HighA => String::from("tg"),
        },
        Embellishment::Doubling(pitch) => format!("db{}", BMWLowercase::new(pitch)),
        Embellishment::HalfDoubling(pitch) => format!("hdb{}", BMWLowercase::new(pitch)),
        Embellishment::ThumbDoubling(pitch) => format!("tdb{}", BMWLowercase::new(pitch)),
        Embellishment::Slur(pitch) => {
            match pitch {
                Pitch::LowG => unimplemented!("This shouldn't be possible"),
                Pitch::LowA => todo!(),
                Pitch::B => String::from("gstb"),
                Pitch::C => todo!(),
                // todo!() light d strike
                Pitch::D => String::from("lgstd"),
                Pitch::E => todo!(),
                Pitch::F => todo!(),
                Pitch::HighG => todo!(),
                Pitch::HighA => String::from("dblha"),
            }
        }
        Embellishment::HornpipeShake(pitch) => format!("pel{}", BMWLowercase::new(pitch)),
        Embellishment::Grip => String::from("grp"),
        Embellishment::BGrip => String::from("grpb"),
        Embellishment::Taorluath => String::from("taor"),
        Embellishment::BTaorluath => String::from("taorb"),
        Embellishment::LGTaorluath => todo!(),
        Embellishment::ThrowD => String::from("thrd"),
        Embellishment::Crunluath => String::from("crunl"),
        Embellishment::BCrunluath => String::from("crunlb"),
        Embellishment::LGCrunluath => todo!(),
        Embellishment::HeavyCrunluath => todo!(),
        Embellishment::HeavyBCrunluath => todo!(),
        Embellishment::Edre => String::from("edre"),
        Embellishment::Dare => String::from("dare"),
        Embellishment::Chedari => todo!(),
        Embellishment::Embari => todo!(),
        Embellishment::Endari => todo!(),
        Embellishment::Birl => String::from("brl"),
        Embellishment::Abirl => String::from("abr"),
        Embellishment::Gbirl => String::from("gbr"),
        Embellishment::Darodo => String::from("bubly"),
        Embellishment::Hodro => String::from("ggrpc"),
        Embellishment::Hiotro => String::from("ggrpb"),
        Embellishment::Tie(pitch) => format!("^t{}", BMWLowercase::new(pitch)),
        Embellishment::LightDSlur => todo!(),
        Embellishment::ThumbSlur(pitch) => todo!(),
        Embellishment::HalfSlur(pitch) => todo!(),
        Embellishment::LightDThumbSlur => todo!(),
        Embellishment::LightDHalfSlur => todo!(),
        Embellishment::ThumbHornpipeShake(pitch) => todo!(),
        Embellishment::HalfHornpipeShake(pitch) => todo!(),
        Embellishment::LightDHornpipeShake => todo!(),
        Embellishment::LightDThumbHornpipeShake => todo!(),
        Embellishment::LightDHalfHornpipeShake => todo!(),
    }
}
