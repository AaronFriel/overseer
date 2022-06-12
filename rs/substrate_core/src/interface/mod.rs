mod decision;
mod prompt;
mod target;
mod viewable;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

pub use self::{decision::*, prompt::*, target::*, viewable::*};
use crate::game::{ClientState, ObjectHandle, ObjectPool, PlayerHandle, ServerState};

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
  fn prompt_yes_no(&mut self, client: &ClientState, prompt: &str) -> InterfaceResult<YesNo>;

  fn prompt_target_select(
    &mut self,
    client: &ClientState,
    server: &ServerState,
    targets: &[Target],
    prompt: &str,
  ) -> InterfaceResult<Target>;

  fn prompt_target_multi_select(
    &mut self,
    client: &ClientState,
    server: &ServerState,
    targets: &[Target],
    prompt: &str,
  ) -> InterfaceResult<Vec<Target>>;

  fn prompt_mulligan_return(
    &mut self,
    client: &ClientState,
    server: &ServerState,
    prompt: &str,
  ) -> InterfaceResult<Vec<Target>>;
}
