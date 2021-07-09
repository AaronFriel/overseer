use std::borrow::Cow;

use insta::assert_yaml_snapshot;
use overseer_util::make_refcounted_pool;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff)]
pub struct FooObject {
  #[serde_diff(opaque)]
  name: Cow<'static, str>,
}

make_refcounted_pool!(FooObject, FooPool, FooHandle, u32);

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
    map:
      1:
        name: one
      2:
        name: TWO
  - next_index: 4
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
    map:
      1:
        name: one
      2:
        name: TWO
  - next_index: 4
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
    map:
      2:
        name: TWO
  - next_index: 4
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
    map:
      2:
        name: TWO
  - next_index: 4
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
    map: {}
  - next_index: 4
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
