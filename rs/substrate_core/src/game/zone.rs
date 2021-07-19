use std::{
  collections::{HashSet, VecDeque},
  iter::FromIterator,
  marker::PhantomData,
};

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

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Zone<K>
where
  K: ZoneKinded,
{
  cards: K::Collection,
  count: u32,
  #[serde(skip)]
  #[serde_diff(skip)]
  kind: PhantomData<K>,
}

impl<K> std::hash::Hash for Zone<K>
where
  K: ZoneKinded,
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.cards.iter().for_each(|x| x.hash(state));
    self.count.hash(state);
  }
}

pub trait ZoneKinded:
  Clone + Eq + PartialEq + PartialOrd + std::hash::Hash + std::fmt::Debug + Default
{
  const KIND: Zones;
  type Collection: CardCollection;
}

impl ZoneKinded for Hand {
  type Collection = HashSet<ObjectHandle>;

  const KIND: Zones = Zones::Hand;
}
impl ZoneKinded for Library {
  type Collection = VecDeque<ObjectHandle>;

  const KIND: Zones = Zones::Library;
}
impl ZoneKinded for Battlefield {
  type Collection = HashSet<ObjectHandle>;

  const KIND: Zones = Zones::Battlefield;
}
impl ZoneKinded for Graveyard {
  type Collection = VecDeque<ObjectHandle>;

  const KIND: Zones = Zones::Graveyard;
}
impl ZoneKinded for Stack {
  type Collection = VecDeque<ObjectHandle>;

  const KIND: Zones = Zones::Stack;
}
impl ZoneKinded for Exile {
  type Collection = HashSet<ObjectHandle>;

  const KIND: Zones = Zones::Exile;
}
impl ZoneKinded for Command {
  type Collection = HashSet<ObjectHandle>;

  const KIND: Zones = Zones::Command;
}

