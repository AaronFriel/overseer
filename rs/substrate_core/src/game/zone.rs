use std::{
  collections::{HashSet, VecDeque},
  iter::FromIterator,
  marker::PhantomData,
};

use serde::{Deserialize, Serialize};

use crate::game::ObjectHandle;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Zone<K>
where
  K: ZoneKinded,
{
  objects: K::Collection,
  count: u32,
  #[serde(skip)]
  kind: PhantomData<K>,
}

impl<K> std::hash::Hash for Zone<K>
where
  K: ZoneKinded,
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.objects.iter().for_each(|x| x.hash(state));
    self.count.hash(state);
  }
}

pub trait ZoneKinded:
  Clone + Eq + PartialEq + PartialOrd + std::hash::Hash + std::fmt::Debug + Default
{
  const KIND: Zones;
  type Collection: ObjectCollection;
}

impl ZoneKinded for Hand {
  type Collection = HashSet<ObjectHandle>;

  const KIND: Zones = Zones::Hand;
}
impl ZoneKinded for Library {
  type Collection = VecDeque<Option<ObjectHandle>>;

  const KIND: Zones = Zones::Library;
}
impl ZoneKinded for Battlefield {
  type Collection = HashSet<ObjectHandle>;

  const KIND: Zones = Zones::Battlefield;
}
impl ZoneKinded for Graveyard {
  type Collection = VecDeque<Option<ObjectHandle>>;

  const KIND: Zones = Zones::Graveyard;
}
impl ZoneKinded for Stack {
  type Collection = VecDeque<Option<ObjectHandle>>;

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
      objects: Default::default(),
      count: 0,
      kind: PhantomData,
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &ObjectHandle> {
    self.objects.iter()
  }

  pub fn iter_weak(&self) -> impl Iterator<Item = ObjectHandle> + '_ {
    self.objects.iter().map(ObjectHandle::weak_clone)
  }

  pub fn insert(&mut self, object: ObjectHandle) -> bool {
    let inserted = self.objects.insert(object);
    debug_assert!(inserted);
    self.count += 1;
    inserted
  }

  pub fn remove(&mut self, object: &ObjectHandle) -> bool {
    let removed = self.objects.remove(object);
    debug_assert!(removed);
    self.count -= 1;
    removed
  }

  pub fn incr(&mut self) {
    self.count += 1;
  }

  pub fn decr(&mut self) {
    self.count -= 1;
  }

  /// Given a filter, return a filtered view that retains the original count.
  pub fn into_filtered_view(&self, mut f: impl FnMut(&ObjectHandle) -> bool) -> Self {
    let objects = self
      .objects
      .iter()
      .filter(|o| f(*o))
      .map(|o| o.weak_clone())
      .collect();
    Self { objects, ..*self }
  }

  /// Get the zone's object count.
  pub fn count(&self) -> u32 {
    self.count
  }
}

impl<K> FromIterator<ObjectHandle> for Zone<K>
where
  K: ZoneKinded,
{
  fn from_iter<T: IntoIterator<Item = ObjectHandle>>(iter: T) -> Self {
    let objects: <K as ZoneKinded>::Collection = iter.into_iter().collect();
    let count = objects.count();

    Self {
      objects,
      count,
      ..Default::default()
    }
  }
}

pub trait ObjectCollection:
  Clone + Default + FromIterator<ObjectHandle> + Serialize + for<'de> Deserialize<'de>
{
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_>;

  fn count(&self) -> u32;

  fn insert(&mut self, object: ObjectHandle) -> bool;

  fn remove(&mut self, object: &ObjectHandle) -> bool;

  fn clear(&mut self);

  fn into_zone<K>(self) -> Zone<K>
  where
    K: ZoneKinded<Collection = Self>,
  {
    let count = self.count();
    Zone {
      objects: self,
      count,
      kind: Default::default(),
    }
  }
}

impl ObjectCollection for HashSet<ObjectHandle> {
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_> {
    Box::new(self.iter())
  }

  fn count(&self) -> u32 {
    self.len() as u32
  }

  fn insert(&mut self, object: ObjectHandle) -> bool {
    self.insert(object)
  }

  fn remove(&mut self, object: &ObjectHandle) -> bool {
    self.remove(object)
  }

  fn clear(&mut self) {
    self.clear()
  }
}

impl ObjectCollection for VecDeque<Option<ObjectHandle>> {
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_> {
    Box::new(self.iter().filter_map(|x| x.as_ref()))
  }

  fn count(&self) -> u32 {
    self.len() as u32
  }

  fn insert(&mut self, object: ObjectHandle) -> bool {
    checked_insert(self, Some(object), |vec, o| vec.push_front(o))
  }

  fn remove(&mut self, object: &ObjectHandle) -> bool {
    let initial_len = self.len();
    self.retain(|o| o.as_ref().map_or(true, |o| *o == *object));
    self.len() != initial_len
  }

  fn clear(&mut self) {
    self.clear()
  }
}

