use std::io::Write;

use crate::{
    ir::internal_representation::{
        Beat, Duration, Embellishment, Measure, Note, Part, Pitch, TimeSignature, Tune,
    },
    writers::MusicWriter,
};

pub struct LilyWriter<W> {
    pub writer: W,
}

impl<W: Write> MusicWriter for LilyWriter<W> {
    fn write_note(&mut self, note: Note) -> std::io::Result<()> {
        let Note {
            pitch,
            duration,
            embellishment,
        } = note;
        if let Some(embellishment) = embellishment {
            write!(
                self.writer,
                "{} {}{} ",
                get_lily_embellishment(embellishment),
                get_lily_pitch(pitch),
                get_lily_duration(duration)
            )?;
        } else {
            write!(
                self.writer,
                "{}{} ",
                get_lily_pitch(pitch),
                get_lily_duration(duration)
            )?;
        }
        Ok(())
    }

    fn write_measure(&mut self, measure: &Measure, _measure_number: usize) -> std::io::Result<()> {
        let beats: Vec<Beat> = measure.get_beats();
        let note_beams_vec = beats
            .iter()
            .map(get_beams)
            .collect::<Vec<Vec<Option<(usize, usize)>>>>();
        write!(self.writer, "\t\t")?;
        for (beat, beams) in beats.iter().zip(note_beams_vec.iter()) {
            // if no beams, just write the notes normally
            if beams.is_empty() {
                for note in &beat.notes {
                    self.write_note(*note)?;
                }
            } else {
                let mut beam_iter = beams.iter();
                let mut current = beam_iter.next();

                for (j, note) in beat.notes.iter().enumerate() {
                    if let Some(Some((start, _))) = current
                        && j == *start
                    {
                        write!(self.writer, "[ ")?;
                    }

                    self.write_note(*note)?;

                    if let Some(Some((_, end))) = current
                        && j == *end
                    {
                        write!(self.writer, "] ")?;
                        current = beam_iter.next();
                    }
                }
            }
        }

        writeln!(self.writer, "|")?;
        Ok(())
    }

    fn write_part(
        &mut self,
        part: &Part,
        _part_number: usize,
        _time_signature: TimeSignature,
    ) -> std::io::Result<()> {
        writeln!(self.writer, "\t\\repeat volta 2 {{")?;
        for (measure_number, measure) in part.bars.iter().enumerate() {
            self.write_measure(measure, measure_number)?;
            if measure_number == 3 || measure_number == 7 {
                writeln!(self.writer, "\t\t\\break")?;
            }
        }
        writeln!(self.writer, "\t}}")?;
        Ok(())
    }

    fn write_tune(&mut self, tune: &Tune) -> std::io::Result<()> {
        let internal_name = tune.name.to_ascii_lowercase().replace(' ', "_");
        writeln!(self.writer, "{internal_name} = {{")?;
        writeln!(self.writer, "\t\\time 6/8")?;
        for (part_number, part) in tune.parts.iter().enumerate() {
            self.write_part(part, part_number, tune.time_signature)?;
        }
        writeln!(self.writer, "}}")?;
        Ok(())
    }
}

/// Function for writing a full lilypond file
///
/// # Errors
///
/// This function returns errors from `write_tune` and the `write!` macro
pub fn write_lily_file<W: Write>(writer: &mut LilyWriter<W>, tune: &Tune) -> std::io::Result<()> {
    let internal_name = tune.name.clone().to_ascii_lowercase().replace(' ', "_");
    let pre_tune_junk = r#"\version "2.25.21"

\include "bagpipe.ly" 

\include "../includes/scw_bagpipe.ly"
\include "../includes/score_settings.ly"

source = "COMPOSER"

#(allow-volta-hook "|")


voltaTwo = \markup  { \hspace #20 \italic \fontsize #+5 { "2" }  }
    
"#
    .replace("COMPOSER", &tune.composer);
    write!(writer.writer, "{pre_tune_junk}")?;
    writer.write_tune(tune)?;
    let meta = r#"

\header { 
  title = \markup  \override #'(line-width . 82) 
  { 
    \column {  
      \center-align {
        \line { 
          TUNENAME
        }
      }
    }
  }
                  
  subtitle = ""
  composer = "COMPOSER"
  arranger = ""
  meter = "" 
}    
"#
    .replace("TUNENAME", &tune.name)
    .replace("COMPOSER", &tune.composer);
    write!(writer.writer, "{meta}")?;

    let post_tune_junk = r#"

\paper {
	#(set-paper-size "letter" 'portrait)
}

\score {
	\new GrandStaff <<
		\new Staff = "GHB" <<
			\new Voice {
			        \global
				\tune_name
			}
		>>		
	>>
        \layout { \ScoreLayout 
                  \context { 
                             \Score
                             \override SpacingSpanner.base-shortest-duration = #(ly:make-moment 1/2) 
                           }            
                }
          
                \header{
        }
}
"#.replace("tune_name", &internal_name);
    write!(writer.writer, "{post_tune_junk}")?;
    Ok(())
}

#[must_use]
pub fn get_beams(beat: &Beat) -> Vec<Option<(usize, usize)>> {
    let num_notes = beat.notes.len();
    let mut beams = Vec::new();
    let mut left_end: Option<usize> = None;

    let mut i = 0;
    while i < num_notes {
        let is_beamed = beat.notes[i].duration.is_beamed();

        if left_end.is_none() && is_beamed {
            // if we don't have a left end yet, and we come across a beamed note, we
            // make a left end
            left_end = Some(i);
        } else if let Some(left) = left_end
            && !is_beamed
        {
            // if we do have a left end and we come across a beamed note, make a
            // right end
            if i > left + 1 {
                beams.push(Some((left + 1, i)));
            }
            left_end = None;
        } else if beat.notes.len() > i + 1 && beat.notes[i + 1].duration.is_beamed() {
            // if the next note exists and is beamed, do nothing
        } else if let Some(left) = left_end {
            // we have gotten to the end of the measure
            if i > left + 1 {
                beams.push(Some((left + 1, i)));
            }
        } else {
            beams.push(None);
        }

        i += 1;
    }

    beams
}

fn get_lily_pitch(pitch: Pitch) -> &'static str {
    match pitch {
        Pitch::LowG => "G",
        Pitch::LowA => "a",
        Pitch::B => "b",
        Pitch::C => "c",
        Pitch::D => "d",
        Pitch::E => "e",
        Pitch::F => "f",
        Pitch::HighG => "g",
        Pitch::HighA => "A",
    }
}

