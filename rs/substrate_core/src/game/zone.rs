use std::{borrow::Cow, marker::PhantomData};

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

pub type ZoneHand<'a> = Zone<'a, Hand>;
pub type ZoneLibrary<'a> = Zone<'a, Library>;
pub type ZoneBattlefield<'a> = Zone<'a, Battlefield>;
pub type ZoneGraveyard<'a> = Zone<'a, Graveyard>;
pub type ZoneStack<'a> = Zone<'a, Stack>;
pub type ZoneExile<'a> = Zone<'a, Exile>;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Zone<'x, K> {
  cards: Cow<'x, CardList>,
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

impl Zone<'static, Hand> {
  pub fn new_hand() -> Self {
    Self::new()
  }
}
impl Zone<'static, Library> {
  pub fn new_library() -> Self {
    Self::new()
  }
}
impl Zone<'static, Battlefield> {
  pub fn new_battlefield() -> Self {
    Self::new()
  }
}
impl Zone<'static, Graveyard> {
  pub fn new_graveyard() -> Self {
    Self::new()
  }
}
impl Zone<'static, Stack> {
  pub fn new_stack() -> Self {
    Self::new()
  }
}
impl Zone<'static, Exile> {
  pub fn new_exile() -> Self {
    Self::new()
  }
}

impl<K> Zone<'static, K> {
  pub fn new() -> Self {
    Self {
      cards: Cow::Owned(vec![]),
      count: 0,
      kind: PhantomData,
    }
  }
}

impl<'a, K> Zone<'a, K> {
  pub fn cow_copy(&'a self) -> Zone<'a, K> {
    Self {
      cards: Cow::Borrowed(&self.cards),
      ..*self
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
