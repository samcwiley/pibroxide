#[cfg(test)]
use crate::{
    ir::internal_representation::{Duration, Embellishment, Pitch},
    lilypond::process_lily_bar,
};

#[test]
fn test_lily_notes() {
    let bar = process_lily_bar(r"\grg G32 [a32 b16] \grg c32 [d32 e16] \grg f32 [g32 A16] |");
    assert_eq!(bar.notes.len(), 9);
    let pitches = bar.notes.iter().map(|note| note.pitch).collect::<Vec<_>>();
    assert_eq!(
        pitches,
        vec![
            Pitch::LowG,
            Pitch::LowA,
            Pitch::B,
            Pitch::C,
            Pitch::D,
            Pitch::E,
            Pitch::F,
            Pitch::HighG,
            Pitch::HighA
        ]
    );

    let embellishments = bar
        .notes
        .iter()
        .map(|note| note.embellishment)
        .collect::<Vec<_>>();
    assert_eq!(
        embellishments[0..=2],
        vec![Some(Embellishment::GraceNote(Pitch::HighG)), None, None]
    );

    let durations = bar
        .notes
        .iter()
        .map(|note| note.duration)
        .collect::<Vec<_>>();
    assert_eq!(
        durations[0..=2],
        vec![
            Duration::ThirtySecond,
            Duration::ThirtySecond,
            Duration::Sixteenth
        ]
    );
}

#[cfg(test)]
use crate::{
    bmw::{process_bmw_bar, process_bmw_note},
    ir::internal_representation::Note,
};
#[test]
fn test_process_note() {
    let note = "LAr_4";
    let parsed = process_bmw_note(note, None).unwrap();
    assert_eq!(parsed, Note::default());
}

#[test]
fn test_bmw_bar() {
    let bar = process_bmw_bar("LA_4 'la grp HA_8 LGr_16 LAr_16 Bl_8");
    let pitches = bar.notes.iter().map(|note| note.pitch).collect::<Vec<_>>();
    assert_eq!(
        pitches,
        vec![
            Pitch::LowA,
            Pitch::HighA,
            Pitch::LowG,
            Pitch::LowA,
            Pitch::B
        ]
    );

    let durations = bar
        .notes
        .iter()
        .map(|note| note.duration)
        .collect::<Vec<_>>();
    assert_eq!(
        durations,
        vec![
            Duration::DottedQuarter,
            Duration::Eighth,
            Duration::Sixteenth,
            Duration::Sixteenth,
            Duration::Eighth,
        ]
    );

    let embellishments = bar
        .notes
        .iter()
        .map(|note| note.embellishment)
        .collect::<Vec<_>>();
    assert_eq!(
        embellishments,
        vec![None, Some(Embellishment::Grip), None, None, None]
    );
}
