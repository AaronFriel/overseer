use core::fmt::Debug;
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
#[derive(Serialize, Deserialize)]
pub enum ActionResult<T> {
  Step,
  Resolved(T),
}

#[derive(Clone, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct SimpleActionResult(ActionResult<()>);

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

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct ChoicePromptTwo<T>
where
  for<'t> T: SerdeDiff + Serialize + Deserialize<'t>,
{
  pub player_handle: Option<PlayerHandle>,
  pub kind: PromptKind,
  pub prompt: String,
  #[serde(bound(deserialize = "Vec<T>: Deserialize<'de>"))]
  pub choices: Vec<T>,
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
pub trait SimpleAction: Debug + ActionHash + Clone {
  fn apply(&mut self, game: &mut Game) -> ActionResult<()>;
}

pub trait ComplexAction<T>: Debug + Clone {
  fn apply(&mut self, game: &mut Game) -> ActionResult<T>;
}

impl<T> ComplexAction<()> for T
where
  T: SimpleAction + Clone,
{
  fn apply(&mut self, game: &mut Game) -> ActionResult<()> {
    self.apply(game)
  }
}

pub type ActionList = Vec<Box<dyn SimpleAction>>;

pub trait ActionHash {
  fn hash_value(&self) -> u64;
}

impl std::hash::Hash for Box<dyn SimpleAction> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.typetag_name().hash(state);
    self.hash_value().hash(state);
  }
}

impl<T> From<T> for Box<dyn SimpleAction>
where
  T: SimpleAction + 'static,
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
