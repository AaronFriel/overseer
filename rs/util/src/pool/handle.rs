use std::{
  fmt::Debug,
  hash::Hash,
  num::NonZeroU128,
  rc::{Rc, Weak},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::NonZeroUuid;

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