impl FromIterator<ObjectHandle> for VecDeque<Option<ObjectHandle>> {
  fn from_iter<T: IntoIterator<Item = ObjectHandle>>(iter: T) -> Self {
    Self::from_iter(iter.into_iter().map(|x| Some(x)))
  }
}

impl<K> ObjectCollection for Zone<K>
where
  K: ZoneKinded,
  <K as ZoneKinded>::Collection: ObjectCollection,
{
  fn iter(&self) -> Box<dyn Iterator<Item = &ObjectHandle> + '_> {
    self.objects.iter()
  }

  fn count(&self) -> u32 {
    self.count
  }

  fn insert(&mut self, object: ObjectHandle) -> bool {
    let inserted = self.objects.insert(object);
    debug_assert!(inserted);
    self.count += 1;
    inserted
  }

  fn remove(&mut self, object: &ObjectHandle) -> bool {
    let removed = self.objects.remove(object);
    debug_assert!(removed);
    self.count -= 1;
    removed
  }

  fn clear(&mut self) {
    self.objects.clear()
  }
}

pub trait OrderedObjectCollection {
  fn get_top(&self, index: usize) -> Option<&ObjectHandle>;

  fn get_bottom(&self, index: usize) -> Option<&ObjectHandle>;

  fn insert_top(&mut self, index: usize, object: ObjectHandle) -> bool;

  fn insert_bottom(&mut self, index: usize, object: ObjectHandle) -> bool;

  fn remove_at(&mut self, index: usize) -> Option<ObjectHandle>;

  fn push_top(&mut self, object: Option<ObjectHandle>) -> bool;

  fn push_bottom(&mut self, object: Option<ObjectHandle>) -> bool;

  fn pop_top(&mut self) -> Option<ObjectHandle>;

  fn pop_bottom(&mut self) -> Option<ObjectHandle>;
}

impl OrderedObjectCollection for VecDeque<Option<ObjectHandle>> {
  fn get_top(&self, index: usize) -> Option<&ObjectHandle> {
    self.get(index).and_then(|x| x.as_ref())
  }

  fn get_bottom(&self, index: usize) -> Option<&ObjectHandle> {
    self
      .get(self.len().wrapping_sub(1 + index))
      .and_then(|x| x.as_ref())
  }

  fn insert_top(&mut self, index: usize, object: ObjectHandle) -> bool {
    checked_insert(self, Some(object), |vec, o| vec.insert(index, o))
  }

  fn insert_bottom(&mut self, index: usize, object: ObjectHandle) -> bool {
    let index = self.len().wrapping_sub(index);
    checked_insert(self, Some(object), |vec, o| vec.insert(index, o))
  }

  fn remove_at(&mut self, index: usize) -> Option<ObjectHandle> {
    self.remove(index).and_then(|x| x)
  }

  fn push_top(&mut self, object: Option<ObjectHandle>) -> bool {
    checked_insert(self, object, |vec, o| vec.push_front(o))
  }

  fn push_bottom(&mut self, object: Option<ObjectHandle>) -> bool {
    checked_insert(self, object, |vec, o| vec.push_back(o))
  }

  fn pop_top(&mut self) -> Option<ObjectHandle> {
    self.pop_front().and_then(|x| x)
  }

  fn pop_bottom(&mut self) -> Option<ObjectHandle> {
    self.pop_back().and_then(|x| x)
  }
}

impl<K> OrderedObjectCollection for Zone<K>
where
  K: ZoneKinded,
  <K as ZoneKinded>::Collection: OrderedObjectCollection,
{
  fn get_top(&self, index: usize) -> Option<&ObjectHandle> {
    self.objects.get_top(index)
  }

  fn get_bottom(&self, index: usize) -> Option<&ObjectHandle> {
    self.objects.get_bottom(index)
  }

  fn insert_top(&mut self, index: usize, object: ObjectHandle) -> bool {
    let inserted = self.objects.insert_top(index, object);
    debug_assert!(inserted);
    self.count += 1;
    inserted
  }

  fn insert_bottom(&mut self, index: usize, object: ObjectHandle) -> bool {
    let inserted = self.objects.insert_bottom(index, object);
    debug_assert!(inserted);
    self.count += 1;
    inserted
  }

  fn remove_at(&mut self, index: usize) -> Option<ObjectHandle> {
    let removed = self.objects.remove_at(index);
    if removed.is_some() {
      self.count -= 1;
    }
    removed
  }

  fn push_top(&mut self, object: Option<ObjectHandle>) -> bool {
    let inserted = self.objects.push_top(object);
    debug_assert!(inserted);
    self.count += 1;
    inserted
  }

  fn push_bottom(&mut self, object: Option<ObjectHandle>) -> bool {
    let inserted = self.objects.push_bottom(object);
    debug_assert!(inserted);
    self.count += 1;
    inserted
  }

  fn pop_top(&mut self) -> Option<ObjectHandle> {
    let removed = self.objects.pop_top();
    if removed.is_some() {
      self.count -= 1;
    }
    removed
  }

  fn pop_bottom(&mut self) -> Option<ObjectHandle> {
    let removed = self.objects.pop_bottom();
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
