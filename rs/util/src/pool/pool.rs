use std::{
  fmt::{Debug, Formatter},
  hash::{BuildHasher, Hash},
  num::NonZeroU128,
  rc::{Rc, Weak},
};

use im::HashMap;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::{Handle, Item, Item::*};

#[cfg(test)]
pub type DefaultHasher = std::hash::BuildHasherDefault<hashers::null::PassThroughHasher>;
#[cfg(not(test))]
pub type DefaultHasher = std::collections::hash_map::RandomState;

#[derive(Clone)]
pub struct Pool<T, S = DefaultHasher> {
  pub map: HashMap<Uuid, Item<T>, S>,
}

pub struct DisassociatedPool<T, S = DefaultHasher> {
  pub map: HashMap<Uuid, Item<T>, S>,
  pub handles: std::collections::HashMap<Uuid, Handle>,
}

impl<T, S> Debug for Pool<T, S>
where
  T: Debug,
  S: BuildHasher,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self.map, f)
  }
}

impl<T, S> Hash for Pool<T, S>
where
  T: Hash,
  S: BuildHasher,
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.map.hash(state)
  }
}

impl<T, S> Pool<T, S>
where
  T: Clone,
  S: BuildHasher + Default,
{
  pub fn new() -> Self {
    Self {
      map: Default::default(),
    }
  }

  pub fn collect_garbage(&mut self) {
    for (k, entry) in self.map.clone().iter() {
      if Weak::strong_count(entry.get_rc()) == 0 {
        self.map.remove(k);
      }
    }
    let key_ref = self.map.clone();
    let mut deduplicated_keys = std::collections::HashMap::<Uuid, Uuid>::new();
    for (k, entry) in self.map.iter_mut() {
      match entry {
        Virtual { mut real_index, .. }
          if key_ref
            .get(&real_index)
            .map_or(true, |ref real_entry| !real_entry.ptr_eq(entry)) =>
        {
          if let Some(new_real_index) = deduplicated_keys.get(&real_index) {
            *&mut real_index = *new_real_index;
          } else {
            deduplicated_keys.insert(real_index, *k);
            entry.promote();
          };
        }
        _ => {}
      }
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = (Uuid, &T)> {
    self.map.iter().map(|(k, v)| (*k, v.get()))
  }

  pub fn into_hashmap(&self) -> std::collections::HashMap<Uuid, &T> {
    self.map.iter().map(|(k, v)| (*k, v.get())).collect()
  }

  pub fn len(&self) -> usize {
    self.map.len()
  }

  pub fn to_disassociated_pool(self) -> DisassociatedPool<T, S> {
    let mut map = self.map;
    let mut handles: std::collections::HashMap<Uuid, Handle> = Default::default();

    let cloned_map = map.clone();

    for (k, v) in map.iter_mut() {
      match v {
        Virtual {
          real_index,
          value: virtual_value,
          ..
        } => {
          // Safety: real_index must invariably point at a live value:
          let real_entry = cloned_map.get(real_index).unwrap();
          match real_entry {
            Shared {
              value: real_value, ..
            } => {
              // Our iterator reached the virtual before the shared, we don't need to create a
              // handle for both, but we do need to link the values.
              *virtual_value = Some(Rc::clone(real_value));
            }
            _ => unreachable!(),
          }
        }
        _ => {}
      }

      let rc = v.get_rc_mut();
      let (handle, weak) = Handle::new_pair(unsafe { NonZeroU128::new_unchecked(k.as_u128()) });
      handles.insert(*k, handle);
      *rc = weak;
    }

    DisassociatedPool { map, handles }
  }
}

impl<T> DisassociatedPool<T>
where
  T: Clone,
{
  pub fn reassociate<'a>(self, handles: impl Iterator<Item = &'a Handle>) -> Pool<T> {
    for h in handles {
      let index = h.get_index();

      if let Some(existing) = self.handles.get(&index) {
        unsafe {
          // No other mutation to the rc occurs, we have exclusive access to self.
          let h_rc = &mut *h.rc.get();
          *h_rc = (&*existing.rc.get()).clone();
        }
      } else {
        // TODO: this should be an error
      }
    }
    std::mem::drop(self.handles);

    let mut pool = Pool { map: self.map };
    pool.collect_garbage();
    pool
  }
}

impl<T> Serialize for Pool<T>
where
  T: Serialize + Clone,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut this = self.clone();
    this.collect_garbage();
    this.map.serialize(serializer)
  }
}

impl<'de, T, S> Deserialize<'de> for Pool<T, S>
where
  T: Deserialize<'de> + Clone,
  S: BuildHasher + Default,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let map = HashMap::<Uuid, Item<T>, S>::deserialize(deserializer)?;

    Ok(Self { map })
  }
}

impl<'de, T, S> Deserialize<'de> for DisassociatedPool<T, S>
where
  T: Deserialize<'de> + Clone,
  S: BuildHasher + Default,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let map = HashMap::<Uuid, Item<T>, S>::deserialize(deserializer)?;
    Ok(Pool { map }.to_disassociated_pool())
  }
}