fn get_lily_duration(duration: Duration) -> &'static str {
    match duration {
        Duration::ThirtySecond => "32",
        Duration::Sixteenth => "16",
        Duration::Eighth => "8",
        Duration::Quarter => "4",
        Duration::Half => "2",
        Duration::Whole => "1",
        Duration::DottedSixteenth => "16.",
        Duration::DottedEighth => "8.",
        Duration::DottedQuarter => "4.",
        Duration::DottedHalf => "2.",
        Duration::DottedWhole => "1.",
    }
}

fn get_lily_embellishment(embellishment: Embellishment) -> String {
    // todo: add half shakes, remove tdblA, light/heavy shakes, light/heavy
    // slurs, add half shakes, remove low g shakes and slurs, add half slurs,
    // add catches, add half catches, add thumb catches, add low g d throw, add
    // heavy d throw, add dbirl, add hadeda (e grip), add redundant taorluath,
    // add taorluath a mach,, etc.
    match embellishment {
        Embellishment::GraceNote(pitch) => format!("\\gr{}", get_lily_pitch(pitch)),
        Embellishment::Doubling(pitch) => format!("\\dbl{}", get_lily_pitch(pitch)),
        Embellishment::HalfDoubling(pitch) => format!("\\hdbl{}", get_lily_pitch(pitch)),
        Embellishment::ThumbDoubling(pitch) => format!("\\tdbl{}", get_lily_pitch(pitch)),
        Embellishment::Slur(pitch) => format!("\\slur{}", get_lily_pitch(pitch)),
        Embellishment::HornpipeShake(pitch) => format!("\\shake{}", get_lily_pitch(pitch)),
        Embellishment::Grip => String::from("\\grip"),
        Embellishment::BGrip => String::from("\\bgrip"),
        Embellishment::Taorluath => String::from("\\taor"),
        Embellishment::BTaorluath => String::from("\\btaor"),
        Embellishment::LGTaorluath => String::from("\\Gtaor"),
        Embellishment::ThrowD => String::from("thrwd"),
        Embellishment::Crunluath => String::from("\\crun"),
        Embellishment::BCrunluath => String::from("\\dcrun"),
        Embellishment::LGCrunluath => String::from("\\Gcrun"),
        Embellishment::HeavyCrunluath => String::from("\\pgrace { G32[ d G e G f G }"),
        Embellishment::HeavyBCrunluath => String::from("\\pgrace { G32[ b G e G f G] }"),
        // took me a minute to realize these had the same grace notes
        Embellishment::Edre | Embellishment::Endari => String::from("\\dre"),
        Embellishment::Dare => String::from("\\dare"),
        Embellishment::Chedari => String::from("\\dari"),
        Embellishment::Embari => String::from("\\bari"),
        Embellishment::Birl => String::from("\\wbirl"),
        Embellishment::Abirl => String::from("\\birl"),
        Embellishment::Gbirl => String::from("\\gbirl"),
        Embellishment::Darodo => String::from("\\darodo"),
        Embellishment::Hodro => String::from("\\catchc"),
        Embellishment::Hiotro => String::from("\\catchb"),
        Embellishment::Tie(_) => String::from("~"),
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
