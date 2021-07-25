use std::borrow::Cow;

use insta::assert_yaml_snapshot;
use overseer_util::make_refcounted_pool;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff)]
pub struct FooObject {
  #[serde_diff(opaque)]
  name: Cow<'static, str>,
}

// make_refcounted_pool!(FooObject, FooPool, FooHandle);
// Recursive expansion of make_refcounted_pool! macro
// ===================================================

mod foo_handle_impl {
  use std::rc::{Rc, Weak};
  #[cfg(test)]
  use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};

  use im::HashMap;
  use overseer_util::pool::{Entry, Handle};
  use serde::{Deserialize, Serialize};
  use Entry::*;

  use super::FooObject as Obj;
  #[cfg(test)]
  type Hasher = BuildHasherDefault<DefaultHasher>;
  #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
  #[derive(Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct FooHandle(Handle);
  #[derive(Clone, Debug)]
  #[derive(Serialize, Deserialize)]
  pub struct FooPool {
    next_index: usize,
    #[cfg(test)]
    map: HashMap<usize, Entry<Obj>, Hasher>,
    #[cfg(not(test))]
    map: HashMap<usize, Entry<Obj>>,
  }
  impl FooPool {
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

    pub fn insert(&mut self, object: Obj) -> FooHandle {
      let index = self.next_index();
      let handle = Handle::new(index);
      self.map.insert(index, Real {
        value: Rc::new(object),
        rc: Rc::downgrade(&handle.rc),
      });
      FooHandle(handle)
    }

    pub fn reinsert(&mut self, FooHandle(handle): &FooHandle) -> Option<FooHandle> {
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
            Some(FooHandle(handle))
          }
          Real { value, .. } => {
            let handle = Handle::new(new_handle_index);
            let value = Virtual {
              value: value.clone(),
              rc: Rc::downgrade(&handle.rc),
              real_index: index,
            };
            self.map.insert(new_handle_index, value);
            Some(FooHandle(handle))
          }
        }
      } else {
        None
      }
    }

    pub fn get(&self, FooHandle(handle): &FooHandle) -> Option<&Obj> {
      self
        .map
        .get(&handle.index)
        .filter(|entry| Weak::as_ptr(entry.get_rc()) == Rc::as_ptr(&handle.rc))
        .map(|entry| entry.get())
    }

    pub fn get_mut(&mut self, FooHandle(handle): &FooHandle) -> Option<&mut Obj> {
      self
        .map
        .get_mut(&handle.index)
        .filter(|entry| Weak::as_ptr(entry.get_rc()) == Rc::as_ptr(&handle.rc))
        .map(|entry| {
          entry.promote();
          entry.get_mut()
        })
    }

    pub fn remove(&mut self, FooHandle(handle): &FooHandle) {
      self.map.remove(&handle.index);
    }

    pub fn reassociate(&mut self, FooHandle(handle): &mut FooHandle) -> bool {
      if let Some(inserted) = self.map.get(&handle.index) {
        if let Some(rc) = Weak::upgrade(inserted.get_rc()) {
          handle.rc = rc;
          return true;
        }
      };

      if let Some(inserted) = self.map.get_mut(&handle.index) {
        let mut new_rc = Rc::downgrade(&mut handle.rc);
        match inserted {
          Virtual { rc, .. } => std::mem::swap(rc, &mut new_rc),
          Real { rc, .. } => std::mem::swap(rc, &mut new_rc),
        };
        return true;
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
      self.map.iter().map(|(k, v)| (*k, v.get()))
    }

    pub fn into_hashmap(&self) -> std::collections::HashMap<usize, &Obj> {
      self.map.iter().map(|(k, v)| (*k, v.get())).collect()
    }

    pub fn len(&self) -> usize {
      self.map.keys().len()
    }
  }
}
pub use foo_handle_impl::{FooHandle, FooPool};

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
  - next_index: 3
    virtual_map: {}
    map:
      1:
        name: one
      2:
        name: TWO
  - next_index: 4
    virtual_map: {}
    map:
      1:
        name: _one_
      2:
        name: TWO
      3:
        name: "3"
  "###);

  // Dropping the handle allows garbage collection to proceed in any layer:
  // a reference counter verifies no outstanding handles exist.
  std::mem::drop(handle_one);

  // Drops don't do anything on their own, though, nothing has changed yet:
  assert_eq!((layer1.len(), layer2.len()), (2, 3));
  assert_yaml_snapshot!((&layer1, &layer2), @r###"
  ---
  - next_index: 3
    virtual_map: {}
    map:
      1:
        name: one
      2:
        name: TWO
  - next_index: 4
    virtual_map: {}
    map:
      1:
        name: _one_
      2:
        name: TWO
      3:
        name: "3"
  "###);

  // Garbage collecting a layer removes values with no handles.
  layer1.collect_garbage();

  // However that only affects the one layer:
  assert_yaml_snapshot!((&layer1, &layer2), @r###"
  ---
  - next_index: 3
    virtual_map: {}
    map:
      2:
        name: TWO
  - next_index: 4
    virtual_map: {}
    map:
      1:
        name: _one_
      2:
        name: TWO
      3:
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
  - next_index: 3
    virtual_map: {}
    map:
      2:
        name: TWO
  - next_index: 4
    virtual_map: {}
    map:
      2:
        name: TWO
      3:
        name: "3"
  "###);

  // Finally, we drop all the remaining handles and clean up:
  std::mem::drop(handle_three);
  std::mem::drop(handle_two_clone);
  layer1.collect_garbage();
  layer2.collect_garbage();

  assert_yaml_snapshot!((&layer1, &layer2), @r###"
  ---
  - next_index: 3
    virtual_map: {}
    map: {}
  - next_index: 4
    virtual_map: {}
    map: {}
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
fn test_cow_mapping() {
  let make_obj = |s: &'static str| FooObject {
    name: Cow::Borrowed(s),
  };
  let mut pool1 = FooPool::new();
  let handle_one = pool1.insert(make_obj("one"));
  let handle_one_cow = pool1.reinsert(&handle_one).unwrap();

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
  virtual_map:
    2: 1
  map:
    1:
      name: one
  "###);

  let mut obj_two = pool1.get_mut(&handle_two_cow).unwrap();
  obj_two.name = "Two".into();

  assert_yaml_snapshot!((&pool1), @r###"
  ---
  next_index: 3
  virtual_map:
    2: 1
  map:
    1:
      name: one
  "###);
}
