use std::{
  fmt::{Debug, Formatter},
  hash::{BuildHasher, Hash},
  hint::unreachable_unchecked,
  num::NonZeroU128,
  rc::{Rc, Weak},
};

use im::HashMap;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use uuid::Uuid;
use Entry::*;

#[cfg(test)]
type DefaultHasher = std::hash::BuildHasherDefault<hashers::null::PassThroughHasher>;
#[cfg(not(test))]
type DefaultHasher = std::collections::hash_map::RandomState;

#[derive(Clone)]
pub struct Pool<T, S = DefaultHasher> {
  pub map: HashMap<Uuid, Entry<T>, S>,
}

pub struct DisassociatedPool<T, S = DefaultHasher> {
  pub map: HashMap<Uuid, Entry<T>, S>,
  pub handles: std::collections::HashMap<Uuid, Handle>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonZeroUuid(pub NonZeroU128);

impl Serialize for NonZeroUuid {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Uuid::from_u128(self.0.get()).serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for NonZeroUuid {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    if let Some(value) = NonZeroU128::new(Uuid::deserialize(deserializer)?.as_u128()) {
      Ok(NonZeroUuid(value))
    } else {
      Err(serde::de::Error::invalid_value(
        serde::de::Unexpected::Unsigned(0),
        &"non-zero value",
      ))
    }
  }
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
}

impl<T> DisassociatedPool<T>
where
  T: Clone,
{
  pub fn reassociate<'a>(self, handles: impl Iterator<Item = &'a mut Handle>) -> Pool<T> {
    for h in handles {
      let index = h.get_index();

      if let Some(other) = self.handles.get(&index) {
        h.rc = other.rc.clone();
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

impl<'de, T, S> Deserialize<'de> for DisassociatedPool<T, S>
where
  T: Deserialize<'de> + Clone,
  S: BuildHasher + Default,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let mut map = HashMap::<Uuid, Entry<T>, S>::deserialize(deserializer)?;
    let mut unassociated_handles: std::collections::HashMap<Uuid, Handle> = Default::default();

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
              *virtual_value = Some(real_value.clone());
            }
            _ => unreachable!(),
          }
        }
        _ => {}
      }

      let rc = v.get_rc_mut();
      let (handle, weak) = Handle::new_pair(unsafe { NonZeroU128::new_unchecked(k.as_u128()) });
      unassociated_handles.insert(*k, handle);
      *rc = weak;
    }

    Ok(Self {
      map,
      handles: unassociated_handles,
    })
  }
}

trait Poollike<T, U> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Handle {
  pub index: NonZeroUuid,
  #[serde(skip)]
  pub rc: Option<Rc<()>>,
}

impl Handle {
  pub fn new(index: NonZeroU128) -> Self {
    Self {
      index: NonZeroUuid(index),
      rc: Some(Rc::default()),
    }
  }

  pub fn get_index(&self) -> Uuid {
    Uuid::from_u128(self.index.0.get())
  }

