use std::{
  borrow::{Borrow, Cow},
  convert::{TryFrom, TryInto},
  fmt::{self, Display, Write},
  iter::FromIterator,
  usize,
};

use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};
use serde_diff::SerdeDiff;

use crate::game::{Color, ObjectColor, ObjectColored};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[derive(SerdeDiff)]
#[serde_diff(opaque)]
pub enum ManaCostPip {
  X,
  Generic(u8),
  Colorless,
  Hybrid(HybridColor),
  Color(Color),
  MonoHybrid(Color),
  Phyrexian(Color),
  Snow,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HybridColor {
  WU,
  WB,
  UB,
  UR,
  BR,
  BG,
  RG,
  RW,
  GW,
  GU,
}

impl ObjectColored for &HybridColor {
  fn get_object_color(self) -> ObjectColor {
    match self {
      HybridColor::WU => ObjectColor::WU,
      HybridColor::WB => ObjectColor::WB,
      HybridColor::UB => ObjectColor::UB,
      HybridColor::UR => ObjectColor::UR,
      HybridColor::BR => ObjectColor::BR,
      HybridColor::BG => ObjectColor::BG,
      HybridColor::RG => ObjectColor::RG,
      HybridColor::RW => ObjectColor::RW,
      HybridColor::GW => ObjectColor::GW,
      HybridColor::GU => ObjectColor::GU,
    }
  }
}

impl HybridColor {
  fn try_from_impl(first_color: Color, second_color: Color) -> Result<HybridColor, String> {
    Ok(match (first_color, second_color) {
      (Color::W, Color::U) => HybridColor::WU,
      (Color::W, Color::B) => HybridColor::WB,
      (Color::U, Color::B) => HybridColor::UB,
      (Color::U, Color::R) => HybridColor::UR,
      (Color::B, Color::R) => HybridColor::BR,
      (Color::B, Color::G) => HybridColor::BG,
      (Color::R, Color::G) => HybridColor::RG,
      (Color::R, Color::W) => HybridColor::RW,
      (Color::G, Color::W) => HybridColor::GW,
      (Color::G, Color::U) => HybridColor::GU,
      (a, b) if a == b => Err(format!(
        "Invalid hybrid color pair {}/{}, did you mean to use the same color twice?",
        a, b
      ))?,
      _ => Err(format!(
        "Invalid hybrid color pair {}/{}, did you mean {}/{}",
        first_color, second_color, second_color, first_color
      ))?,
    })
  }
}

impl TryFrom<(Color, Color)> for HybridColor {
  type Error = String;

  fn try_from((first_color, second_color): (Color, Color)) -> Result<Self, Self::Error> {
    HybridColor::try_from_impl(first_color, second_color)
  }
}

impl From<HybridColor> for (Color, Color) {
  fn from(c: HybridColor) -> Self {
    match c {
      HybridColor::WU => (Color::W, Color::U),
      HybridColor::WB => (Color::W, Color::B),
      HybridColor::UB => (Color::U, Color::B),
      HybridColor::UR => (Color::U, Color::R),
      HybridColor::BR => (Color::B, Color::R),
      HybridColor::BG => (Color::B, Color::G),
      HybridColor::RG => (Color::R, Color::G),
      HybridColor::RW => (Color::R, Color::W),
      HybridColor::GW => (Color::G, Color::W),
      HybridColor::GU => (Color::G, Color::U),
    }
  }
}

impl ManaCostPip {
  pub fn as_generic(&self) -> Option<u8> {
    if let Self::Generic(v) = self {
      Some(*v)
    } else {
      None
    }
  }
}

impl Display for ManaCostPip {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('{')?;
    match self {
      ManaCostPip::Color(c) => c.fmt(f)?,
      ManaCostPip::Colorless => f.write_char('C')?,
      ManaCostPip::Generic(n) => n.fmt(f)?,
      ManaCostPip::X => f.write_char('X')?,
      ManaCostPip::Hybrid(h) => {
        let (fst, snd) = HybridColor::into(*h);
        fst.fmt(f)?;
        f.write_char('/')?;
        snd.fmt(f)?;
      }
      ManaCostPip::MonoHybrid(c) => {
        f.write_str("2/")?;
        c.fmt(f)?;
      }
      ManaCostPip::Phyrexian(c) => {
        c.fmt(f)?;
        f.write_str("/P")?;
      }
      ManaCostPip::Snow => f.write_char('S')?,
    };
    f.write_char('}')?;
    Ok(())
  }
}

fn byte_to_color(value: u8) -> Option<Color> {
  match value {
    b'W' => Some(Color::W),
    b'U' => Some(Color::U),
    b'B' => Some(Color::B),
    b'R' => Some(Color::R),
    b'G' => Some(Color::G),
    _ => None,
  }
}

