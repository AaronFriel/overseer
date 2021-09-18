use std::borrow::Cow;

use overseer_substrate::{game::Card, mana_cost, type_line};

pub const METALLIC_SLIVER: &'static Card = &Card {
  name: Cow::Borrowed("Metallic Sliver"),
  mana_cost: mana_cost!("{1}"),
  type_line: type_line!("Artifact Creature"),
  power: 1,
  toughness: 1,
  loyalty: 0,
  color_indicator: None,
  rules_text: Cow::Borrowed(""),
  #[cfg(feature = "vanguard")]
  hand_modifier: 0,
  #[cfg(feature = "vanguard")]
  life_modifier: 0,
};
