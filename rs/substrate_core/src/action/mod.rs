use core::fmt::Debug;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use dyn_clonable::clonable;
use dyn_partial_eq::dyn_partial_eq;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{
  game::{Game, PlayerHandle, Visibility},
  interface::{InterfaceError, Target},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ActionError {
  Step,
  Waiting,
}

impl From<InterfaceError> for ActionError {
  fn from(err: InterfaceError) -> Self {
    match err {
      InterfaceError::Waiting => ActionError::Waiting,
    }
  }
}

pub type ActionResult<T> = Result<T, ActionError>;

pub trait ActionResultLike {
  fn as_simple(self) -> ActionResult<()>;
}

impl<T> ActionResultLike for ActionResult<T> {
  fn as_simple(self) -> ActionResult<()> {
    self.map(|_| ())
  }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ActionThunk<A>
where
  A: ComplexAction,
{
  Resolved(A::Result),
  Action(A),
}

impl<A> From<A> for ActionThunk<A>
where
  A: ComplexAction,
{
  fn from(action: A) -> Self {
    ActionThunk::Action(action)
  }
}

impl<A> ComplexAction for ActionThunk<A>
where
  A: ComplexAction,
  A::Result: Clone,
{
  type Result = A::Result;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    self.apply_cloned(game)
  }
}

impl<A> ActionThunk<A>
where
  A: ComplexAction,
{
  pub fn apply_copied(&mut self, game: &mut Game) -> ActionResult<<A as ComplexAction>::Result>
  where
    <A as ComplexAction>::Result: Copy,
  {
    match self {
      ActionThunk::Resolved(value) => Ok(*value),
      ActionThunk::Action(action) => {
        let value = action.apply(game)?;

        *self = ActionThunk::Resolved(value);

        Ok(value)
      }
    }
  }

  pub fn apply_cloned(&mut self, game: &mut Game) -> ActionResult<<A as ComplexAction>::Result>
  where
    <A as ComplexAction>::Result: Clone,
  {
    match self {
      ActionThunk::Resolved(value) => Ok(value.clone()),
      ActionThunk::Action(action) => {
        let value = action.apply(game)?;

        *self = ActionThunk::Resolved(value.clone());

        Ok(value)
      }
    }
  }

  pub fn apply_borrowed(&mut self, game: &mut Game) -> ActionResult<&<A as ComplexAction>::Result> {
    match self {
      ActionThunk::Resolved(value) => Ok(value),
      ActionThunk::Action(action) => {
        let value = action.apply(game)?;

        *self = ActionThunk::Resolved(value);

        match self {
          ActionThunk::Resolved(value) => Ok(value),
          ActionThunk::Action(_) => unreachable!(),
        }
      }
    }
  }
}

#[derive(Clone, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct SimpleActionResult(ActionResult<()>);

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct ChoicePrompt {
  pub player: Option<PlayerHandle>,
  pub kind: PromptKind,
  pub prompt: String,
  pub choices: Vec<Target>,
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
  fn perform(&mut self, game: &mut Game) -> ActionResult<()>;
}

pub trait ComplexAction {
  type Result;

  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    self.apply(game).as_simple()
  }

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result>;
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
