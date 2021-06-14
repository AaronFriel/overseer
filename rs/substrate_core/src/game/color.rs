use std::{
  convert::TryFrom,
  fmt::{self, Display, Write},
};

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::game::{ObjectColor, ObjectColored};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum Color {
  W,
  U,
  B,
  R,
  G,
}
pub use Color::*;

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Color::W => f.write_char('W'),
      Color::U => f.write_char('U'),
      Color::B => f.write_char('B'),
      Color::R => f.write_char('R'),
      Color::G => f.write_char('G'),
    }
  }
}

impl TryFrom<&str> for Color {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "W" => Ok(Color::W),
      "U" => Ok(Color::U),
      "B" => Ok(Color::B),
      "R" => Ok(Color::R),
      "G" => Ok(Color::G),
      _ => Err("Expected a single character of W, U, B, R, or G"),
    }
  }
}

impl ObjectColored for &Color {
  fn get_object_color(self) -> ObjectColor {
    match self {
      W => ObjectColor::W,
      U => ObjectColor::U,
      B => ObjectColor::B,
      R => ObjectColor::R,
      G => ObjectColor::G,
    }
  }
}