  pub fn new_pair(index: NonZeroU128) -> (Self, Weak<()>) {
    let rc = Rc::default();
    let weak = Rc::downgrade(&rc);
    (
      Self {
        index: NonZeroUuid(index),
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
    real_index: Uuid,
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
    take_mut::take(self, |this| match this {
      Virtual { value, rc, .. } => {
        // SAFETY: Never a none stored.
        let value = value.unwrap();
        match Rc::try_unwrap(value) {
          Ok(value) => Owned { value, rc },
          Err(value) => Shared { value, rc },
        }
      }
      Shared { value, rc } => match Rc::try_unwrap(value) {
        Ok(value) => Owned { value, rc },
        Err(value) => Shared { value, rc },
      },
      otherwise => otherwise,
    });
  }

  /// Returns `true` if the entry is [`Virtual`].
  pub fn is_virtual(&self) -> bool {
    matches!(self, Self::Virtual { .. })
  }
}

impl<T> Entry<T> {
  pub fn get_rc(&self) -> &Weak<()> {
    match self {
      Virtual { rc, .. } | Shared { rc, .. } | Owned { rc, .. } => rc,
    }
  }

  pub fn get_rc_mut(&mut self) -> &mut Weak<()> {
    match self {
      Virtual { rc, .. } | Shared { rc, .. } | Owned { rc, .. } => rc,
    }
  }

  pub fn set_rc(&mut self, value: Weak<()>) {
    match self {
      Virtual { rc, .. } => *rc = value,
      Shared { rc, .. } => *rc = value,
      Owned { rc, .. } => *rc = value,
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
    take_mut::take(self, |this| match this {
      Virtual { value, rc, .. } => Owned {
        value: value.as_ref().unwrap().as_ref().clone(),
        rc,
      },
      Shared { value, rc } => Owned {
        value: value.as_ref().clone(),
        rc,
      },
      owned => owned,
    });

    match self {
      Owned { value, .. } => value,
      // SAFETY: Inner value converted to owned.
      _ => unsafe { unreachable_unchecked() },
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

  pub fn share(&mut self, index: Uuid) -> (&Rc<T>, Uuid) {
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
        use std::num::NonZeroU128;

        use $crate::deps::serde::{Deserialize, Serialize};

        use super::$object_name as Obj;

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[derive(Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $struct_name($crate::pool::Handle);

        impl $struct_name {
          pub fn weak_clone(&self) -> Self {
            $struct_name(self.0.weak_clone())
          }
        }

        #[derive(Clone, Debug, Hash)]
        #[derive(Serialize)]
        #[serde(transparent)]
        pub struct $pool_name {
          pool: $crate::pool::Pool<Obj>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        pub struct [<Disassociated $pool_name:camel>] {
          pool: $crate::pool::DisassociatedPool<Obj>,
        }

        impl $pool_name {
          pub fn new() -> Self {
            Self {
              pool: $crate::pool::Pool::new(),
            }
          }

          #[cfg(test)]
          fn next_index(&mut self) -> u128 {
            (self.len() + 1) as u128
          }

          #[cfg(not(test))]
          fn next_index(&mut self) -> u128 {
            let index = $crate::deps::uuid::Uuid::new_v4();
            index.as_u128()
          }

          fn get_next_handle(&mut self) -> ($crate::pool::Handle, Weak<()>) {
            let index = self.next_index();
            // SAFETY: Index starts at 1
            $crate::pool::Handle::new_pair(unsafe { NonZeroU128::new_unchecked(index) })
          }

          pub fn insert(&mut self, object: Obj) -> $struct_name {
            let (handle, weak) = self.get_next_handle();
            let index = handle.get_index();
            self
              .pool
              .map
              .insert(index, $crate::pool::Entry::Owned {
                value: object,
                rc: weak,
              });

            $struct_name(handle)
          }


          pub fn reinsert(&mut self, $struct_name(original_handle): &$struct_name) -> Option<$struct_name> {
            let original_index = original_handle.get_index();
            let (new_handle, rc) = self.get_next_handle();
            let new_handle_index = new_handle.get_index();
            if let Some(current_entry) = self
              .pool
              .map
              .get_mut(&original_handle.get_index())
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == original_handle.as_ptr())
            {
              let (value, real_index) = current_entry.share(original_index);
              let inserted = $crate::pool::Entry::Virtual {
                value: Some(value.clone()),
                rc,
                real_index,
              };
              self.pool.map.insert(new_handle_index, inserted);
              Some($struct_name(new_handle))
            } else {
              None
            }
          }

          pub fn get(&self, $struct_name(handle): &$struct_name) -> Option<&Obj> {
            self.pool.map
              .get(&handle.get_index())
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == handle.as_ptr())
              .map(|entry| entry.get())
          }

          pub fn get_mut(&mut self, $struct_name(handle): &$struct_name) -> Option<&mut Obj> {
            self.pool.map
              .get_mut(&handle.get_index())
              .filter(|entry| Weak::as_ptr(entry.get_rc()) == handle.as_ptr())
              .map(|entry| {
                entry.promote();
                entry.get_mut()
              })
          }

          pub fn remove(&mut self, $struct_name(handle): &$struct_name) {
            self.pool.map.remove(&handle.get_index());
          }

          pub fn collect_garbage(&mut self) {
            self.pool.collect_garbage()
          }

          pub fn iter_ids(&self) -> impl Iterator<Item = ($crate::deps::uuid::Uuid, &Obj)> {
            self.pool.iter()
          }

          pub fn as_weak_iter(&self) -> impl Iterator<Item = ($struct_name, &Obj)> {
            self.pool.map.iter().map(|(k, v)| {
              ($struct_name($crate::pool::Handle {
                // SAFETY: Indexing starts at 1
                // index: NonZeroUuid(unsafe { NonZeroU128::new_unchecked(k) }),
                // Goal: NonZeroUuid, have: Uuid
                index: $crate::pool::NonZeroUuid(unsafe { NonZeroU128::new_unchecked(k.as_u128()) }),
                rc: None,
              }), v.get())
            })
          }

          pub fn into_hashmap(&self) -> std::collections::HashMap<$crate::deps::uuid::Uuid, &Obj> {
            self.pool.map.iter().map(|(k, v)| {
              (*k, v.get())
            }).collect()
          }

          pub fn len(&self) -> usize {
            self.pool.len()
          }
        }

        impl [<Disassociated $pool_name:camel>] {
          pub fn reassociate<'a>(self, handles: impl IntoIterator<Item = &'a mut $struct_name>) -> $pool_name {
            let pool = self.pool.reassociate(handles.into_iter().map(|$struct_name(handle)| handle));

            $pool_name { pool }
          }
        }
      }
      pub use [<$struct_name:snake _impl>]::$struct_name;
      pub use [<$struct_name:snake _impl>]::$pool_name;
      pub use [<$struct_name:snake _impl>]::[<Disassociated $pool_name:camel>];
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
      std::mem::size_of::<usize>() + 16
    );

    assert_eq!(
      std::mem::size_of::<Option<FooHandle>>(),
      std::mem::size_of::<usize>() + 16
    );

    assert_eq!(
      std::mem::size_of::<FooPool>(),
      std::mem::size_of::<usize>() * 3
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
    - 00000000-0000-0000-0000-000000000001:
        type: Owned
        value:
          name: one
      00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
    - 00000000-0000-0000-0000-000000000001:
        type: Owned
        value:
          name: _one_
      00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
      00000000-0000-0000-0000-000000000003:
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
    - 00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
    - 00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
      00000000-0000-0000-0000-000000000003:
        type: Owned
        value:
          name: "3"
    "###);

    // Garbage collecting a layer removes values with no handles.
    layer1.collect_garbage();

    // However that only affects the one layer:
    assert_yaml_snapshot!((&layer1, &layer2), @r###"
    ---
    - 00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
    - 00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
      00000000-0000-0000-0000-000000000003:
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
    - 00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
    - 00000000-0000-0000-0000-000000000002:
        type: Owned
        value:
          name: TWO
      00000000-0000-0000-0000-000000000003:
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
    - {}
    - {}
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
    00000000-0000-0000-0000-000000000001:
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
    00000000-0000-0000-0000-000000000001:
      type: Shared
      value:
        name: one
    00000000-0000-0000-0000-000000000002:
      type: Virtual
      real_index: 00000000-0000-0000-0000-000000000001
    00000000-0000-0000-0000-000000000003:
      type: Virtual
      real_index: 00000000-0000-0000-0000-000000000001
    "###);

    let mut obj_two = pool1.get_mut(&handle_one_cow).unwrap();
    obj_two.name = "TWO".into();
    let mut obj_three = pool1.get_mut(&handle_one_cow_2).unwrap();
    obj_three.name = "tres".into();

    assert_yaml_snapshot!((&pool1), @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Shared
      value:
        name: one
    00000000-0000-0000-0000-000000000002:
      type: Owned
      value:
        name: TWO
    00000000-0000-0000-0000-000000000003:
      type: Owned
      value:
        name: tres
    "###);

    pool1.collect_garbage();
    pool1.collect_garbage();
    // Should convert shared values into owned values
    assert_yaml_snapshot!((&pool1), @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Shared
      value:
        name: one
    00000000-0000-0000-0000-000000000002:
      type: Owned
      value:
        name: TWO
    00000000-0000-0000-0000-000000000003:
      type: Owned
      value:
        name: tres
    "###);
  }

  fn serde_clone(value: &FooPool) -> DisassociatedFooPool {
    let serialized = serde_json::to_string(value).unwrap();
    let deserialized: DisassociatedFooPool = serde_json::from_str(&serialized).unwrap();

    deserialized
  }

  #[test]
  fn test_reassociation_simple() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let mut handle_one = pool.insert(make_obj("one"));

    assert_yaml_snapshot!(pool, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Owned
      value:
        name: one
    "###);

    let pool_clone = serde_clone(&pool).reassociate([]);
    assert_yaml_snapshot!(pool_clone, @r###"
    ---
    {}
    "###);

    let pool_clone = serde_clone(&pool).reassociate([&mut handle_one]);

    // After re
    assert_yaml_snapshot!(pool_clone, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Owned
      value:
        name: one
    "###);
  }

  #[test]
  fn test_reassociation_shared() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let mut handle_one = pool.insert(make_obj("one"));
    let mut handle_two = pool.reinsert(&handle_one).unwrap();

    let pool_clone = serde_clone(&pool).reassociate([&mut handle_one]);
    // Only the second handle was reassociated, and it became shared
    assert_yaml_snapshot!(pool_clone, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Shared
      value:
        name: one
    "###);

    let pool_clone = serde_clone(&pool).reassociate([&mut handle_two]);
    // Only the second handle was reassociated, and it became shared
    assert_yaml_snapshot!(pool_clone, @r###"
    ---
    00000000-0000-0000-0000-000000000002:
      type: Shared
      value:
        name: one
    "###);
  }

  #[test]
  fn get_mut_takes_ownership() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));
    let handle_two = pool.reinsert(&handle_one).unwrap();

    assert_yaml_snapshot!(pool, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Shared
      value:
        name: one
    00000000-0000-0000-0000-000000000002:
      type: Virtual
      real_index: 00000000-0000-0000-0000-000000000001
    "###);

    pool.get_mut(&handle_two);

    assert_yaml_snapshot!(pool, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Shared
      value:
        name: one
    00000000-0000-0000-0000-000000000002:
      type: Owned
      value:
        name: one
    "###);
  }

  #[test]
  fn weak_clone_allows_garbage_collection() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));
    let handle_one_weak = handle_one.weak_clone();

    pool.collect_garbage(); // No-op, handle_one still live
    assert_yaml_snapshot!(pool, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Owned
      value:
        name: one
    "###);

    std::mem::drop(handle_one);
    pool.collect_garbage(); // Drops entry, no strong handle alive
    assert_yaml_snapshot!(pool, @r###"
    ---
    {}
    "###);

    std::mem::drop(handle_one_weak);
  }

  #[test]
  fn remove_is_immutabable_op() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));

    let mut layer = pool.clone();
    layer.remove(&handle_one);

    assert_yaml_snapshot!((&pool, &layer), @r###"
    ---
    - 00000000-0000-0000-0000-000000000001:
        type: Owned
        value:
          name: one
    - {}
    "###);

    std::mem::drop(handle_one);
  }

  #[test]
  fn remove_does_not_break_virtual_references() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));
    let handle_two = pool.reinsert(&handle_one).unwrap();

    let mut layer = pool.clone();
    layer.remove(&handle_one);

    assert_yaml_snapshot!((&pool, &layer), @r###"
    ---
    - 00000000-0000-0000-0000-000000000001:
        type: Shared
        value:
          name: one
      00000000-0000-0000-0000-000000000002:
        type: Virtual
        real_index: 00000000-0000-0000-0000-000000000001
    - 00000000-0000-0000-0000-000000000002:
        type: Shared
        value:
          name: one
    "###);

    // The value is lazily still "shared" until we try to mutate it, keeping
    // reinserts cheap.
    let mut value = layer.get_mut(&handle_two).unwrap();
    value.name = "dos".into();
    layer.collect_garbage();
    assert_yaml_snapshot!(layer, @r###"
    ---
    00000000-0000-0000-0000-000000000002:
      type: Owned
      value:
        name: dos
    "###);

    std::mem::drop(handle_one);
    std::mem::drop(handle_two);
  }

  #[test]
  fn can_iter_over_uuids_and_values() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));
    pool.reinsert(&handle_one).unwrap();

    let collected: Vec<_> = pool.iter_ids().collect();
    assert_yaml_snapshot!(collected, @r###"
    ---
    - - 00000000-0000-0000-0000-000000000001
      - name: one
    - - 00000000-0000-0000-0000-000000000002
      - name: one
    "###);
  }

  #[test]
  fn can_iter_over_weak_handles() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));
    pool.reinsert(&handle_one).unwrap();

    let collected: Vec<_> = pool.as_weak_iter().collect();
    // Because the rc is skipped, the serialization format is the same as iter_ids()
    assert_yaml_snapshot!(collected, @r###"
    ---
    - - 00000000-0000-0000-0000-000000000001
      - name: one
    - - 00000000-0000-0000-0000-000000000002
      - name: one
    "###);

    std::mem::drop(handle_one);

    assert_yaml_snapshot!(pool, @r###"
    ---
    {}
    "###);

    // Drop the weak handles
    std::mem::drop(collected);
  }

  #[test]
  fn can_into_hashmap() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert(make_obj("one"));
    pool.reinsert(&handle_one).unwrap();

    let std_hashmap = pool.into_hashmap();

    let mut items = std_hashmap.iter().collect::<Vec<_>>();
    items.sort();
    // The order would be nondeterministic, so render the sorted items:
    assert_yaml_snapshot!(items, @r###"
    ---
    - - 00000000-0000-0000-0000-000000000001
      - name: one
    - - 00000000-0000-0000-0000-000000000002
      - name: one
    "###);
  }
}
