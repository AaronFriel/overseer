use dyn_partial_eq::*;

use crate::{ChoicePrompt, Game, StateAction};

#[typetag::serde(tag = "kind")]
#[dyn_partial_eq]
pub trait Choice: core::fmt::Debug {
  fn get_choice(&self) -> ChoicePrompt;
  fn apply_choice(self, option: usize, game: &mut Game) -> StateAction;
}
