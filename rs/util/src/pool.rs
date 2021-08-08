mod entry;
mod handle;
mod nonzerouuid;
mod pool;

#[doc(hidden)]
pub use self::{entry::*, handle::*, nonzerouuid::*, pool::*};

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

          pub fn is_weak(&self) -> bool {
            self.0.is_weak()
          }

          pub fn as_uuid(&self) -> $crate::deps::uuid::Uuid {
            self.0.get_index()
          }
        }

        #[derive(Clone, Debug, Hash)]
        #[derive(Serialize, Deserialize)]
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


          fn next_index_monotonic(&mut self) -> NonZeroU128 {
            let mut index = (self.len() + 1) as u128;

            while (self.pool.map.contains_key(&$crate::deps::uuid::Uuid::from_u128(index))) {
              index += 1;
            }

            unsafe { NonZeroU128::new_unchecked((self.len() + 1) as u128) }
          }

          fn next_index(&mut self) -> NonZeroU128 {
            let index = $crate::deps::uuid::Uuid::new_v4();
            unsafe { NonZeroU128::new_unchecked(index.as_u128()) }
          }

          pub fn insert(&mut self, object: Obj) -> $struct_name {
            let index = self.next_index();
            self.insert_with_index(object, index)
          }

          pub fn insert_monotonic(&mut self, object: Obj) -> $struct_name {
            let index = self.next_index_monotonic();
            self.insert_with_index(object, index)
          }

          fn insert_with_index(&mut self, object: Obj, index: NonZeroU128) -> $struct_name {
            let (handle, weak) = $crate::pool::Handle::new_pair(index);
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

          pub fn reinsert(&mut self, handle: &$struct_name) -> Option<$struct_name> {
            let new_index = self.next_index();
            self.reinsert_with_index(handle, new_index)
          }

          pub fn reinsert_monotonic(&mut self, handle: &$struct_name) -> Option<$struct_name> {
            let new_index = self.next_index_monotonic();
            self.reinsert_with_index(handle, new_index)
          }

          fn reinsert_with_index(&mut self, $struct_name(original_handle): &$struct_name, new_index: NonZeroU128) -> Option<$struct_name> {
            let original_index = original_handle.get_index();
            let (new_handle, rc) = $crate::pool::Handle::new_pair(new_index);
            let new_index = new_handle.get_index();
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
              self.pool.map.insert(new_index, inserted);
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

          pub fn reassociate<'a>(&mut self, handles: impl IntoIterator<Item = &'a mut $struct_name>) {
            let pool = self.pool.clone();
            let pool = pool.to_disassociated_pool();
            let pool = pool.reassociate(handles.into_iter().map(|$struct_name(handle)| handle));
            self.pool = pool;
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
    let handle_one = layer1.insert_monotonic(obj_one);

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
    let handle_one = layer1.insert_monotonic(obj_one);

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
    let handle_one = layer1.insert_monotonic(make_obj("one")); // We will drop this handle first.
    let handle_two = layer1.insert_monotonic(make_obj("TWO")); // We will drop this second.

    let mut layer2 = layer1.clone();
    layer2.get_mut(&handle_one).unwrap().name = Cow::Borrowed("_one_");
    let handle_three = layer2.insert_monotonic(make_obj("3")); // We'll hold on to this until the end
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
    let handle_one = pool1.insert_monotonic(make_obj("one"));

    let mut layer1 = FooPool::new();
    let handle_two = layer1.insert_monotonic(make_obj("TWO"));

    // Handles must be from the pool they were created from.
    assert_eq!(pool1.get(&handle_two), None);
    assert_eq!(layer1.get(&handle_one), None);

    let mut layer2 = layer1.clone();

    let layer1_handle = layer1.insert_monotonic(make_obj("3"));
    let layer2_handle = layer2.insert_monotonic(make_obj("3"));

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
    let handle_one = pool1.insert_monotonic(make_obj("one"));
    assert_yaml_snapshot!((&pool1), @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Owned
      value:
        name: one
    "###);

    let handle_one_cow = pool1.reinsert_monotonic(&handle_one).unwrap();
    let handle_one_cow_2 = pool1.reinsert_monotonic(&handle_one_cow).unwrap();

    let mut layer1 = FooPool::new();
    let handle_two = layer1.insert_monotonic(make_obj("TWO"));
    let handle_two_cow = layer1.reinsert_monotonic(&handle_two).unwrap();

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
    let deserialized = serde_json::from_str(&serialized).unwrap();

    deserialized
  }

  #[test]
  fn test_reassociation_simple() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let mut handle_one = pool.insert_monotonic(make_obj("one"));

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
    let mut handle_one = pool.insert_monotonic(make_obj("one"));
    let mut handle_two = pool.reinsert_monotonic(&handle_one).unwrap();

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
    let handle_one = pool.insert_monotonic(make_obj("one"));
    let handle_two = pool.reinsert_monotonic(&handle_one).unwrap();

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
    let handle_one = pool.insert_monotonic(make_obj("one"));
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
    let handle_one = pool.insert_monotonic(make_obj("one"));

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
    let handle_one = pool.insert_monotonic(make_obj("one"));
    let handle_two = pool.reinsert_monotonic(&handle_one).unwrap();

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
    let handle_one = pool.insert_monotonic(make_obj("one"));
    pool.reinsert_monotonic(&handle_one).unwrap();

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
    let handle_one = pool.insert_monotonic(make_obj("one"));
    pool.reinsert_monotonic(&handle_one).unwrap();

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
    let handle_one = pool.insert_monotonic(make_obj("one"));
    pool.reinsert_monotonic(&handle_one).unwrap();

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

  #[test]
  fn manual_reassociate() {
    let make_obj = |s: &'static str| FooObject {
      name: Cow::Borrowed(s),
    };
    let mut pool = FooPool::new();
    let handle_one = pool.insert_monotonic(make_obj("one"));
    let mut handle_one_weak = handle_one.weak_clone();

    std::mem::drop(handle_one); // Entry is now disassociated, but we can recover

    // Proof of diassociation, a clone that's been garbage collected will be empty:
    {
      let mut cloned = pool.clone();
      cloned.collect_garbage();
      assert_yaml_snapshot!(cloned, @r###"
      ---
      {}
      "###);
    }

    assert_eq!(handle_one_weak.is_weak(), true);
    pool.reassociate([&mut handle_one_weak]);
    assert_eq!(handle_one_weak.is_weak(), false);

    // Proof of reassociation, same operation but the weak handle is now strong:
    {
      let mut cloned = pool.clone();
      cloned.collect_garbage();
      assert_yaml_snapshot!(cloned, @r###"
      ---
      00000000-0000-0000-0000-000000000001:
        type: Owned
        value:
          name: one
      "###);
    }
    pool.collect_garbage();
    assert_yaml_snapshot!(pool, @r###"
    ---
    00000000-0000-0000-0000-000000000001:
      type: Owned
      value:
        name: one
    "###);
  }

  #[test]
  fn insert_non_monotonic() {
    let make_obj = |s: String| FooObject {
      name: Cow::Owned(s),
    };
    let mut pool = FooPool::new();

    let handle_one = pool.insert(make_obj("1".to_string()));
    let generated_handle_less_than_first = (2..=256)
      .into_iter()
      .map(|i| pool.insert(make_obj(i.to_string())))
      .any(|handle| handle < handle_one);

    assert!(generated_handle_less_than_first);

    // This test is technically nondeterministic, but the odds of not generating
    // a UUID less than the initial are improbable.
  }

  #[test]
  fn reinsert_non_monotonic() {
    let make_obj = |s: String| FooObject {
      name: Cow::Owned(s),
    };
    let mut pool = FooPool::new();

    let handle_one = pool.insert(make_obj("1".to_string()));
    let generated_handle_less_than_first = (2..=256)
      .into_iter()
      .map(|_| pool.reinsert(&handle_one).unwrap())
      .any(|handle| handle < handle_one);

    assert!(generated_handle_less_than_first);

    // This test is technically nondeterministic, but the odds of not generating
    // a UUID less than the initial are improbable.
  }

  #[test]
  fn insert_monotonic() {
    let make_obj = |s: String| FooObject {
      name: Cow::Owned(s),
    };
    let mut pool = FooPool::new();

    for i in 1..=256 {
      let handle = pool.insert_monotonic(make_obj(i.to_string()));
      assert_eq!(handle.as_uuid().as_u128(), i);
    }
  }
}
