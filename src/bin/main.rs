use std::{fs::File, io::BufWriter};

use pibroxide::bmw::process_bmw_bar;
use pibroxide::parsers::lilypond::process_lily;
use pibroxide::writers::{
    bmw::{BMWWriter, write_bmw_file},
    lilypond::{LilyWriter, write_lily_file},
};

fn main() -> Result<(), std::io::Error> {
    let bar = "gg E_4 'e 'e	dbe Er_8	Cl_8	dg LAl_8";
    println!("{}", process_bmw_bar(bar));

    let tune = process_lily()?;
    let lily_out = File::create("music/outputs/atholl_highlanders_out.ly")?;
    let bmw_out = File::create("music/outputs/atholl_highlanders_out.bww")?;

    let lily_buf_writer = BufWriter::new(lily_out);
    let bmw_buf_writer = BufWriter::new(bmw_out);

    let mut lily_writer = LilyWriter {
        writer: lily_buf_writer,
    };
    let mut bmw_writer = BMWWriter {
        writer: bmw_buf_writer,
    };

    write_lily_file(&mut lily_writer, &tune)?;
    write_bmw_file(&mut bmw_writer, &tune)?;
    Ok(())
}
