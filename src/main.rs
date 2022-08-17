mod layout;
mod unit;

use layout::Layout;
use termion::cursor::HideCursor;
use termion::raw::IntoRawMode;

use std::path::Path;
use std::io::{Result as IO, stdout, stdin, Write};

use termion::{input::*, cursor::Goto, event::*};

const PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/sample.toml");

fn main () -> IO<()> {
    let path = std::env::args().skip(1).next().unwrap_or(String::from(PATH));
    let layout = verbosely_load(&path)?;
    event_loop(vec![layout])
}

/// Load a layout from a file, using stdout to indicate progress.
fn verbosely_load(path: &String) -> IO<Layout> {
    print!("Reading layout from {}... ", PATH);
    let layout = layout::from_toml(Path::new(path))?;
    println!("DONE!");
    Ok(layout)
}

/// The program's input-render loop.
fn event_loop (layouts: Vec<Layout>) -> IO<()> {
    // set up mouse reporting from stdout, and use raw mode to avoid line-buffering
    let mut stdout = HideCursor::from(MouseTerminal::from(stdout().into_raw_mode()?));
    write!(stdout, "{}{}{}", termion::clear::All, Goto(1, 1), layouts.first().map_or("TEST", |l| &l.name))?;
    stdout.flush()?;
    for event in stdin().events() {
        match event? {
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
    }
    Ok(())
}