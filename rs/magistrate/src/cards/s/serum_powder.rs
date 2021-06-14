use std::borrow::Cow;

use overseer_substrate::{
  game::{Card, ObjectColor},
  mana_cost, type_line,
};

pub const SERUM_POWDER: Card = Card {
  name: Cow::Borrowed("Serum Powder"),
  mana_cost: mana_cost!("{3}"),
  type_line: type_line!("Artifact"),
  rules_text: Cow::Borrowed(concat!(
    "{T}: Add {C}.",
    "\n\n",
    "Any time you could mulligan and Serum Powder is in your hand, you may exile all the cards \
     from your hand, then draw that many cards."
  )),
  loyalty: 0,
  power: 0,
  toughness: 0,
  color_indicator: ObjectColor::NONE,
  #[cfg(feature = "vanguard")]
  hand_modifier: 0,
  #[cfg(feature = "vanguard")]
  life_modifier: 0,
};
