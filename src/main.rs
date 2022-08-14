use std::path::Path;

mod layout;
mod unit;

const PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/sample.toml");

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from(PATH));
    print!("Reading layout from {}... ", PATH);
    let layout = layout::from_toml(Path::new(&path));
    println!("DONE!");
    println!("{:?}", layout);
}