impl TryFrom<&str> for ManaCostPip {
  type Error = String;

  fn try_from(str_value: &str) -> Result<Self, Self::Error> {
    let value = str_value.as_bytes();
    if !value.starts_with(b"{") || !value.ends_with(b"}") {
      Err(format!(
        "Invalid mana symbol {}, expected a mana cost symbol enclosed by curly brackets such as a \
         generic cost {{0}}, {{1}}, ..., or a symbol {{C}}, {{X}}, {{W}}, {{R/G}}, {{2/U}}, \
         {{B/P}}, or {{S}}",
        str_value
      ))?;
    }
    let value = &value[1..value.len() - 1];

    let r: Self = if value.iter().all(u8::is_ascii_digit) {
      if let Ok(Ok(val)) = std::str::from_utf8(value).map(|x| x.parse::<u8>()) {
        ManaCostPip::Generic(val)
      } else {
        Err(format!(
          "Invalid mana symbol {}, expected to parse a valid non-negative integer generic symbol \
           such as {{3}}",
          str_value
        ))?
      }
    } else if value.len() == 1 {
      if let Some(color) = byte_to_color(value[0]) {
        ManaCostPip::Color(color)
      } else {
        match value[0] {
          b'C' => ManaCostPip::Colorless,
          b'X' => ManaCostPip::X,
          b'S' => ManaCostPip::Snow,
          _ => Err(format!(
            "Invalid single character mana symbol {}, Expected one of {{W}}, {{U}}, {{B}}, {{R}}, \
             {{G}}, {{C}}, {{X}}, or {{S}} or a numeric generic mana symbol {{1}}, {{2}}, etc.",
            str_value
          ))?,
        }
      }
    } else if value.len() == 3 && value.starts_with(b"2/") {
      if let Some(color) = byte_to_color(value[2]) {
        ManaCostPip::MonoHybrid(color)
      } else {
        Err(format!(
          "Invalid monocolored hybrid mana symbol {}, expected a monocolored hybrid symbol of \
           {{2/W}}, {{2/U}}, {{2/B}}, {{2/R}}, or {{2/G}}",
          str_value
        ))?
      }
    } else if value.len() == 3 && value.ends_with(b"/P") {
      if let Some(color) = byte_to_color(value[0]) {
        ManaCostPip::Phyrexian(color)
      } else {
        Err(format!(
          "Invalid Phyrexian mana symbol {}, expected a Phyrexian symbol of {{W/P}}, {{U/P}}, \
           {{B/P}}, {{R/P}}, {{G/P}}",
          str_value
        ))?
      }
    } else if value.len() == 3 && value[1] == b'/' {
      if let (Some(fst), Some(snd)) = (byte_to_color(value[0]), byte_to_color(value[2])) {
        ManaCostPip::Hybrid((fst, snd).try_into()?)
      } else {
        Err(format!(
          "Invalid hybrid mana symbol {}, expected a hybrid symbol of {{W/U}}, {{W/B}}, {{U/B}}, \
           {{U/R}}, {{B/R}}, {{B/G}}, {{R/G}}, {{R/W}}, {{G/W}}, or {{G/U}}",
          str_value
        ))?
      }
    } else {
      Err(format!(
        "Invalid symbol {}, expected a value mana symbol",
        str_value
      ))?
    };

    Ok(r)
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ManaValue {
  Zero,
  Value(u8),
}

pub trait ManaValued {
  fn mana_value(self, x: Option<usize>) -> usize;
}

impl ManaValued for &ManaCostPip {
  fn mana_value(self, x: Option<usize>) -> usize {
    use ManaCostPip::*;

    match self {
      X => x.unwrap_or(0),
      MonoHybrid(_) => 2,
      Generic(n) => *n as usize,
      _ => 1,
    }
  }
}

impl<'a, T: IntoIterator<Item = &'a ManaCostPip>> ManaValued for T {
  fn mana_value(self, x: Option<usize>) -> usize {
    self.into_iter().map(|pip| pip.mana_value(x)).sum()
  }
}

impl ManaValued for &ManaCost {
  fn mana_value(self, x: Option<usize>) -> usize {
    self.pips().mana_value(x)
  }
}

impl Serialize for ManaCostPip {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

struct ManaCostPipVisitor;

impl<'de> Visitor<'de> for ManaCostPipVisitor {
  type Value = ManaCostPip;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter
      .write_str("a valid mana cost symbol, such as {C}, {3}, {W}, {2/U}, {P/R}, {S}, or {X}")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    ManaCostPip::try_from(v).map_err(de::Error::custom)
  }
}

impl<'de> Deserialize<'de> for ManaCostPip {
  fn deserialize<D>(deserializer: D) -> Result<ManaCostPip, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(ManaCostPipVisitor)
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(SerdeDiff)]
#[serde_diff(opaque)]
pub struct ManaCost(Cow<'static, [ManaCostPip]>);

impl ManaCost {
  pub const NONE: ManaCost = ManaCost(Cow::Borrowed(&[]));

  pub fn build() -> ManaCostBuilder {
    ManaCostBuilder::default()
  }

  pub fn from(value: Vec<ManaCostPip>) -> Self {
    ManaCost(Cow::Owned(value))
  }

  pub const fn from_static(value: &'static [ManaCostPip]) -> Self {
    ManaCost(Cow::Borrowed(value))
  }

  pub fn pips(&self) -> &[ManaCostPip] {
    self.0.borrow()
  }
}

impl From<Vec<ManaCostPip>> for ManaCost {
  fn from(value: Vec<ManaCostPip>) -> Self {
    ManaCost(value.into())
  }
}

impl From<&'static [ManaCostPip]> for ManaCost {
  fn from(value: &'static [ManaCostPip]) -> Self {
    ManaCost(value.into())
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct ManaCostBuilder {
  pips: Vec<ManaCostPip>,
  generic: u8,
}

impl ManaCostBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn color(mut self, color: Color) -> Self {
    self.pips.push(ManaCostPip::Color(color));
    self
  }

  pub fn colorless(mut self) -> Self {
    self.pips.push(ManaCostPip::Colorless);
    self
  }

  pub fn generic(mut self, value: u8) -> Self {
    self.generic += value;
    self
  }

  pub fn x(mut self) -> Self {
    self.pips.push(ManaCostPip::X);
    self
  }

  pub fn hybrid(mut self, first_color: Color, second_color: Color) -> Self {
    self.pips.push(ManaCostPip::Hybrid(
      (first_color, second_color).try_into().unwrap(),
    ));
    self
  }

  pub fn mono_hybrid(mut self, color: Color) -> Self {
    self.pips.push(ManaCostPip::MonoHybrid(color));
    self
  }

  pub fn phyrexian(mut self, color: Color) -> Self {
    self.pips.push(ManaCostPip::Phyrexian(color));
    self
  }

  pub fn snow(mut self) -> Self {
    self.pips.push(ManaCostPip::Snow);
    self
  }

  pub fn seal(self) -> ManaCost {
    let mut pips = self.pips;

    if self.generic > 0 {
      pips.push(ManaCostPip::Generic(self.generic));
    }
    pips.sort();

    ManaCost::from(pips)
  }
}

impl Display for ManaCost {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for pip in self.0.iter() {
      pip.fmt(f)?;
    }
    Ok(())
  }
}

impl TryFrom<&str> for ManaCost {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let pip_parts = value.split_inclusive('}');

    let (pips, errors): (Vec<Result<ManaCostPip, _>>, _) =
      pip_parts.map(|x| x.try_into()).partition(Result::is_ok);

    if errors.len() > 0 {
      Err(errors[0].clone().unwrap_err())?;
    }

    let pips: Vec<_> = pips.into_iter().map(Result::unwrap).collect();

    Ok(ManaCost(Cow::Owned(pips)))
  }
}

impl Serialize for ManaCost {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

struct ManaCostVisitor;

impl<'de> Visitor<'de> for ManaCostVisitor {
  type Value = ManaCost;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter
      .write_str("a valid mana cost symbol, such as {C}, {3}, {W}, {2/U}, {P/R}, {S}, or {X}")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    ManaCost::try_from(v).map_err(de::Error::custom)
  }
}

impl<'de> Deserialize<'de> for ManaCost {
  fn deserialize<D>(deserializer: D) -> Result<ManaCost, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(ManaCostVisitor)
  }
}

impl ObjectColored for &ManaCostPip {
  fn get_object_color(self) -> ObjectColor {
    match self {
      ManaCostPip::X => ObjectColor::NONE,
      ManaCostPip::Generic(_) => ObjectColor::NONE,
      ManaCostPip::Colorless => ObjectColor::NONE,
      ManaCostPip::Hybrid(mn) => mn.get_object_color(),
      ManaCostPip::Color(c) => c.get_object_color(),
      ManaCostPip::MonoHybrid(c) => c.get_object_color(),
      ManaCostPip::Phyrexian(c) => c.get_object_color(),
      ManaCostPip::Snow => ObjectColor::NONE,
    }
  }
}

impl ObjectColored for &ManaCost {
  fn get_object_color(self) -> ObjectColor {
    ObjectColor::from_iter(self.pips().iter().map(|x| x.get_object_color()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn assert_sizeof() {
    assert_eq!(
      std::mem::size_of::<ManaCost>(),
      std::mem::size_of::<usize>() * 2
    );
    assert_eq!(std::mem::size_of::<ManaCostPip>(), 2);
    assert_eq!(std::mem::size_of::<[ManaCostPip; 10]>(), 20);
  }
}
