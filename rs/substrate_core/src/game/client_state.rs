use std::{collections::HashSet, fmt::Display};

use overseer_util::{make_handle, Handle};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{
  action::{ChoicePrompt, SimpleAction},
  game::{Battlefield, Card, Command, Exile, ObjectHandle, ObjectPool, Player, Stack, Zone},
  interface::DecisionList,
};

#[derive(Clone, Hash, Debug, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde_diff(opaque)]
/// Return value for actions.
pub enum StateAction {
  Do(Box<dyn SimpleAction>),
  Prompt(Box<dyn SimpleAction>, ChoicePrompt),
}

#[derive(Clone, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ClientState {
  pub cards: Vec<Card>,
  pub players: Vec<Player>,
  pub active_player: Option<PlayerHandle>,

  pub current_player: Option<PlayerHandle>,

  pub objects: ObjectPool,

  pub battlefield: Zone<Battlefield>,
  pub stack: Zone<Stack>,
  pub exile: Zone<Exile>,
  pub command: Zone<Command>,

  pub decisions: DecisionList,
}

impl ClientState {
  // TODO: Determine a better way to declare a game and set of valid cards?
  pub fn new(cards: Vec<Card>, players: Vec<Player>) -> Self {
    Self {
      cards,
      players,
      active_player: Some(PlayerHandle::from_index(0)),

      current_player: None,

      objects: ObjectPool::new(),

      battlefield: Zone::new(),
      stack: Zone::new(),
      exile: Zone::new(),
      command: Zone::new(),
    }
  }

  pub fn get_active_player(&self) -> Option<&Player> {
    todo!()
    // self.active_player.map(|active_player|
    // &self.players[active_player.to_index()])
  }

  pub fn get_players(&self) -> impl Iterator<Item = PlayerHandle> {
    (0..self.players.len()).map(PlayerHandle::from_index)
  }

  pub fn get_players_from(&self, player: PlayerHandle) -> impl Iterator<Item = PlayerHandle> {
    let len = self.players.len();

    (0..self.players.len())
      .map(move |i| (i + player.to_index()) % len)
      .map(PlayerHandle::from_index)
  }

  pub fn get_players_from_active(&self) -> Option<impl Iterator<Item = PlayerHandle>> {
    if let Some(active_player) = self.active_player {
      Some(self.get_players_from(active_player))
    } else {
      None
    }
  }

  pub fn get_player(&self, player: PlayerHandle) -> &Player {
    &self.players[player.to_index()]
  }

  pub fn get_player_mut(&mut self, player: PlayerHandle) -> &mut Player {
    &mut self.players[player.to_index()]
  }

  /// Set the game's active player.
  pub fn set_active_player(&mut self, active_player: PlayerHandle) {
    self.active_player = Some(active_player);
  }

  /// Set the game's active player.
  pub fn increment_active_player(&mut self) {
    self.active_player = self.active_player.map(|active_player| {
      PlayerHandle::from_index((active_player.to_index() + 1) % self.players.len())
    });
  }

  pub fn get_next_player_after(&self, player: PlayerHandle) -> PlayerHandle {
    PlayerHandle::from_index((player.to_index() + 1) % self.players.len())
  }

  /*
   */
  fn enumerate_players(&self) -> impl Iterator<Item = (PlayerHandle, &Player)> {
    self.get_players().zip(self.players.iter())
  }

  pub fn get_visible_set(&self, player_handle: PlayerHandle) -> HashSet<ObjectHandle> {
    let mut visible_set: HashSet<ObjectHandle> = HashSet::new();
    // let mut hidden_set: HashSet<ObjectHandle> = HashSet::new();

    for (other_player_handle, player) in self.enumerate_players() {
      merge_object_list(&mut visible_set, player.graveyard.iter_weak());
      // merge_object_list(&mut hidden_set, player.library.iter());
      if other_player_handle == player_handle || player.controller == Some(player_handle) {
        merge_object_list(&mut visible_set, player.hand.iter_weak());
        merge_object_list(&mut visible_set, player.graveyard.iter_weak());
      } else {
        // merge_object_list(&mut hidden_set, player.hand.iter());
      }
    }

    merge_object_list(&mut visible_set, self.battlefield.iter_weak());
    merge_object_list(&mut visible_set, self.stack.iter_weak());
    merge_object_list(&mut visible_set, self.exile.iter_weak());
    merge_object_list(&mut visible_set, self.command.iter_weak());

    visible_set
  }

  pub fn view_as_player(&self, player: PlayerHandle) -> Self {
    let visible_set = self.get_visible_set(player);

    let visible_set_filter = |handle: &ObjectHandle| visible_set.contains(handle);

    let players = self
      .players
      .iter()
      .map(|p| p.clone_visible(visible_set_filter))
      .collect();

    let battlefield = self.battlefield.into_filtered_view(visible_set_filter);
    let stack = self.stack.into_filtered_view(visible_set_filter);
    let exile = self.exile.into_filtered_view(visible_set_filter);
    let command = self.command.into_filtered_view(visible_set_filter);

    Self {
      cards: self.cards.clone(),
      players,
      active_player: self.active_player,
      current_player: Some(player),

      // TOOD: Not accurate
      objects: self.objects.clone(),

      battlefield,
      stack,
      exile,
      command,
    }
  }
}

fn merge_object_list<'a>(
  set: &mut HashSet<ObjectHandle>,
  iter: impl IntoIterator<Item = ObjectHandle>,
) {
  for item in iter {
    if !set.contains(&item) {
      set.insert(item);
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

impl Display for PlayerHandle {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.number().fmt(f)
  }
}
