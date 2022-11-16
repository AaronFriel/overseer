use dyn_partial_eq::*;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{ChoiceOption, ChoicePrompt, Game};

pub enum ActionResult {
  Step,
  Prompt(ChoicePrompt),
  Resolved,
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
pub trait Action: core::fmt::Debug {
  fn apply(&mut self, game: &mut Game) -> ActionResult;
}

impl std::hash::Hash for Box<dyn Action> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.typetag_name().hash(state);
    let json_repr = serde_json::to_string(&self).unwrap_or("unhashable".to_string());

    json_repr.hash(state)
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
