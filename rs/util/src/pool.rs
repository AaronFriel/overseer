use std::rc::{Rc, Weak};

use serde::{Deserialize, Serialize};

#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Serialize,
  Deserialize,
  Default
)]
pub struct Handle {
  pub rc: Rc<()>,
  pub index: usize,
}

impl Handle {
  pub fn new(index: usize) -> Self {
    Self {
      index,
      rc: Rc::default(),
    }
  }
}

impl std::hash::Hash for Handle {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.index.hash(state)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Entry<T> {
  Virtual {
    real_index: usize,
    value: Rc<T>,
    #[serde(skip)]
    rc: Weak<()>,
  },
  Real {
    value: Rc<T>,
    #[serde(skip)]
    rc: Weak<()>,
  },
}

impl<T> Entry<T> {
  pub fn promote(&mut self) {
    use Entry::*;
    if self.is_virtual() {
      take_mut::take(self, |this| match this {
        Virtual { value, rc, .. } => Real { value, rc },
        otherwise => otherwise,
      });
    }
  }

  /// Returns `true` if the entry is [`Virtual`].
  pub fn is_virtual(&self) -> bool {
    matches!(self, Self::Virtual { .. })
  }
}

impl<T> Entry<T> {
  pub fn get_rc(&self) -> &Weak<()> {
    match self {
      Virtual { rc, .. } | Real { rc, .. } => &rc,
    }
  }

  pub fn get_value_rc(&self) -> &Rc<T> {
    match self {
      Virtual { value, .. } | Real { value, .. } => &value,
    }
  }

  pub fn get(&self) -> &T {
    match self {
      Virtual { value, .. } | Real { value, .. } => &value,
    }
  }

  pub fn get_mut(&mut self) -> &mut T
  where
    T: Clone,
  {
    match self {
      Virtual { ref mut value, .. } | Real { ref mut value, .. } => Rc::make_mut(value),
    }
  }

  pub fn ptr_eq(&self, other: &Entry<T>) -> bool {
    Rc::ptr_eq(self.get_value_rc(), other.get_value_rc())
  }
}
use Entry::*;

#[macro_export]
macro_rules! make_refcounted_pool {
  ($object_name:ident, $pool_name:ident, $struct_name:ident) => {
    paste::paste! {
      mod [<$struct_name:snake _impl>] {
        use std::rc::{Rc, Weak};
        #[cfg(test)]
        use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};

        use im::HashMap;
        use serde::{Deserialize, Serialize};

        use $crate::pool::{Entry, Handle};
        use Entry::*;

        use super::$object_name as Obj;

        #[cfg(test)]
        type Hasher = BuildHasherDefault<DefaultHasher>;

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $struct_name(Handle);

        #[derive(Clone, Debug)]
        #[derive(Serialize, Deserialize)]
        pub struct $pool_name {
          next_index: usize,
          #[cfg(test)]
          map: HashMap<usize, Entry<Obj>, Hasher>,
          #[cfg(not(test))]
          map: HashMap<usize, Entry<Obj>>,
        }

        impl $pool_name {
          pub fn new() -> Self {
            Self {
              next_index: 0,
              #[cfg(test)]
              map: HashMap::with_hasher(Hasher::default()),
              #[cfg(not(test))]
              map: HashMap::new(),
            }
          }

          fn next_index(&mut self) -> usize {
            let index = self.next_index;
            self.next_index += 1;
            index
          }

          pub fn insert(&mut self, object: Obj) -> $struct_name {
            let index = self.next_index();

            let handle = Handle::new(index);
            self.map.insert(index, Real {
              value: Rc::new(object),
              rc: Rc::downgrade(&handle.rc),
            });

            $struct_name(handle)
          }


          pub fn reinsert(&mut self, $struct_name(handle): &$struct_name) -> Option<$struct_name> {
            let index = handle.index;
            let new_handle_index = self.next_index();
            if let Some(current_entry) = self
              .map
              .get(&handle.index)
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == Rc::as_ptr(&handle.rc))
            {
              match current_entry {
                Virtual {
                  real_index: index,
                  value,
                  ..
                } => {
                  let handle = Handle::new(new_handle_index);
                  let value = Virtual {
                    value: value.clone(),
                    rc: Rc::downgrade(&handle.rc),
                    real_index: *index,
                  };
                  self.map.insert(new_handle_index, value);
                  Some($struct_name(handle))
                }
                Real { value, .. } => {
                  let handle = Handle::new(new_handle_index);
                  let value = Virtual {
                    value: value.clone(),
                    rc: Rc::downgrade(&handle.rc),
                    real_index: index,
                  };
                  self.map.insert(new_handle_index, value);
                  Some($struct_name(handle))
                }
              }
            } else {
              None
            }
          }

          pub fn get(&self, $struct_name(handle): &$struct_name) -> Option<&Obj> {
            self.map
              .get(&handle.index)
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == Rc::as_ptr(&handle.rc))
              .map(|entry| entry.get())
          }

          pub fn get_mut(&mut self, $struct_name(handle): &$struct_name) -> Option<&mut Obj> {
            self.map
              .get_mut(&handle.index)
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == Rc::as_ptr(&handle.rc))
              .map(|entry| {
                entry.promote();
                entry.get_mut()
              })
          }

          pub fn remove(&mut self, $struct_name(handle): &$struct_name) {
            self.map.remove(&handle.index);
          }

          pub fn reassociate(&mut self, $struct_name(handle): &mut $struct_name) -> bool {
            if let Some(inserted) = self.map.get(&handle.index) {
              if let Some(rc) = Weak::upgrade(inserted.get_rc()) {
                handle.rc = rc;
                return true;
              }
            };

            false
          }

          pub fn collect_garbage(&mut self) {
            for (k, entry) in self.map.clone().iter() {
              if Weak::strong_count(entry.get_rc()) == 0 {
                self.map.remove(k);
              }
            }
            let key_ref = self.map.clone();
            let mut deduplicated_keys = std::collections::HashMap::<usize, usize>::new();
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

          pub fn iter(&self) -> impl Iterator<Item = (usize, &Obj)> {
            self.map.iter().map(|(k, v)| {
              (*k, v.get())
            })
          }

          pub fn into_hashmap(&self) -> std::collections::HashMap<usize, &Obj> {
            self.map.iter().map(|(k, v)| {
              (*k, v.get())
            }).collect()
          }

          pub fn len(&self) -> usize {
            self.map.keys().len()
          }
        }
      }
      pub use [<$struct_name:snake _impl>]::$struct_name;
      pub use [<$struct_name:snake _impl>]::$pool_name;
    }
  };
}
