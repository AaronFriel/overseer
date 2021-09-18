use std::borrow::Cow;

use overseer_substrate::{
  game::{Card, ManaCost, TypeLine},
  type_line,
};

pub const fn basic_land(name: &'static str, type_line: TypeLine) -> Card {
  Card {
    name: Cow::Borrowed(name),
    type_line,
    color_indicator: None,
    power: 0,
    toughness: 0,
    loyalty: 0,
    mana_cost: ManaCost::NONE,
    rules_text: Cow::Borrowed(""),
    #[cfg(feature = "vanguard")]
    hand_modifier: 0,
    #[cfg(feature = "vanguard")]
    life_modifier: 0,
  }
}

pub const PLAINS: Card = basic_land("Plains", type_line!("Basic Land - Plains"));

pub const ISLAND: Card = basic_land("Island", type_line!("Basic Land - Island"));

pub const SWAMP: Card = basic_land("Swamp", type_line!("Basic Land - Swamp"));

pub const MOUNTAIN: Card = basic_land("Mountain", type_line!("Basic Land - Mountain"));

pub const FOREST: Card = basic_land("Forest", type_line!("Basic Land - Forest"));

pub const WASTES: &'static Card = &basic_land("Wastes", type_line!("Basic Land"));

pub const SNOW_COVERED_PLAINS: Card = basic_land(
  "Snow-Covered Plains",
  type_line!("Basic Snow Land - Plains"),
);

pub const SNOW_COVERED_ISLAND: Card = basic_land(
  "Snow-Covered Island",
  type_line!("Basic Snow Land - Island"),
);

pub const SNOW_COVERED_SWAMP: Card =
  basic_land("Snow-Covered Swamp", type_line!("Basic Snow Land - Swamp"));

pub const SNOW_COVERED_MOUNTAIN: Card = basic_land(
  "Snow-Covered Mountain",
  type_line!("Basic Snow Land - Mountain"),
);

pub const SNOW_COVERED_FOREST: Card = basic_land(
  "Snow-Covered Forest",
  type_line!("Basic Snow Land - Forest"),
);
