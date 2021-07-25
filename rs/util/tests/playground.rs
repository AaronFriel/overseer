use std::{
  collections::hash_map::DefaultHasher,
  hash::BuildHasherDefault,
  rc::{Rc, Weak},
  sync::atomic::{AtomicUsize, Ordering},
};

use crossbeam::atomic::AtomicCell;
use im::HashMap;
use insta::assert_yaml_snapshot;
use serde::{Deserialize, Serialize};

type Hasher = BuildHasherDefault<DefaultHasher>;

#[test]
fn tests() {
  #[derive(Debug, Clone, Serialize, Deserialize)]
  struct Obj {
    a: usize,
    b: usize,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  struct Handle {
    index: usize,
    #[serde(skip)]
    rc: Rc<()>,
  }

  impl Handle {
    fn new(index: usize) -> Self {
      Self {
        index,
        rc: Rc::default(),
      }
    }
  }

  struct OptShared<T> {
    Owned(T),
    Shared(Rc<T>),
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
    fn promote(&mut self) {
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

  impl<T> Entry<T>
  where
    T: Clone,
  {
    fn get_rc(&self) -> &Weak<()> {
      match self {
        Virtual { rc, .. } | Real { rc, .. } => &rc,
      }
    }

    fn get_value(&self) -> &Rc<T> {
      match self {
        Virtual { value, .. } | Real { value, .. } => &value,
      }
    }

    fn get_value_mut(&mut self) -> &mut T {
      match self {
        Virtual { ref mut value, .. } | Real { ref mut value, .. } => Rc::make_mut(value),
      }
    }

    fn ptr_eq(&self, other: &Entry<T>) -> bool {
      Rc::ptr_eq(self.get_value(), other.get_value())
    }
  }
  use Entry::*;

  let test_a: AtomicCell<usize> = AtomicCell::new(0);

  // let index = AtomicUsize::new(0);
  let make = || Obj {
    a: (&test_a).fetch_add(1),
    b: (&test_a).fetch_add(1),
  };

  type Map = HashMap<usize, Entry<Obj>, Hasher>;
  let counter = AtomicUsize::new(0);
  let mut map: Map = HashMap::with_hasher(Hasher::default());

  fn insert(counter: &AtomicUsize, map: &mut Map, obj: Obj) -> Handle {
    let index = counter.fetch_add(1, Ordering::AcqRel);

    let handle = Handle::new(index);
    map.insert(index, Real {
      value: Rc::new(obj),
      rc: Rc::downgrade(&handle.rc),
    });
    handle
  }
  fn re_insert_ll(counter: &AtomicUsize, map: &mut Map, index: usize) -> Handle {
    let current_entry = map.get(&index).unwrap();
    let new_handle_index = counter.fetch_add(1, Ordering::AcqRel);
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
        map.insert(new_handle_index, value);
        handle
      }
      Real { value, .. } => {
        let handle = Handle::new(new_handle_index);
        let value = Virtual {
          value: value.clone(),
          rc: Rc::downgrade(&handle.rc),
          real_index: index,
        };
        map.insert(new_handle_index, value);
        handle
      }
    }
  }

  fn get_mut_ll<'a>(map: &'a mut Map, original_handle: &Handle) -> Option<&'a mut Obj> {
    let entry = map.get_mut(&original_handle.index);
    entry.map(|e| {
      e.promote();
      e.get_value_mut()
    })
  }

  fn collect_garbage(map: &mut Map) {
    for (k, entry) in map.clone().iter() {
      if Weak::strong_count(entry.get_rc()) == 0 {
        map.remove(k);
      }
    }
    let key_ref = map.clone();
    let mut deduplicated_keys = std::collections::HashMap::<usize, usize>::new();
    for (k, entry) in map.iter_mut() {
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

  let a_handle = insert(&counter, &mut map, make());
  let b_handle = insert(&counter, &mut map, make());
  let c_handle = insert(&counter, &mut map, make());
  insert(&counter, &mut map, make());

  let x_handle = re_insert_ll(&counter, &mut map, a_handle.index);
  let y_handle = re_insert_ll(&counter, &mut map, b_handle.index);
  let z_handle = re_insert_ll(&counter, &mut map, c_handle.index);
  {
    let z = get_mut_ll(&mut map, &z_handle).unwrap();
    z.a = 100;
  }

  assert_yaml_snapshot!(map, @r###"
  ---
  0:
    type: Real
    value:
      a: 0
      b: 1
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Virtual
    real_index: 0
    value:
      a: 0
      b: 1
  2:
    type: Real
    value:
      a: 4
      b: 5
  1:
    type: Real
    value:
      a: 2
      b: 3
  3:
    type: Real
    value:
      a: 6
      b: 7
  5:
    type: Virtual
    real_index: 1
    value:
      a: 2
      b: 3
  "###);

  let mut map2 = map.clone();
  {
    let a = get_mut_ll(&mut map2, &a_handle).unwrap();
    a.a = 1000;
  }

  assert_yaml_snapshot!(map, @r###"
  ---
  0:
    type: Real
    value:
      a: 0
      b: 1
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Virtual
    real_index: 0
    value:
      a: 0
      b: 1
  2:
    type: Real
    value:
      a: 4
      b: 5
  1:
    type: Real
    value:
      a: 2
      b: 3
  3:
    type: Real
    value:
      a: 6
      b: 7
  5:
    type: Virtual
    real_index: 1
    value:
      a: 2
      b: 3
  "###);

  assert_yaml_snapshot!(map2, @r###"
  ---
  0:
    type: Real
    value:
      a: 1000
      b: 1
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Virtual
    real_index: 0
    value:
      a: 0
      b: 1
  2:
    type: Real
    value:
      a: 4
      b: 5
  1:
    type: Real
    value:
      a: 2
      b: 3
  3:
    type: Real
    value:
      a: 6
      b: 7
  5:
    type: Virtual
    real_index: 1
    value:
      a: 2
      b: 3
  "###);

  collect_garbage(&mut map2);
  assert_yaml_snapshot!(map2, @r###"
  ---
  0:
    type: Real
    value:
      a: 1000
      b: 1
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Real
    value:
      a: 0
      b: 1
  2:
    type: Real
    value:
      a: 4
      b: 5
  1:
    type: Real
    value:
      a: 2
      b: 3
  5:
    type: Virtual
    real_index: 1
    value:
      a: 2
      b: 3
  "###);

  std::mem::drop(c_handle);
  collect_garbage(&mut map2);

  assert_yaml_snapshot!(map2, @r###"
  ---
  0:
    type: Real
    value:
      a: 1000
      b: 1
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Real
    value:
      a: 0
      b: 1
  1:
    type: Real
    value:
      a: 2
      b: 3
  5:
    type: Virtual
    real_index: 1
    value:
      a: 2
      b: 3
  "###);

  std::mem::drop(b_handle);
  collect_garbage(&mut map2);
  assert_yaml_snapshot!(map2, @r###"
  ---
  0:
    type: Real
    value:
      a: 1000
      b: 1
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Real
    value:
      a: 0
      b: 1
  5:
    type: Real
    value:
      a: 2
      b: 3
  "###);
  std::mem::drop(a_handle);
  collect_garbage(&mut map2);
  assert_yaml_snapshot!(map2, @r###"
  ---
  6:
    type: Real
    value:
      a: 100
      b: 5
  4:
    type: Real
    value:
      a: 0
      b: 1
  5:
    type: Real
    value:
      a: 2
      b: 3
  "###);

  std::mem::drop(x_handle);
  std::mem::drop(y_handle);
  std::mem::drop(z_handle);
}
