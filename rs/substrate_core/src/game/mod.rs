mod card;
mod cardtype;
mod color;
mod mana_cost;
mod object;
mod object_color;
mod player;
mod target;
mod zone;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

pub use self::{
  card::*, cardtype::*, color::*, mana_cost::*, object::*, object_color::*, player::*, target::*,
  zone::*,
};
use crate::{
  action::{ChoicePrompt, SimpleAction},
  make_handle,
  util::Handle,
};

#[derive(Clone, Hash, Debug, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde_diff(opaque)]
/// Return value for actions.
pub enum StateAction {
  Do(Box<dyn SimpleAction>),
  Prompt(Box<dyn SimpleAction>, ChoicePrompt),
}

#[derive(Clone, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize/* , SerdeDiff */)]
pub struct Game {
  pub cards: Vec<Card>,
  pub players: Vec<Player>,
  pub active_player: Option<PlayerHandle>,

  pub log: Vec<StateAction>,
  /// The sequence of choices made in a game
  pub current_decision: usize,
  pub decisions: Vec<String>,
}

impl Game {
  // TODO: Determine a better way to declare a game and set of valid cards?
  pub fn new(cards: Vec<Card>, players: Vec<Player>) -> Self {
    Self {
      cards,
      players,
      active_player: Some(PlayerHandle::from_index(0)),
      log: Vec::new(),
      current_decision: 0,
      decisions: Vec::new(),
    }
  }
}

impl Game {
  pub fn get_active_player(&self) -> Option<&Player> {
    todo!()
    // self.active_player.map(|active_player|
    // &self.players[active_player.to_index()])
  }

  pub fn get_players(&self) -> impl Iterator<Item = PlayerHandle> {
    (0..self.players.len()).map(PlayerHandle::from_index)
  }

  pub fn get_player(&self, handle: PlayerHandle) -> &Player {
    &self.players[handle.to_index()]
  }

  pub fn get_player_mut(&mut self, handle: PlayerHandle) -> &mut Player {
    &mut self.players[handle.to_index()]
  }

  /// Set the game's active player.
  pub fn set_active_player(&mut self, active_player: PlayerHandle) {
    self.active_player = Some(active_player);
  }

  pub fn get_player_handles(&self) -> impl Iterator<Item = PlayerHandle> {
    (0..self.players.len()).map(PlayerHandle::from_index)
  }

  /// Set the game's active player.
  pub fn increment_active_player(&mut self) {
    self.active_player = self.active_player.map(|active_player| {
      PlayerHandle::from_index((active_player.to_index() + 1) % self.players.len())
    });
  }

  pub fn get_next_player_after(&self, player_handle: PlayerHandle) -> PlayerHandle {
    PlayerHandle::from_index((player_handle.to_index() + 1) % self.players.len())
  }

  pub fn push_decision(&mut self, value: impl ToString) {
    self.decisions.push(value.to_string());
  }

  pub fn get_decision(&mut self) -> Option<&str> {
    if let Some(decision) = self.decisions.get(self.current_decision) {
      self.current_decision += 1;
      Some(decision)
    } else {
      None
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub enum Visibility {
  AllPlayers,
  Players(Vec<PlayerHandle>),
}

make_handle!(PlayerHandle, u8);
make_handle!(ObjectHandle, u32);
