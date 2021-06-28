use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::game::{Card, Zone, ZoneKind};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Player {
  pub name: String,

  pub deck: Vec<Card>,
  pub sideboard: Vec<Card>,

  pub library: Zone,
  pub hand: Zone,
  pub graveyard: Zone,

  pub life: i32, // 20

  pub has_left_game: bool,
  pub has_lost_game: bool,
}

impl Player {
  pub fn new<T>(name: T, deck: Vec<Card>, sideboard: Vec<Card>) -> Self
  where
    T: Into<String>,
  {
    Self {
      name: name.into(),
      library: ZoneKind::Library.new_zone(),
      graveyard: ZoneKind::Graveyard.new_zone(),
      hand: ZoneKind::Graveyard.new_zone(),

      life: 20,
      deck,
      sideboard,

      has_left_game: false,
      has_lost_game: false,
    }
  }
}

/*

// Extra fields
// pub final Map<Card, Integer> assignedDamage, // Maps.newHashMap();
// pub final Map<Card, Integer> assignedCombatDamage, // Maps.newHashMap();
// pub lastDrawnCard, // null;
// pub String namedCard, // "";
// pub String namedCard2, // "";
*/
