use std::borrow::Cow;

use overseer_util::make_refcounted_pool;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_diff::SerdeDiff;
use bitflags::bitflags;


use crate::{
  game::{ManaCost, ObjectColor, PlayerHandle, TypeLine, RegisteredCard},
};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum ObjectKind {
  Ability,
  Card,
  CopyOfCard,
  Token,
  Spell,
  Permanent,
  Emblem,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Object {
  pub kind: ObjectKind,
  pub characteristics: Option<Characteristics>,
  pub card: Option<RegisteredCard>,

  #[serde_diff(opaque)]
  pub status: Status,
  pub owner: Option<PlayerHandle>,
  pub controller: Option<PlayerHandle>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
/// 109.3
pub struct Characteristics {
  #[serde_diff(opaque)]
  pub name: Cow<'static, str>,
  pub mana_cost: ManaCost,

  pub color: ObjectColor,
  pub color_indicator: Option<usize>,

  pub type_line: TypeLine,

  #[serde_diff(opaque)]
  pub rules_text: Cow<'static, str>,
  // pub abilities: Vec<String>,

  // 208.
  pub power: usize,
  pub toughness: usize,

  /// 209.
  pub loyalty: usize,

  #[cfg(feature = "vanguard")]
  /// 210.
  pub hand_modifier: i8,

  #[cfg(feature = "vanguard")]
  /// 211.
  pub life_modifier: i8,
}

bitflags! {
  pub struct Status: u8 {
    const TAPPED = 1<<0;
    const FLIPPED = 1<<1;
    const FACED_DOWN = 1<<2;
    const PHASED_OUT = 1<<3;
  }
}

impl Default for Status {
    fn default() -> Self {
        Status::empty()
    }
}

impl Serialize for Status {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut status_vec = Vec::new();

    if self.intersects(Status::TAPPED) {
      status_vec.push("tapped")
    }
    if self.intersects(Status::FLIPPED) {
      status_vec.push("flipped")
    }
    if self.intersects(Status::FACED_DOWN) {
      status_vec.push("faced_down")
    }
    if self.intersects(Status::PHASED_OUT) {
      status_vec.push("phased_out")
    }

    status_vec.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Status {
  fn deserialize<D>(deserializer: D) -> Result<Status, D::Error>
  where
    D: Deserializer<'de>,
  {
    let status_vec = Vec::<String>::deserialize(deserializer)?;

    let mut status = Status::empty();
    status_vec.iter().for_each(|value| match value.as_ref() {
      "tapped" => status |= Status::TAPPED,
      "flipped" => status |= Status::FLIPPED,
      "faced_down" => status |= Status::FACED_DOWN,
      "phased_out" => status |= Status::PHASED_OUT,
      _ => {}
    });

    Ok(status)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn size_of_characteristics() {
    let expected_size = 176;
    #[cfg(feature = "vanguard")]
    let expected_size = expected_size + 2;

    assert_eq!(std::mem::size_of::<Characteristics>(), expected_size);
  }
}

make_refcounted_pool!(Object, ObjectPool, ObjectHandle, usize);
