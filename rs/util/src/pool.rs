#[macro_export]
macro_rules! make_refcounted_pool {
  ($object_name:ident, $pool_name:ident, $struct_name:ident, $size:ty) => {
    paste::paste! {
      mod [<$struct_name:snake _impl>] {
        use std::{
          collections::HashMap as StdHashMap,
          ptr,
          rc::{Rc, Weak},
        };

        use im::HashMap;
        use nonzero_ext::NonZeroAble;
        use serde::{Deserialize, Serialize};
        use serde_diff::SerdeDiff;

        use super::$object_name as Obj;
        use $crate::pool::{RefCounted, WeakCounted};

        #[cfg(test)]
        use std::{collections::hash_map::DefaultHasher, hash::BuildHasherDefault};
        #[cfg(test)]
        type Hasher = BuildHasherDefault<DefaultHasher>;

        // Macro inputs:
        pub type Zeroable = $size;
        pub type NonZero = <$size as NonZeroAble>::NonZero;

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[derive(Serialize, Deserialize, SerdeDiff)]
        #[serde(transparent)]
        pub struct $struct_name(RefCounted<NonZero>);

        impl $struct_name {
          #[inline]
          pub(crate) fn weak_clone(&self) -> Self {
            $struct_name(RefCounted{
              value: self.0.value,
              rc: Rc::default(),
            })
          }
        }

        #[derive(Clone, Hash, Debug)]
        #[derive(Serialize, Deserialize, SerdeDiff)]
        pub struct $pool_name {
          next_index: NonZero,
          #[cfg(test)]
          map: HashMap<NonZero, WeakCounted<Obj>, Hasher>,
          #[cfg(not(test))]
          map: HashMap<NonZero, WeakCounted<Obj>>,
        }

        impl $pool_name {
          pub fn new() -> Self {
            Self {
              next_index: unsafe { (1 as Zeroable).into_nonzero_unchecked() },
              #[cfg(test)]
              map: HashMap::with_hasher(Hasher::default()),
              #[cfg(not(test))]
              map: HashMap::new(),
            }
          }

          pub fn insert(&mut self, object: Obj) -> $struct_name {
            let index = self.next_index;
            let next_index = unsafe { (index.get() + 1).into_nonzero_unchecked() };

            let handle = RefCounted {
              value: index,
              rc: Rc::default(),
            };
            let inserted = WeakCounted {
              value: object,
              rc: Rc::downgrade(&handle.rc),
            };

            self.map.insert(index, inserted);
            self.next_index = next_index;

            $struct_name(handle)
          }

          pub fn get(&self, handle: &$struct_name) -> Option<&Obj> {
            self.map
              .get(&handle.0.value)
              .and_then(|f|
                  if ptr::eq(&*handle.0.rc, f.rc.as_ptr()) {
                    Some(&f.value)
                  } else {
                    None
                  }
              )
          }

          pub fn get_mut(&mut self, handle: &$struct_name) -> Option<&mut Obj> {
            self.map.get_mut(&handle.0.value).map(|f| &mut f.value)
          }

          pub fn remove(&mut self, handle: &$struct_name) {
            self.map.remove(&handle.0.value);
          }

          pub fn reassociate(&self, handle: &mut $struct_name) -> bool {
            if let Some(inserted) = self.map.get(&handle.0.value) {
              if let Some(rc) = inserted.rc.upgrade() {
                handle.0.rc = rc;
                return true;
              }
            };

            false
          }

          pub fn collect_garbage(&mut self) {
            let removable_keys: Vec<NonZero> = self.map.iter().filter_map(|(k, v)| {
              if Weak::strong_count(&v.rc) == 0 { Some(*k) } else { None }
            }).collect();

            removable_keys.iter().for_each(|k| {
              if let Some(v) = self.map.remove(&k) {
                std::mem::drop(v);
              }
            });
          }

          pub fn iter(&self) -> impl Iterator<Item = (Zeroable, &Obj)> {
            self.map.iter().map(|(k, v)| {
              (k.get(), &v.value)
            })
          }

          pub fn iter_weak(&self) -> impl Iterator<Item = ($struct_name, &Obj)> {
            self.map.iter().map(|(k, v)| {
              ($struct_name(RefCounted { value: *k, rc: Rc::default() }), &v.value)
            })
          }

          pub fn into_hashmap(&self) -> StdHashMap<Zeroable, &Obj> {
            self.map.iter().map(|(k, v)| {
              (k.get(), &v.value)
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

mod ref_counted {
  use std::{
    fmt::Debug,
    rc::{Rc, Weak},
  };

  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use serde_diff::SerdeDiff;

  macro_rules! make_refcounted {
    ($object_name:ident, $rc_type:ident) => {
      #[derive(Clone, Debug)]
      pub struct $object_name<T> {
        pub value: T,
        pub rc: $rc_type<()>,
      }

      impl<T> PartialEq for $object_name<T>
      where
        T: PartialEq,
      {
        fn eq(&self, other: &Self) -> bool {
          self.value.eq(&other.value)
        }
      }

      impl<T> Eq for $object_name<T>
      where
        T: Eq,
      {
        fn assert_receiver_is_total_eq(&self) {}
      }

      impl<T> PartialOrd for $object_name<T>
      where
        T: PartialOrd,
      {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
          self.value.partial_cmp(&other.value)
        }
      }

      impl<T> Ord for $object_name<T>
      where
        T: Ord,
      {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
          self.value.cmp(&other.value)
        }
      }

      impl<T> std::hash::Hash for $object_name<T>
      where
        T: std::hash::Hash,
      {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
          self.value.hash(state)
        }
      }

      impl<T> Serialize for $object_name<T>
      where
        T: Serialize,
      {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
          S: Serializer,
        {
          self.value.serialize(serializer)
        }
      }
      //#endregion

      impl<'de, T> Deserialize<'de> for $object_name<T>
      where
        T: Deserialize<'de>,
      {
        fn deserialize<D>(deserializer: D) -> Result<$object_name<T>, D::Error>
        where
          D: Deserializer<'de>,
        {
          let value = T::deserialize(deserializer)?;
          Ok($object_name {
            value,
            rc: $rc_type::default(),
          })
        }
      }

      impl<T> SerdeDiff for $object_name<T>
      where
        T: PartialEq + SerdeDiff + Serialize,
        for<'de> T: Deserialize<'de>,
      {
        fn diff<'a, S: serde::ser::SerializeSeq>(
          &self,
          ctx: &mut serde_diff::DiffContext<'a, S>,
          other: &Self,
        ) -> Result<bool, S::Error> {
          if self.value != other.value {
            ctx.save_value(other)?;
            Ok(true)
          } else {
            Ok(false)
          }
        }

        fn apply<'de, A>(
          &mut self,
          seq: &mut A,
          ctx: &mut serde_diff::ApplyContext,
        ) -> Result<bool, <A as serde::de::SeqAccess<'de>>::Error>
        where
          A: serde::de::SeqAccess<'de>,
        {
          ctx.read_value(seq, self)
        }
      }
    };
  }

  make_refcounted!(RefCounted, Rc);
  make_refcounted!(WeakCounted, Weak);
}

pub use ref_counted::{RefCounted, WeakCounted};
