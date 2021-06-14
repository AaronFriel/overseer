use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::game::CardList;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum ZoneKind {
  Hand,
  Library,
  Battlefield,
  Graveyard,
  Stack,
  Exile,
}

impl ZoneKind {
  pub fn new_zone(self) -> Zone {
    Zone::new(self)
  }

  pub fn is_public(&self) -> bool {
    match self {
      ZoneKind::Library => false,
      ZoneKind::Hand => false,
      _ => true,
    }
  }

  pub fn is_shared(&self) -> bool {
    match self {
      ZoneKind::Library => false,
      ZoneKind::Hand => false,
      ZoneKind::Graveyard => false,
      _ => true,
    }
  }

  pub fn is_ordered(&self) -> bool {
    match self {
      ZoneKind::Library => true,
      ZoneKind::Graveyard => true,
      ZoneKind::Stack => true,
      _ => false,
    }
  }
}

impl Default for ZoneKind {
  fn default() -> Self {
    ZoneKind::Battlefield
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Zone {
  pub cards: CardList,
  pub kind: ZoneKind,
}

impl Zone {
  pub fn new(kind: ZoneKind) -> Self {
    Self {
      cards: Vec::new(),
      kind,
    }
  }

  pub fn set_cards(&mut self, cards: CardList) {
    self.cards = cards;
  }
}
