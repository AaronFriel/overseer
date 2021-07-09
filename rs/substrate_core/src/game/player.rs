use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{
  game::{ObjectHandle, PlayerHandle, CardHandle, Zone, Library, Hand, Graveyard},
};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Player {
  pub name: String,
  pub handle: Option<PlayerHandle>,
  pub controller: Option<PlayerHandle>,

  pub deck: Vec<CardHandle>,
  pub sideboard: Vec<CardHandle>,

  pub library: Zone<Library>,
  pub hand: Zone<Hand>,
  pub graveyard: Zone<Graveyard>,
  pub revealed: Vec<ObjectHandle>,

  pub life: u32, // 20

  pub has_left_game: bool,
  pub has_lost_game: bool,
}

impl Player {
  pub fn new(name: impl ToString, deck: Vec<CardHandle>, sideboard: Vec<CardHandle>) -> Self {
    Self {
      name: name.to_string().into(),

      handle: None,
      controller: None,

      library: Zone::new(),
      graveyard: Zone::new(),
      hand: Zone::new(),
      revealed: Vec::new(),

      life: 20,
      deck,
      sideboard,

      has_left_game: false,
      has_lost_game: false,
    }
  }
}
