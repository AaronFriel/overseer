use std::{
  fmt::{Debug, Formatter},
  hash::{BuildHasher, Hash},
  hint::unreachable_unchecked,
  rc::{Rc, Weak},
};

use im::HashMap;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use Entry::*;

#[cfg(test)]
type DefaultHasher = nohash_hasher::BuildNoHashHasher<u64>;
#[cfg(not(test))]
type DefaultHasher = std::collections::hash_map::RandomState;

#[derive(Clone)]
pub struct Pool<T, S = DefaultHasher> {
  pub map: HashMap<usize, Entry<T>, S>,
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

impl<T> Pool<T>
where
  T: Clone,
{
  pub fn new() -> Self {
    Self {
      map: HashMap::default(),
    }
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

  pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
    self.map.iter().map(|(k, v)| (*k, v.get()))
  }

  pub fn into_hashmap(&self) -> std::collections::HashMap<usize, &T> {
    self.map.iter().map(|(k, v)| (*k, v.get())).collect()
  }

  pub fn len(&self) -> usize {
    self.map.keys().len()
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
    let map = HashMap::<usize, Entry<T>, S>::deserialize(deserializer)?;

    Ok(Self { map })
  }
}

trait Poollike<T, U> {}

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
  pub rc: Option<Rc<()>>,
  pub index: usize,
}

impl Handle {
  pub fn new(index: usize) -> Self {
    Self {
      index,
      rc: Some(Rc::default()),
    }
  }

  pub fn new_pair(index: usize) -> (Self, Weak<()>) {
    let rc = Rc::default();
    let weak = Rc::downgrade(&rc);
    (
      Self {
        index,
        rc: Some(rc),
      },
      weak,
    )
  }

  pub fn weak_clone(&self) -> Self {
    Self {
      index: self.index,
      rc: None,
    }
  }

  pub fn as_ptr(&self) -> *const () {
    match self.rc {
      Some(ref rc) => Rc::as_ptr(rc),
      None => std::ptr::null(),
    }
  }
}

impl Hash for Handle {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.index.hash(state)
  }
}

const fn const_none<T>() -> Option<T> {
  None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Entry<T> {
  Virtual {
    real_index: usize,
    /// Reference counted shared access to the underlying data.
    ///
    ///  We will never actually store a None here except during serde.
    #[serde(skip, default = "const_none")]
    value: Option<Rc<T>>,
    #[serde(skip)]
    rc: Weak<()>,
  },
  Shared {
    value: Rc<T>,
    #[serde(skip)]
    rc: Weak<()>,
  },
  Owned {
    value: T,
    #[serde(skip)]
    rc: Weak<()>,
  },
}

impl<T> Entry<T> {
  pub fn promote(&mut self) {
    use Entry::*;
    if self.is_virtual() {
      take_mut::take(self, |this| match this {
        Virtual { value, rc, .. } => Shared {
          /// SAFETY: Never a None stored.
          value: value.unwrap(),
          rc,
        },
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
      Virtual { rc, .. } | Shared { rc, .. } | Owned { rc, .. } => &rc,
    }
  }

  pub fn get_value_rc(&self) -> Option<&Rc<T>> {
    match self {
      Virtual { value, .. } => value.as_ref(),
      Shared { value, .. } => Some(&value),
      _ => None,
    }
  }

  pub fn get(&self) -> &T {
    match self {
      // SAFETY: Never a None stored.
      Virtual { value, .. } => value.as_ref().unwrap(),
      Shared { value, .. } => &value,
      Owned { value, .. } => &value,
    }
  }

  pub fn get_mut(&mut self) -> &mut T
  where
    T: Clone,
  {
    match self {
      // SAFETY: Never a None stored.
      Virtual { value, .. } => Rc::make_mut(value.as_mut().unwrap()),
      Shared { value, .. } => Rc::make_mut(value),
      Owned { value, .. } => value,
    }
  }

  pub fn ptr_eq(&self, other: &Entry<T>) -> bool {
    match (self.get_value_rc(), other.get_value_rc()) {
      (None, None) => false,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(this), Some(other)) => Rc::ptr_eq(this, other),
    }
  }

  pub fn share(&mut self, index: usize) -> (&Rc<T>, usize) {
    match self {
      Virtual {
        value, real_index, ..
      } => (value.as_ref().unwrap(), *real_index),
      Shared { value, .. } => (value, index),
      Owned { .. } => {
        take_mut::take(self, |this| {
          if let Owned { value, rc } = this {
            Shared {
              value: Rc::new(value),
              rc,
            }
          } else {
            // Safety value can only be Owned.
            unsafe { unreachable_unchecked() }
          }
        });

        // 3 Value replaced with Owned.
        (self.get_value_rc().unwrap(), index)
      }
    }
  }
}

impl<T> Hash for Entry<T>
where
  T: Hash,
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.get().hash(state)
  }
}

