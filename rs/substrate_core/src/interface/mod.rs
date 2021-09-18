mod choice;
mod decision;
mod prompt;
mod viewable;

use serde::{Deserialize, Serialize};

pub use self::{choice::*, decision::*, prompt::*, viewable::*};
use crate::game::PlayerHandle;

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
  fn prompt_yes_no(&mut self, player: PlayerHandle) -> InterfaceResult<YesNo>;
}
