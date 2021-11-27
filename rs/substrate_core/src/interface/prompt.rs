use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use super::Target;
use crate::{
  action::PromptKind,
  game::{PlayerHandle, Visibility},
};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Prompt {
  pub player: Option<PlayerHandle>,
  pub kind: PromptKind,
  pub prompt: String,
  pub choices: Vec<Target>,
  pub visibility: Visibility,
}