impl<K> Zone<K>
where
  K: ZoneKinded,
{
  pub fn new() -> Self {
    Self {
      cards: Default::default(),
      count: 0,
      kind: PhantomData,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &ObjectHandle> {
    self.cards.iter()
  }

  pub fn insert(&mut self, object_handle: ObjectHandle) -> bool {
    let inserted = self.cards.insert(object_handle);
    if inserted {
      self.count += 1
    }
    inserted
  }

  pub fn remove(&mut self, object_handle: &ObjectHandle) -> bool {
    let removed = self.cards.remove(object_handle);

    if removed {
      self.count -= 1;
    }
    removed
  }

  pub fn incr(&mut self) {
    self.count += 1;
  }

  pub fn decr(&mut self) {
    self.count -= 1;
  }

  pub fn clone_visible(&self, f: impl FnMut(&ObjectHandle) -> bool) -> Self {
    let cards = self.cards.filter_clone(f);
    Self { cards, ..*self }
  }

  /// Get the zone's card count.
  pub fn count(&self) -> u32 {
    self.count
  }
}

impl<K> FromIterator<ObjectHandle> for Zone<K>
where
  K: ZoneKinded,
{
  fn from_iter<T: IntoIterator<Item = ObjectHandle>>(iter: T) -> Self {
    let mut count = 0;
    let cards = iter
      .into_iter()
      .map(|x| {
        count += 1;
        x
      })
      .collect();

    Self {
      cards,
      count,
      ..Default::default()
    }
  }
}

pub enum ViewedAs {
  Player,
  ControllerOfPlayer,
  Other,
}

pub trait CardCollection:
  Clone + Default + FromIterator<ObjectHandle> + Serialize + for<'de> Deserialize<'de> + SerdeDiff
{
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_>;

  fn insert(&mut self, object_handle: ObjectHandle) -> bool;

  fn remove(&mut self, object_handle: &ObjectHandle) -> bool;

  fn retain(&mut self, f: impl FnMut(&ObjectHandle) -> bool);

  fn filter_clone(&self, f: impl FnMut(&ObjectHandle) -> bool) -> Self;
}

impl CardCollection for HashSet<ObjectHandle> {
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_> {
    Box::new(self.iter())
  }

  fn insert(&mut self, object_handle: ObjectHandle) -> bool {
    self.insert(object_handle)
  }

  fn remove(&mut self, object_handle: &ObjectHandle) -> bool {
    self.remove(object_handle)
  }

  fn retain(&mut self, f: impl FnMut(&ObjectHandle) -> bool) {
    self.retain(f)
  }

  fn filter_clone(&self, mut f: impl FnMut(&ObjectHandle) -> bool) -> Self {
    self.iter().filter(|h| f(h)).cloned().collect()
  }
}

impl CardCollection for VecDeque<ObjectHandle> {
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_> {
    Box::new(self.iter())
  }

  fn insert(&mut self, object_handle: ObjectHandle) -> bool {
    checked_insert(self, object_handle, |vec, o| vec.push_front(o))
  }

  fn remove(&mut self, object_handle: &ObjectHandle) -> bool {
    let initial_len = self.len();
    self.retain(|o| *o != *object_handle);
    self.len() != initial_len
  }

  fn retain(&mut self, f: impl FnMut(&ObjectHandle) -> bool) {
    self.retain(f)
  }

  fn filter_clone(&self, mut f: impl FnMut(&ObjectHandle) -> bool) -> Self {
    self.iter().filter(|h| f(h)).cloned().collect()
  }
}

pub trait OrderedCardCollection {
  fn insert_at(&mut self, index: usize, object_handle: ObjectHandle) -> bool;

  fn remove_at(&mut self, index: usize) -> Option<ObjectHandle>;

  fn push_top(&mut self, object_handle: ObjectHandle) -> bool;

  fn push_bottom(&mut self, object_handle: ObjectHandle) -> bool;

  fn pop_top(&mut self) -> Option<ObjectHandle>;

  fn pop_bottom(&mut self) -> Option<ObjectHandle>;
}

impl OrderedCardCollection for VecDeque<ObjectHandle> {
  fn insert_at(&mut self, index: usize, object_handle: ObjectHandle) -> bool {
    checked_insert(self, object_handle, |vec, o| vec.insert(index, o))
  }

  fn remove_at(&mut self, index: usize) -> Option<ObjectHandle> {
    self.remove(index)
  }

  fn push_top(&mut self, object_handle: ObjectHandle) -> bool {
    checked_insert(self, object_handle, |vec, o| vec.push_front(o))
  }

  fn push_bottom(&mut self, object_handle: ObjectHandle) -> bool {
    checked_insert(self, object_handle, |vec, o| vec.push_back(o))
  }

  fn pop_top(&mut self) -> Option<ObjectHandle> {
    self.pop_front()
  }

  fn pop_bottom(&mut self) -> Option<ObjectHandle> {
    self.pop_back()
  }
}

impl<K> OrderedCardCollection for Zone<K>
where
  K: ZoneKinded,
  <K as ZoneKinded>::Collection: OrderedCardCollection,
{
  fn insert_at(&mut self, index: usize, object_handle: ObjectHandle) -> bool {
    let inserted = self.cards.insert_at(index, object_handle);
    if inserted {
      self.count += 1;
    }
    inserted
  }

  fn remove_at(&mut self, index: usize) -> Option<ObjectHandle> {
    let removed = self.cards.remove_at(index);
    if removed.is_some() {
      self.count -= 1;
    }
    removed
  }

  fn push_top(&mut self, object_handle: ObjectHandle) -> bool {
    let inserted = self.cards.push_top(object_handle);
    if inserted {
      self.count += 1;
    }
    inserted
  }

  fn push_bottom(&mut self, object_handle: ObjectHandle) -> bool {
    let inserted = self.cards.push_bottom(object_handle);
    if inserted {
      self.count += 1;
    }
    inserted
  }

  fn pop_top(&mut self) -> Option<ObjectHandle> {
    let removed = self.cards.pop_top();
    if removed.is_some() {
      self.count -= 1;
    }
    removed
  }

  fn pop_bottom(&mut self) -> Option<ObjectHandle> {
    let removed = self.cards.pop_bottom();
    if removed.is_some() {
      self.count -= 1;
    }
    removed
  }
}

/// Perform an inserting operation on a VecDeque that will rearrange elements if
/// necessary to perform the insert operation and maintain the invariant that
/// the item exists only once in the sequence.
fn checked_insert<T: PartialEq>(
  mut vec: &mut VecDeque<T>,
  item: T,
  operation: impl FnOnce(&mut VecDeque<T>, T),
) -> bool {
  let initial_len = vec.len();
  vec.retain(|o| *o != item);
  operation(&mut vec, item);
  vec.len() != initial_len
}
