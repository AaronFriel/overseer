use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use super::{Card, ManaCost, ObjectColor, TypeLine};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
/// 109.3
pub struct Characteristics {
  #[serde_diff(opaque)]
  pub name: Cow<'static, str>,
  pub mana_cost: ManaCost,

  pub color: ObjectColor,
  pub color_indicator: Option<ObjectColor>,

  pub type_line: TypeLine,

  #[serde_diff(opaque)]
  pub rules_text: Cow<'static, str>,
  // pub abilities: Vec<String>,

  // 208.
  pub power: u32,
  pub toughness: u32,

  /// 209.
  pub loyalty: u32,

  #[cfg(feature = "vanguard")]
  /// 210.
  pub hand_modifier: i8,

  #[cfg(feature = "vanguard")]
  /// 211.
  pub life_modifier: i8,
}

impl From<Card> for Characteristics {
  fn from(card: Card) -> Self {
    Self {
      name: card.name,
      mana_cost: card.mana_cost,
      color: ObjectColor::C,
      color_indicator: card.color_indicator,
      type_line: card.type_line,
      rules_text: card.rules_text,
      power: card.power,
      toughness: card.toughness,
      loyalty: card.loyalty,
      #[cfg(feature = "vanguard")]
      hand_modifier: card.hand_modifier,
      #[cfg(feature = "vanguard")]
      life_modifier: card.life_modifier,
    }
  }
}

pub trait Characterized {
  fn get_name(&self) -> &Cow<'static, str>;

  fn get_mana_cost(&self) -> &ManaCost;

  fn get_color(&self) -> ObjectColor;

  fn get_color_indicator(&self) -> Option<ObjectColor>;

  fn get_type_line(&self) -> &TypeLine;

  fn get_rules_text(&self) -> &Cow<'static, str>;

  fn get_power(&self) -> u32;

  fn get_toughness(&self) -> u32;

  fn get_loyalty(&self) -> u32;

  fn get_hand_modifier(&self) -> i8 {
    0
  }

  fn get_life_modifier(&self) -> i8 {
    0
  }
}

impl Characterized for Characteristics {
  fn get_name(&self) -> &Cow<'static, str> {
    &self.name
  }

  fn get_mana_cost(&self) -> &ManaCost {
    &self.mana_cost
  }

  fn get_color(&self) -> ObjectColor {
    self.color
  }

  fn get_color_indicator(&self) -> Option<ObjectColor> {
    self.color_indicator
  }

  fn get_type_line(&self) -> &TypeLine {
    &self.type_line
  }

  fn get_rules_text(&self) -> &Cow<'static, str> {
    &self.rules_text
  }

  fn get_power(&self) -> u32 {
    self.power
  }

  fn get_toughness(&self) -> u32 {
    self.toughness
  }

  fn get_loyalty(&self) -> u32 {
    self.loyalty
  }

  #[cfg(feature = "vanguard")]
  fn get_hand_modifier(&self) -> i8 {
    self.hand_modifier
  }

  #[cfg(feature = "vanguard")]
  fn get_life_modifier(&self) -> i8 {
    self.life_modifier
  }
}
