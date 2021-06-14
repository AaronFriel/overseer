use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use dyn_clonable::clonable;
use dyn_partial_eq::dyn_partial_eq;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{
  game::{Game, PlayerHandle, Visibility},
  interface::ChoiceOption,
};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum ActionResult {
  Step,
  Prompt(ChoicePrompt),
  Resolved,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct ChoicePrompt {
  pub player_handle: Option<PlayerHandle>,
  pub kind: PromptKind,
  pub prompt: String,
  pub choices: Vec<ChoiceOption>,
  pub visibility: Visibility,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum PromptKind {
  Select,
  MultiSelect,
  Shuffle,
  Sort,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum PromptResult {
  None,
  Selected(ChoiceOption),
  MultiSelected(Vec<ChoiceOption>),
  Shuffled(Vec<ChoiceOption>),
  Sorted(Vec<ChoiceOption>),
}

#[typetag::serde(tag = "kind")]
#[dyn_partial_eq]
#[clonable]
pub trait Action: core::fmt::Debug + ActionHash + Clone {
  fn apply(&mut self, game: &mut Game, choice: PromptResult) -> ActionResult;
}

pub trait ActionHash {
  fn hash_value(&self) -> u64;
}

impl std::hash::Hash for Box<dyn Action> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.typetag_name().hash(state);
    self.hash_value().hash(state);
  }
}

impl<T> From<T> for Box<dyn Action>
where
  T: Action + 'static,
{
  #[inline(always)]
  fn from(x: T) -> Self {
    Box::new(x)
  }
}

impl<T> ActionHash for T
where
  T: std::hash::Hash,
{
  fn hash_value(&self) -> u64 {
    let mut x = DefaultHasher::new();
    self.hash(&mut x);
    x.finish()
  }
}
