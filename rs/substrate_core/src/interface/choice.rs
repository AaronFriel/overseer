use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::game::{ObjectHandle, PlayerHandle};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(tag = "kind")]
#[serde_diff(opaque)]
pub enum ChoiceOption {
  Player(PlayerHandle),
  Object(ObjectHandle),
  Yes,
  No,
  Custom(String),
}

impl From<PlayerHandle> for ChoiceOption {
  fn from(v: PlayerHandle) -> Self {
    Self::Player(v)
  }
}

impl ChoiceOption {
  /// Returns `true` if the choice_option is [`Yes`].
  pub fn is_yes(&self) -> bool {
    matches!(self, Self::Yes)
  }

  /// Returns `true` if the choice_option is [`No`].
  pub fn is_no(&self) -> bool {
    matches!(self, Self::No)
  }

  /// Returns `true` if the choice_option is [`Player`].
  pub fn is_player(&self) -> bool {
    matches!(self, Self::Player(..))
  }

  /// Returns `true` if the choice_option is [`Object`].
  pub fn is_object(&self) -> bool {
    matches!(self, Self::Object(..))
  }

  pub fn as_custom(&self) -> Option<&String> {
    if let Self::Custom(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_player(&self) -> Option<&PlayerHandle> {
    if let Self::Player(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_object(&self) -> Option<&ObjectHandle> {
    if let Self::Object(v) = self {
      Some(v)
    } else {
      None
    }
  }
}
