use serde::{Deserialize, Serialize};

use crate::game::{Graveyard, Hand, Library, ObjectHandle, PlayerHandle, RegisteredCard, Zone};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Player {
  pub name: String,
  pub handle: Option<PlayerHandle>,
  pub controller: Option<PlayerHandle>,

  pub deck: Vec<RegisteredCard>,
  pub sideboard: Vec<RegisteredCard>,

  pub library: Zone<Library>,
  pub hand: Zone<Hand>,
  pub graveyard: Zone<Graveyard>,
  pub revealed: Vec<ObjectHandle>,

  pub life: u32, // 20

  pub has_left_game: bool,
  pub has_lost_game: bool,
}

impl Player {
  pub fn new(
    name: impl ToString,
    deck: Vec<RegisteredCard>,
    sideboard: Vec<RegisteredCard>,
  ) -> Self {
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

  pub fn clone_visible(&self, mut visible_set_filter: impl FnMut(&ObjectHandle) -> bool) -> Player {
    let library = self.library.into_filtered_view(&mut visible_set_filter);
    let hand = self.hand.into_filtered_view(&mut visible_set_filter);
    let graveyard = self.graveyard.into_filtered_view(&mut visible_set_filter);

    Player {
      name: self.name.clone(),
      deck: Default::default(),
      sideboard: Default::default(),
      revealed: Default::default(),
      library,
      hand,
      graveyard,
      ..*self
    }
  }
}
