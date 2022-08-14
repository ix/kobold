use std::fmt::{Debug, Formatter};

use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Unit(f32);

impl Debug for Unit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}u", self.0)
    }
}