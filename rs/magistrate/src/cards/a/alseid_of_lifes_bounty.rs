use std::borrow::Cow;

use overseer_substrate::{game::Card, mana_cost, type_line};

pub const ALSEID_OF_LIFES_BOUNTY: Card = Card {
  name: Cow::Borrowed("Alseid of Life's Bounty"),
  mana_cost: mana_cost!("{W}"),
  type_line: type_line!("Enchantment Creature Nymph"),
  rules_text: Cow::Borrowed(concat!(
    "Lifelink",
    "\n\n",
    "{1}, Sacrifice Alseid of Life’s Bounty: Target creature or enchantment you control gains \
     protection from the color of your choice until end of turn."
  )),
  power: 1,
  toughness: 1,
  loyalty: 0,
  color_indicator: None,
  #[cfg(feature = "vanguard")]
  hand_modifier: 0,
  #[cfg(feature = "vanguard")]
  life_modifier: 0,
};
