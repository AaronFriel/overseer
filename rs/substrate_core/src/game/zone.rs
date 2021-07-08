use std::{marker::PhantomData};

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{game::ObjectHandle};

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

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Hand;
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Library;
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Battlefield;
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Graveyard;
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Stack;
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Exile;

pub type ZoneHand = Zone<Hand>;
pub type ZoneLibrary = Zone<Library>;
pub type ZoneBattlefield = Zone<Battlefield>;
pub type ZoneGraveyard = Zone<Graveyard>;
pub type ZoneStack = Zone<Stack>;
pub type ZoneExile = Zone<Exile>;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Zone<K> {
  pub cards: Vec<ObjectHandle>,
  pub count: usize,
  #[serde(skip)]
  #[serde_diff(skip)]
  kind: PhantomData<K>,
}

pub trait ZoneKinded {
  const KIND: ZoneKind;
}

impl ZoneKinded for Hand {
  const KIND: ZoneKind = ZoneKind::Hand;
}
impl ZoneKinded for Library {
  const KIND: ZoneKind = ZoneKind::Library;
}
impl ZoneKinded for Battlefield {
  const KIND: ZoneKind = ZoneKind::Battlefield;
}
impl ZoneKinded for Graveyard {
  const KIND: ZoneKind = ZoneKind::Graveyard;
}
impl ZoneKinded for Stack {
  const KIND: ZoneKind = ZoneKind::Stack;
}
impl ZoneKinded for Exile {
  const KIND: ZoneKind = ZoneKind::Exile;
}

impl Zone<Hand> {
  pub fn new_hand() -> Self {
    Self::new()
  }
}
impl Zone<Library> {
  pub fn new_library() -> Self {
    Self::new()
  }
}
impl Zone<Battlefield> {
  pub fn new_battlefield() -> Self {
    Self::new()
  }
}
impl Zone<Graveyard> {
  pub fn new_graveyard() -> Self {
    Self::new()
  }
}
impl Zone<Stack> {
  pub fn new_stack() -> Self {
    Self::new()
  }
}
impl Zone<Exile> {
  pub fn new_exile() -> Self {
    Self::new()
  }
}

impl<K> Zone<K> {
  pub fn new() -> Self {
    Self {
      cards: vec![],
      count: 0,
      kind: PhantomData,
    }
  }
}

// enum ZoneViewAs {
//   Owner,
//   Other,
// }

// impl<'a, K> Viewable for Zone<'a, K> {
//   type Context = (ZoneViewAs, PlayerHandle);

//   fn view_as(&self, context: &Self::Context) -> Self {
//     Zone {
//         cards: self.cards,
//         count: (),
//         kind: (),
//     }
//   }
// }
