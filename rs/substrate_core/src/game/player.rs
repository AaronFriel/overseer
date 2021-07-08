use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use super::CardHandle;
use crate::game::{Graveyard, Hand, Library, PlayerHandle, Zone};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Player<'x> {
  #[serde_diff(opaque)]
  pub name: Cow<'x, str>,
  pub handle: Option<PlayerHandle>,
  pub controller: Option<PlayerHandle>,

  pub deck: Vec<CardHandle>,
  pub sideboard: Vec<CardHandle>,

  pub library: Zone<'x, Library>,
  pub hand: Zone<'x, Hand>,
  pub graveyard: Zone<'x, Graveyard>,

  pub life: i32, // 20

  pub has_left_game: bool,
  pub has_lost_game: bool,
}

impl Player<'static> {
  pub fn new(name: impl ToString, deck: Vec<CardHandle>, sideboard: Vec<CardHandle>) -> Self {
    Self {
      name: name.to_string().into(),

      handle: None,
      controller: None,

      library: Zone::new(),
      graveyard: Zone::new(),
      hand: Zone::new(),

      life: 20,
      deck,
      sideboard,

      has_left_game: false,
      has_lost_game: false,
    }
  }
}

// impl<'a> Visible for Player<'a> {
//   type Context = PlayerHandle;

//   fn is_visible(&self, context: &Self::Context) -> bool {
//     if self.handle.eq(other) == context {
//       return true;
//     }

//     if self.controller == context {
//       return true;
//     };

//     false
//   }

//   fn to_visible(&self, context: &Self::Context) -> Self {
//     Player {
//       name: (&*self.name).into(),
//       handle: self.handle,
//       controller: self.handle,

//       deck: vec![],
//       sideboard: vec![],
//       library: self.library.as_hidden(),
//       hand: self.hand.as_hidden(),
//       graveyard: self.graveyard.clone(),
//       ..*self
//     }
//   }

//   fn to_hidden(&self, context: &Self::Context) -> Self {
//     Player {
//       name: (&*self.name).into(),
//       handle: self.handle,
//       controller: self.handle,

//       deck: vec![],
//       sideboard: vec![],
//       library: self.library.view_as(),
//       hand: self.hand.cow_copy(),
//       graveyard: self.graveyard.cow_copy(),
//       ..*self
//     }
//   }
// }
