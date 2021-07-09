use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::game::ObjectHandle;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum Zones {
  Hand,
  Library,
  Battlefield,
  Graveyard,
  Stack,
  Exile,
  Command,
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
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Command;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Zone<K> {
  pub cards: Vec<ObjectHandle>,
  #[serde(skip)]
  #[serde_diff(skip)]
  kind: PhantomData<K>,
}

pub trait ZoneKinded:
  Clone + Eq + PartialEq + PartialOrd + std::hash::Hash + std::fmt::Debug + Default
{
  const KIND: Zones;
}

impl ZoneKinded for Hand {
  const KIND: Zones = Zones::Hand;
}
impl ZoneKinded for Library {
  const KIND: Zones = Zones::Library;
}
impl ZoneKinded for Battlefield {
  const KIND: Zones = Zones::Battlefield;
}
impl ZoneKinded for Graveyard {
  const KIND: Zones = Zones::Graveyard;
}
impl ZoneKinded for Stack {
  const KIND: Zones = Zones::Stack;
}
impl ZoneKinded for Exile {
  const KIND: Zones = Zones::Exile;
}
impl ZoneKinded for Command {
  const KIND: Zones = Zones::Command;
}

impl<K> Zone<K> {
  pub fn new() -> Self {
    Self {
      cards: vec![],
      kind: PhantomData,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &ObjectHandle> {
    self.cards.iter()
  }
}

pub enum ViewedAs {
  Player,
  ControllerOfPlayer,
  Other,
}
