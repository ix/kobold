use serde_derive::Deserialize;
use std::{fs::read_to_string, path::Path, io::{Result, Error, ErrorKind}};
use core::result::Result as core;

use crate::unit::Unit;

#[derive(Deserialize, Debug)]
pub struct Layout {
    pub name: String,
    pub layers: Vec<Layer>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub rows: Vec<Row>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Row(Vec<Key>);

#[derive(Deserialize, Debug, Clone)]
pub struct Key {
    pub display: String,
    pub code: char,
    pub width: Unit
}

pub fn from_toml(path: &Path) -> Result<Layout> {
    let raw_toml = read_to_string(path)?;
    match toml::de::from_str::<Layout>(raw_toml.as_str()) {
        core::Ok(result)  => Ok(result),
        core::Err(e) => Err(Error::new(ErrorKind::Other, e.to_string()))
    }
}
