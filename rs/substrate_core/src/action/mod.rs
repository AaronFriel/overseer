use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use dyn_clonable::clonable;
use dyn_partial_eq::dyn_partial_eq;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{
  game::{Game, PlayerHandle, Visibility},
  interface::ChoiceOption,
};

#[derive(Clone, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub enum ActionResult {
  Step,
  Do(Box<dyn Action>),
  DoMulti(ActionList),
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
  Selected(usize),
  Unordered(Vec<usize>),
  Ordered(Vec<usize>),
}

#[typetag::serde(tag = "kind")]
#[dyn_partial_eq]
#[clonable]
pub trait Action: core::fmt::Debug + ActionHash + Clone {
  fn apply(&mut self, game: &mut Game, choice: PromptResult) -> ActionResult;
}

pub type ActionList = Vec<Box<dyn Action>>;

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
