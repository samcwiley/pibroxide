#[cfg(test)]
use {
    crate::{
        lilypond::process_lily_bar,
        writers::bmw::{self, BeamSide},
        writers::{
            MusicWriter,
            bmw::BMWWriter,
            lilypond::LilyWriter,
            lilypond::{self},
        },
    },
    std::io::BufWriter,
};

/// Test BMW Beaming
#[test]
fn test_bmw_beams() {
    let bar = process_lily_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let beats = bar.get_beats();
    assert_eq!(beats.len(), 2);
    let beams1 = bmw::get_beams(&beats[0]);
    let beams2 = bmw::get_beams(&beats[1]);
    assert_eq!(
        beams1,
        vec![
            Some(BeamSide::Right),
            Some(BeamSide::Left),
            Some(BeamSide::Left)
        ]
    );
    assert_eq!(
        beams2,
        vec![
            Some(BeamSide::Right),
            Some(BeamSide::Left),
            Some(BeamSide::Left)
        ]
    );

    let bar = process_lily_bar(r"\grg a4 \grd c8 \grg b4 \grd c8");
    let beats = bar.get_beats();
    assert_eq!(beats.len(), 2);
    let beams1 = bmw::get_beams(&beats[0]);
    let beams2 = bmw::get_beams(&beats[1]);
    assert_eq!(beams1, vec![None, None]);
    assert_eq!(beams2, vec![None, None]);
}
/// More lilypond beaming
#[test]
fn test_lily_beams() {
    let bar = process_lily_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let beats = bar.get_beats();
    let beams1 = lilypond::get_beams(&beats[0]);
    let beams2 = lilypond::get_beams(&beats[1]);
    assert_eq!(beams1, vec![Some((1, 2))]);
    assert_eq!(beams2, vec![Some((1, 2))]);
    let beats = process_lily_bar(r"\grg a4 \grd c8 \grg b4 \grd c8").get_beats();
    let beams1 = lilypond::get_beams(&beats[0]);
    let beams2 = lilypond::get_beams(&beats[1]);
    assert_eq!(beams1, Vec::new());
    assert_eq!(beams2, Vec::new());
}

#[test]
fn test_lily_out() {
    // this is a pretty tautological test but it's checking that the writer works as expected
    let bar = process_lily_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let writer = BufWriter::new(Vec::new());
    let mut lily_writer = LilyWriter { writer };
    lily_writer
        .write_measure(&bar, 0)
        .expect("Couldn't write in test");
    let vec_out = lily_writer.writer.into_inner().unwrap();
    assert_eq!(
        b"\t\t\\grg c8 [ e8 \\gra e8 ] \\grg d8 [ e8 \\gra e8 ] |\n",
        &vec_out[..]
    );
}

#[test]
fn test_bmw_out() {
    let bar = process_lily_bar(r"\grg c8 [e8 \gra e8] \grg d8 [e8 \gra e8]");
    let writer = BufWriter::new(Vec::new());
    let mut bmw_writer = BMWWriter { writer };
    bmw_writer
        .write_measure(&bar, 1)
        .expect("Couldn't write in test");
    let vec_out = bmw_writer.writer.into_inner().unwrap();
    assert_eq!(
        b"gg Cr_8 El_8 strla El_8 \tgg Dr_8 El_8 strla El_8 \t",
        &vec_out[..]
    );
}
