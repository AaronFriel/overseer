use std::fmt::{self, Display};

use bitflags::bitflags;
use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};
use serde_diff::SerdeDiff;

pub trait ObjectColored {
  fn get_object_color(self) -> ObjectColor;
}

bitflags! {
  #[derive(SerdeDiff)]
  pub struct ObjectColor: u8 {
      const W = 1 << 0;
      const U = 1 << 1;
      const B = 1 << 2;
      const R = 1 << 3;
      const G = 1 << 4;

      const WU = Self::W.bits | Self::U.bits;
      const UB = Self::U.bits | Self::B.bits;
      const BR = Self::B.bits | Self::R.bits;
      const RG = Self::R.bits | Self::G.bits;
      const GW = Self::G.bits | Self::W.bits;

      const WB = Self::W.bits | Self::B.bits;
      const UR = Self::U.bits | Self::R.bits;
      const BG = Self::B.bits | Self::G.bits;
      const RW = Self::R.bits | Self::W.bits;
      const GU = Self::G.bits | Self::U.bits;

      const WUB = Self::W.bits | Self::U.bits | Self::B.bits;
      const UBR = Self::U.bits | Self::B.bits | Self::R.bits;
      const BRG = Self::B.bits | Self::R.bits | Self::G.bits;
      const RGW = Self::R.bits | Self::G.bits | Self::W.bits;
      const GWU = Self::G.bits | Self::W.bits | Self::U.bits;

      const WBG = Self::W.bits | Self::B.bits | Self::G.bits;
      const URW = Self::U.bits | Self::R.bits | Self::W.bits;
      const BGU = Self::B.bits | Self::G.bits | Self::U.bits;
      const RWB = Self::R.bits | Self::W.bits | Self::B.bits;
      const GUR = Self::G.bits | Self::U.bits | Self::R.bits;

      const WUBR = Self::W.bits | Self::U.bits | Self::B.bits | Self::R.bits;
      const UBRG = Self::U.bits | Self::B.bits | Self::R.bits | Self::G.bits;
      const BRGW = Self::B.bits | Self::R.bits | Self::G.bits | Self::W.bits;
      const RGWU = Self::R.bits | Self::G.bits | Self::W.bits | Self::U.bits;
      const GWUB = Self::G.bits | Self::W.bits | Self::U.bits | Self::B.bits;

      const WUBRG = Self::W.bits |Self::U.bits | Self::B.bits | Self::R.bits | Self::G.bits;

      const C = 0;
      const NONE = 0;
  }
}

impl ObjectColor {
  pub const fn as_str(&self) -> &'static str {
    match *self {
      // Colorless
      Self::C => "C",

      Self::W => "W",
      Self::U => "U",
      Self::B => "B",
      Self::R => "R",
      Self::G => "G",

      Self::WU => "WU",
      Self::UB => "UB",
      Self::BR => "BR",
      Self::RG => "RG",
      Self::GW => "GW",

      Self::WB => "WB",
      Self::UR => "UR",
      Self::BG => "BG",
      Self::RW => "RW",
      Self::GU => "GU",

      Self::WUB => "WUB",
      Self::UBR => "UBR",
      Self::BRG => "BRG",
      Self::RGW => "RGW",
      Self::GWU => "GWU",

      Self::WBG => "WBG",
      Self::URW => "URW",
      Self::BGU => "BGU",
      Self::RWB => "RWB",
      Self::GUR => "GUR",

      Self::WUBR => "WUBR",
      Self::UBRG => "UBRG",
      Self::BRGW => "BRGW",
      Self::RGWU => "RGWU",
      Self::GWUB => "GWUB",

      Self::WUBRG => "WUBRG",

      _ => "C",
    }
  }

  pub const fn is_colorless(&self) -> bool {
    self.bits.count_ones() == 0
  }

  pub const fn is_monocolored(&self) -> bool {
    self.bits.count_ones() == 1
  }

  pub const fn is_multicolored(&self) -> bool {
    self.bits.count_ones() >= 2
  }
}

impl Display for ObjectColor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

impl Default for ObjectColor {
  fn default() -> Self {
    ObjectColor::NONE
  }
}

impl From<&str> for ObjectColor {
  fn from(value: &str) -> Self {
    let mut color = ObjectColor::empty();

    if value.contains('W') {
      color |= ObjectColor::W;
    }

    if value.contains('U') {
      color |= ObjectColor::U;
    }

    if value.contains('B') {
      color |= ObjectColor::B;
    }

    if value.contains('R') {
      color |= ObjectColor::R;
    }

    if value.contains('G') {
      color |= ObjectColor::G;
    }

    color
  }
}

impl Serialize for ObjectColor {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
  type Value = ObjectColor;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str(
      "a string containing any combination of the characters W, U, B, R, G or the character C",
    )
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(ObjectColor::from(v))
  }
}

impl<'de> Deserialize<'de> for ObjectColor {
  fn deserialize<D>(deserializer: D) -> Result<ObjectColor, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(ColorVisitor)
  }
}

impl ObjectColored for &ObjectColor {
  fn get_object_color(self) -> ObjectColor {
    *self
  }
}
