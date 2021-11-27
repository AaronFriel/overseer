mod decision;
mod prompt;
mod target;
mod viewable;

use serde::{Deserialize, Serialize};

pub use self::{decision::*, prompt::*, target::*, viewable::*};
use crate::game::{ClientState, ObjectPool, PlayerHandle};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub enum InterfaceError {
  Waiting,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub enum YesNo {
  Yes,
  No,
}

pub type InterfaceResult<T> = std::result::Result<T, InterfaceError>;

pub trait UserInterface {
  fn prompt_yes_no(
    &mut self,
    state: &ClientState,
    player: PlayerHandle,
    prompt: &str,
  ) -> InterfaceResult<YesNo>;

  fn prompt_target_select(
    &mut self,
    state: &ClientState,
    objects: &ObjectPool,
    player: PlayerHandle,
    targets: &[Target],
  ) -> InterfaceResult<Target>;
}
