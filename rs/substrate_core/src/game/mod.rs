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
use crate::action::{Action, ChoicePrompt};

#[derive(Clone, Hash, Debug, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde_diff(opaque)]
/// Return value for actions.
pub enum StateAction {
  Do(Box<dyn Action>),
  Prompt(Box<dyn Action>, ChoicePrompt),
}

#[derive(Clone, PartialEq, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Game {
  pub cards: CardList,
  pub players: Vec<Player>,
  pub active_player: PlayerHandle,

  pub log: Vec<StateAction>,
}

impl Game {
  pub fn new(cards: CardList, players: Vec<Player>) -> Self {
    Self {
      cards,
      players,
      active_player: PlayerHandle(0),
      log: Vec::new(),
    }
  }

  pub fn get_active_player(&self) -> &Player {
    &self.players[self.active_player.to_index()]
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
    self.active_player = active_player;
  }

  pub fn get_player_handles(&self) -> impl Iterator<Item = PlayerHandle> {
    (0..self.players.len()).map(PlayerHandle::from_index)
  }

  /// Set the game's active player.
  pub fn increment_active_player(&mut self) {
    self.active_player =
      PlayerHandle::from_index((self.active_player.to_index() + 1) % self.players.len());
  }

  pub fn get_next_player_after(&self, player_handle: PlayerHandle) -> PlayerHandle {
    PlayerHandle::from_index((player_handle.to_index() + 1) % self.players.len())
  }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(transparent)]
#[serde_diff(opaque)]
pub struct PlayerHandle(usize);

impl PlayerHandle {
  pub(crate) fn from_index(value: usize) -> Self {
    PlayerHandle(value as usize)
  }

  pub(crate) fn to_index(self) -> usize {
    self.0 as usize
  }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct ObjectHandle {
  zone: ZoneKind,
  player: PlayerHandle,
  index: usize,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub enum Visibility {
  AllPlayers,
  Players(Vec<PlayerHandle>),
}
