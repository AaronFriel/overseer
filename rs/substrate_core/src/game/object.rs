use std::borrow::Cow;

use bitflags::bitflags;
use overseer_util::make_refcounted_pool;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_diff::SerdeDiff;

use super::{
  Characteristics, Characterized, ManaCost, ObjectColor, PlayerHandle, RegisteredCard, TypeLine,
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
  // TODO: make private
  pub characteristics: Option<Characteristics>,
  pub card: Option<RegisteredCard>,

  #[serde_diff(opaque)]
  pub status: Status,
  pub owner: Option<PlayerHandle>,
  pub controller: Option<PlayerHandle>,
}

impl From<RegisteredCard> for Object {
  #[inline]
  fn from(card: RegisteredCard) -> Self {
    Self {
      kind: ObjectKind::Card,
      characteristics: Default::default(),
      card: Some(card),
      status: Default::default(),
      owner: Default::default(),
      controller: Default::default(),
    }
  }
}

const EMPTY_STR: Cow<'static, str> = Cow::Borrowed("");

impl Characterized for Object {
  fn get_name(&self) -> &Cow<'static, str> {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_name()
    } else if let Some(card) = &self.card {
      card.get_name()
    } else {
      &EMPTY_STR
    }
  }

  fn get_mana_cost(&self) -> &ManaCost {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_mana_cost()
    } else if let Some(card) = &self.card {
      card.get_mana_cost()
    } else {
      &ManaCost::NONE
    }
  }

  fn get_color(&self) -> ObjectColor {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_color()
    } else if let Some(card) = &self.card {
      card.get_color()
    } else {
      Default::default()
    }
  }

  fn get_color_indicator(&self) -> Option<ObjectColor> {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_color_indicator()
    } else if let Some(card) = &self.card {
      card.get_color_indicator()
    } else {
      Default::default()
    }
  }

  fn get_type_line(&self) -> &TypeLine {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_type_line()
    } else if let Some(card) = &self.card {
      card.get_type_line()
    } else {
      &TypeLine::NONE
    }
  }

  fn get_rules_text(&self) -> &Cow<'static, str> {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_rules_text()
    } else if let Some(card) = &self.card {
      card.get_rules_text()
    } else {
      &EMPTY_STR
    }
  }

  fn get_power(&self) -> u32 {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_power()
    } else if let Some(card) = &self.card {
      card.get_power()
    } else {
      Default::default()
    }
  }

  fn get_toughness(&self) -> u32 {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_toughness()
    } else if let Some(card) = &self.card {
      card.get_toughness()
    } else {
      Default::default()
    }
  }

  fn get_loyalty(&self) -> u32 {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_loyalty()
    } else if let Some(card) = &self.card {
      card.get_loyalty()
    } else {
      Default::default()
    }
  }

  fn get_hand_modifier(&self) -> i8 {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_hand_modifier()
    } else if let Some(card) = &self.card {
      card.get_hand_modifier()
    } else {
      Default::default()
    }
  }

  fn get_life_modifier(&self) -> i8 {
    if let Some(characteristics) = &self.characteristics {
      characteristics.get_life_modifier()
    } else if let Some(card) = &self.card {
      card.get_life_modifier()
    } else {
      Default::default()
    }
  }
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

make_refcounted_pool!(Object, ObjectPool, ObjectHandle, u32);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn size_of_characteristics() {
    let expected_size = 144;
    #[cfg(feature = "vanguard")]
    let expected_size = expected_size + 2;

    assert_eq!(std::mem::size_of::<Characteristics>(), expected_size);
  }

  #[test]
  fn size_of() {
    make_refcounted_pool!(Object, FooPool, FooHandle, u32);
    assert_eq!(std::mem::size_of::<FooHandle>(), 16);
    assert_eq!(
      std::mem::size_of::<FooHandle>(),
      std::mem::size_of::<Option<FooHandle>>()
    );
  }
}
