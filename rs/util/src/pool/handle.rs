use std::{
  cell::UnsafeCell,
  fmt::Debug,
  hash::Hash,
  num::NonZeroU128,
  rc::{Rc, Weak},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::NonZeroUuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Handle {
  pub index: NonZeroUuid,
  #[serde(skip)]
  pub rc: UnsafeCell<Option<Rc<()>>>,
}

impl Ord for Handle {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.index.cmp(&other.index)
  }
}

impl Eq for Handle {}

impl PartialOrd for Handle {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.index.partial_cmp(&other.index)
  }
}

impl PartialEq for Handle {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index
  }
}

impl Clone for Handle {
  fn clone(&self) -> Self {
    Self {
      index: self.index.clone(),
      rc: UnsafeCell::new(unsafe { &*self.rc.get() }.clone()),
    }
  }
}

impl Handle {
  pub fn new(index: NonZeroU128) -> Self {
    Self {
      index: NonZeroUuid(index),
      rc: UnsafeCell::new(Some(Rc::default())),
    }
  }

  pub fn is_weak(&self) -> bool {
    unsafe { &*self.rc.get() }.is_none()
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
        rc: UnsafeCell::new(Some(rc)),
      },
      weak,
    )
  }

  pub fn weak_clone(&self) -> Self {
    Self {
      index: self.index,
      rc: UnsafeCell::new(None),
    }
  }

  pub fn as_ptr(&self) -> *const () {
    match unsafe { &*self.rc.get() } {
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