#[macro_export]
macro_rules! make_refcounted_pool {
  ($object_name:ident, $pool_name:ident, $struct_name:ident) => {
    paste::paste! {
      mod [<$struct_name:snake _impl>] {
        use std::rc::Weak;

        use $crate::deps::serde::{Deserialize, Serialize};

        use super::$object_name as Obj;

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $struct_name($crate::pool::Handle);

        impl $struct_name {
          pub fn weak_clone(&self) -> Self {
            $struct_name(self.0.weak_clone())
          }
        }

        #[derive(Clone, Debug, Hash)]
        #[derive(Serialize, Deserialize)]
        pub struct $pool_name {
          next_index: usize,
          pool: $crate::pool::Pool<Obj>,
        }

        impl $pool_name {
          pub fn new() -> Self {
            Self {
              next_index: 0,
              pool: $crate::pool::Pool::new(),
            }
          }

          fn next_index(&mut self) -> usize {
            let index = self.next_index;
            self.next_index += 1;
            index
          }

          pub fn insert(&mut self, object: Obj) -> $struct_name {
            let index = self.next_index();
            let (handle, weak) = $crate::pool::Handle::new_pair(index);
            self
              .pool
              .map
              .insert(index, $crate::pool::Entry::Owned {
                value: object,
                rc: weak,
              });

            $struct_name(handle)
          }


          pub fn reinsert(&mut self, $struct_name(handle): &$struct_name) -> Option<$struct_name> {
            let index = handle.index;
            let new_handle_index = self.next_index();
            if let Some(current_entry) = self
              .pool
              .map
              .get_mut(&handle.index)
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == handle.as_ptr())
            {
              match current_entry {
                $crate::pool::Entry::Virtual { .. } => {
                  let (handle, weak) = $crate::pool::Handle::new_pair(new_handle_index);
                  let (value, index) = current_entry.share(index);
                  let inserted = $crate::pool::Entry::Virtual {
                    value: Some(value.clone()),
                    rc: weak,
                    real_index: index,
                  };
                  self.pool.map.insert(new_handle_index, inserted);
                  Some($struct_name(handle))
                }
                $crate::pool::Entry::Shared { .. } => {
                  let (handle, weak) = $crate::pool::Handle::new_pair(new_handle_index);
                  let (value, index) = current_entry.share(index);
                  let inserted = $crate::pool::Entry::Virtual {
                    value: Some(value.clone()),
                    rc: weak,
                    real_index: index,
                  };
                  self.pool.map.insert(new_handle_index, inserted);
                  Some($struct_name(handle))
                }
                $crate::pool::Entry::Owned { .. } => {
                  let (handle, weak) = $crate::pool::Handle::new_pair(new_handle_index);
                  let (value, index) = current_entry.share(index);
                  let inserted = $crate::pool::Entry::Virtual {
                    value: Some(value.clone()),
                    rc: weak,
                    real_index: index,
                  };
                  self.pool.map.insert(new_handle_index, inserted);
                  Some($struct_name(handle))
                }
              }
            } else {
              None
            }
          }

          pub fn get(&self, $struct_name(handle): &$struct_name) -> Option<&Obj> {
            self.pool.map
              .get(&handle.index)
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == handle.as_ptr())
              .map(|entry| entry.get())
          }

          pub fn get_mut(&mut self, $struct_name(handle): &$struct_name) -> Option<&mut Obj> {
            self.pool.map
              .get_mut(&handle.index)
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == handle.as_ptr())
              .map(|entry| {
                entry.promote();
                entry.get_mut()
              })
          }

          pub fn remove(&mut self, $struct_name(handle): &$struct_name) {
            self.pool.map.remove(&handle.index);
          }

          pub fn reassociate(&mut self, $struct_name(handle): &mut $struct_name) -> bool {
            if let Some(inserted) = self.pool.map.get(&handle.index) {
              if let Some(rc) = Weak::upgrade(inserted.get_rc()) {
                handle.rc = Some(rc);
                return true;
              }
            };

            false
          }

          pub fn collect_garbage(&mut self) {
            for (k, entry) in self.pool.map.clone().iter() {
              if Weak::strong_count(entry.get_rc()) == 0 {
                self.pool.map.remove(k);
              }
            }
            let key_ref = self.pool.map.clone();
            let mut deduplicated_keys = std::collections::HashMap::<usize, usize>::new();
            for (k, entry) in self.pool.map.iter_mut() {
              match entry {
                $crate::pool::Entry::Virtual { mut real_index, .. }
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
            self.pool.map.iter().map(|(k, v)| {
              (*k, v.get())
            })
          }

          pub fn iter_weak(&self) -> impl Iterator<Item = ($struct_name, &Obj)> {
            self.pool.map.iter().map(|(k, v)| {
              ($struct_name($crate::pool::Handle {
                index: *k,
                rc: None,
              }), v.get())
            })
          }

          pub fn into_hashmap(&self) -> std::collections::HashMap<usize, &Obj> {
            self.pool.map.iter().map(|(k, v)| {
              (*k, v.get())
            }).collect()
          }

          pub fn len(&self) -> usize {
            self.pool.map.keys().len()
          }
        }
      }
      pub use [<$struct_name:snake _impl>]::$struct_name;
      pub use [<$struct_name:snake _impl>]::$pool_name;
    }
  };
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;

  use insta::assert_yaml_snapshot;

  use crate::make_refcounted_pool;

  #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
  #[derive(serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff)]
  pub struct FooObject {
    #[serde_diff(opaque)]
    name: Cow<'static, str>,
  }

  make_refcounted_pool!(FooObject, FooPool, FooHandle);

  #[test]
  fn test_sizeof() {
    assert_eq!(
      std::mem::size_of::<FooHandle>(),
      std::mem::size_of::<usize>() * 2
    );

    assert_eq!(
      std::mem::size_of::<FooPool>(),
      std::mem::size_of::<usize>() * 4
    );
  }

  #[test]
  fn test_insert() {
    let mut layer1 = FooPool::new();

    let obj_one = FooObject {
      name: Cow::Borrowed("one"),
    };
    let handle_one = layer1.insert(obj_one);

    assert_eq!(
      layer1.get(&handle_one),
      Some(&FooObject {
        name: Cow::Borrowed("one")
      })
    );
  }

  #[test]
  fn test_overwrite() {
    let mut layer1 = FooPool::new();

    let obj_one = FooObject {
      name: Cow::Borrowed("one"),
    };
    let handle_one = layer1.insert(obj_one);

    {
      let mut obj_two = layer1.get_mut(&handle_one.clone()).unwrap();
      obj_two.name = Cow::Borrowed("two");
    }

    assert_eq!(
      layer1.get(&handle_one),
      Some(&FooObject {
        name: Cow::Borrowed("two")
      })
    );
  }

  /// Test that we can create an object pool, create handles (references) to
  /// objects in that pool, and we can clone the pool to create layers and
  /// references are tracked.
  #[cfg(test)]
  #[test]
  fn test_gc() {
    let mut layer1 = FooPool::new();

    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let handle_one = layer1.insert(make_obj("one")); // We will drop this handle first.
    let handle_two = layer1.insert(make_obj("TWO")); // We will drop this second.

    let mut layer2 = layer1.clone();
    layer2.get_mut(&handle_one).unwrap().name = Cow::Borrowed("_one_");
    let handle_three = layer2.insert(make_obj("3")); // We'll hold on to this until the end
    let handle_two_clone = handle_two.clone(); // This will keep the reference to object TWO alive to the end.

    // The mutation and new value are only visible in layer 2:
    assert_yaml_snapshot!((&layer1, &layer2), @r###"
    ---
    - next_index: 2
      pool:
        0:
          type: Owned
          value:
            name: one
        1:
          type: Owned
          value:
            name: TWO
    - next_index: 3
      pool:
        0:
          type: Owned
          value:
            name: _one_
        1:
          type: Owned
          value:
            name: TWO
        2:
          type: Owned
          value:
            name: "3"
    "###);

    // Dropping the handle allows garbage collection to proceed in any layer:
    // a reference counter verifies no outstanding handles exist.
    std::mem::drop(handle_one);

    // Drops don't do anything on their own, though, nothing has changed yet:
    assert_eq!((layer1.len(), layer2.len()), (2, 3));
    assert_yaml_snapshot!((&layer1, &layer2), @r###"
    ---
    - next_index: 2
      pool:
        1:
          type: Owned
          value:
            name: TWO
    - next_index: 3
      pool:
        1:
          type: Owned
          value:
            name: TWO
        2:
          type: Owned
          value:
            name: "3"
    "###);

    // Garbage collecting a layer removes values with no handles.
    layer1.collect_garbage();

    // However that only affects the one layer:
    assert_yaml_snapshot!((&layer1, &layer2), @r###"
    ---
    - next_index: 2
      pool:
        1:
          type: Owned
          value:
            name: TWO
    - next_index: 3
      pool:
        1:
          type: Owned
          value:
            name: TWO
        2:
          type: Owned
          value:
            name: "3"
    "###);

    // Dropping handle_two won't do anything yet because we have a clone of it, the
    // pools know of this due to the shared reference counter. We can decrement
    // the handle count by one here:
    std::mem::drop(handle_two);

    layer2.collect_garbage();
    assert_eq!((layer1.len(), layer2.len()), (1, 2));
    assert_yaml_snapshot!((&layer1, &layer2), @r###"
    ---
    - next_index: 2
      pool:
        1:
          type: Owned
          value:
            name: TWO
    - next_index: 3
      pool:
        1:
          type: Owned
          value:
            name: TWO
        2:
          type: Owned
          value:
            name: "3"
    "###);

    // Finally, we drop all the remaining handles and clean up:
    std::mem::drop(handle_three);
    std::mem::drop(handle_two_clone);
    layer1.collect_garbage();
    layer2.collect_garbage();

    assert_yaml_snapshot!((&layer1, &layer2), @r###"
    ---
    - next_index: 2
      pool: {}
    - next_index: 3
      pool: {}
    "###);
  }

  /// Test that handles cannot be used across different pools, in case an object
  /// is inserted in a lower layer.
  #[test]
  fn test_handle_safety() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool1 = FooPool::new();
    let handle_one = pool1.insert(make_obj("one"));

    let mut layer1 = FooPool::new();
    let handle_two = layer1.insert(make_obj("TWO"));

    // Handles must be from the pool they were created from.
    assert_eq!(pool1.get(&handle_two), None);
    assert_eq!(layer1.get(&handle_one), None);

    let mut layer2 = layer1.clone();

    let layer1_handle = layer1.insert(make_obj("3"));
    let layer2_handle = layer2.insert(make_obj("3"));

    // This applies to querying handles created in a prior layer after it was
    // cloned:
    assert_eq!(layer1.get(&layer2_handle), None);
    assert_eq!(layer2.get(&layer1_handle), None);

    // If we use the right handles, we can get values:
    assert_yaml_snapshot!(layer1.get(&layer1_handle), @r###"
    ---
    name: "3"
    "###);
    assert_yaml_snapshot!(layer2.get(&layer2_handle), @r###"
    ---
    name: "3"
    "###);

    // And these values are structurally equal, even if they are from different
    // pools:
    assert_eq!(layer1_handle, layer2_handle);
  }

  /// Test that handles cannot be used across different pools, in case an object
  /// is inserted in a lower layer.
  #[test]
  fn test_reinsert() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool1 = FooPool::new();
    let handle_one = pool1.insert(make_obj("one"));
    assert_yaml_snapshot!((&pool1), @r###"
    ---
    next_index: 1
    pool:
      0:
        type: Owned
        value:
          name: one
    "###);

    let handle_one_cow = pool1.reinsert(&handle_one).unwrap();
    let handle_one_cow_2 = pool1.reinsert(&handle_one_cow).unwrap();

    let mut layer1 = FooPool::new();
    let handle_two = layer1.insert(make_obj("TWO"));
    let handle_two_cow = layer1.reinsert(&handle_two).unwrap();

    assert_eq!(pool1.get(&handle_two_cow), None);
    assert_eq!(layer1.get(&handle_one_cow), None);

    assert_eq!(pool1.get(&handle_one), pool1.get(&handle_one_cow));
    assert_eq!(layer1.get(&handle_two), layer1.get(&handle_two_cow));

    assert_yaml_snapshot!((&pool1), @r###"
    ---
    next_index: 3
    pool:
      0:
        type: Shared
        value:
          name: one
      1:
        type: Virtual
        real_index: 0
      2:
        type: Virtual
        real_index: 0
    "###);

    let mut obj_two = pool1.get_mut(&handle_one_cow).unwrap();
    obj_two.name = "TWO".into();
    let mut obj_three = pool1.get_mut(&handle_one_cow_2).unwrap();
    obj_three.name = "tres".into();

    assert_yaml_snapshot!((&pool1), @r###"
    ---
    next_index: 3
    pool:
      0:
        type: Shared
        value:
          name: one
      1:
        type: Shared
        value:
          name: TWO
      2:
        type: Shared
        value:
          name: tres
    "###);
  }
}
