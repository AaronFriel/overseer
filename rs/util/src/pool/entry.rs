use std::{
  fmt::Debug,
  hash::Hash,
  hint::unreachable_unchecked,
  rc::{Rc, Weak},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
use Entry::*;

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
