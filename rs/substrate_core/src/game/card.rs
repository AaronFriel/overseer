use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::game::{ManaCost, ObjectColor, TypeLine};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct Card {
  pub name: Cow<'static, str>,
  pub mana_cost: ManaCost,

  pub color_indicator: ObjectColor,

  pub type_line: TypeLine,

  pub rules_text: Cow<'static, str>,

  pub power: usize,
  pub toughness: usize,

  pub loyalty: usize,

  #[cfg(feature = "vanguard")]
  /// 210.
  pub hand_modifier: i8,

  #[cfg(feature = "vanguard")]
  /// 211.
  pub life_modifier: i8,
  /* expansion symbol
   * illustration
   * illustration credit
   * legal text
   * collector number */
}

impl Card {
  pub const DEFAULT: Card = Card::const_default();

  pub const fn const_default() -> Self {
    Self {
      name: Cow::Borrowed(""),
      mana_cost: ManaCost::NONE,
      color_indicator: ObjectColor::NONE,
      type_line: TypeLine::const_default(),
      rules_text: Cow::Borrowed(""),
      power: 0,
      toughness: 0,
      loyalty: 0,
      #[cfg(feature = "vanguard")]
      hand_modifier: 0,
      #[cfg(feature = "vanguard")]
      life_modifier: 0,
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(transparent)]
pub struct CardHandle(usize);

pub type CardList = Vec<CardHandle>;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn assert_sizeof() {
    let expected_size = 158;
    #[cfg(feature = "vanguard")]
    let expected_size = expected_size + 2;

    assert_eq!(std::mem::size_of::<Card>(), expected_size);
  }
}
