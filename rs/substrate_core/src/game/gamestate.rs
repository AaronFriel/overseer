use std::collections::HashSet;

use overseer_util::{make_handle, Handle};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::{
  action::{ChoicePrompt, SimpleAction},
  game::{Battlefield, Card, Command, Exile, ObjectHandle, ObjectPool, Player, Stack, Zone},
};

#[derive(Clone, Hash, Debug, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde_diff(opaque)]
/// Return value for actions.
pub enum StateAction {
  Do(Box<dyn SimpleAction>),
  Prompt(Box<dyn SimpleAction>, ChoicePrompt),
}

#[derive(Clone, Hash, Debug)]
#[derive(Serialize, Deserialize/* , SerdeDiff */)]
pub struct Game {
  pub cards: Vec<Card>,
  pub players: Vec<Player>,
  pub objects: ObjectPool,
  pub active_player: Option<PlayerHandle>,

  pub log: Vec<StateAction>,
  /// The sequence of choices made in a game
  pub current_decision: usize,
  pub decisions: Vec<String>,

  pub battlefield: Zone<Battlefield>,
  pub stack: Zone<Stack>,
  pub exile: Zone<Exile>,
  pub command: Zone<Command>,
}

impl Game {
  // TODO: Determine a better way to declare a game and set of valid cards?
  pub fn new(cards: Vec<Card>, players: Vec<Player>) -> Self {
    Self {
      cards,
      players,
      objects: ObjectPool::new(),
      active_player: Some(PlayerHandle::from_index(0)),

      battlefield: Zone::new(),
      stack: Zone::new(),
      exile: Zone::new(),
      command: Zone::new(),

      log: Vec::new(),
      current_decision: 0,
      decisions: Vec::new(),
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

  pub fn get_player(&self, player_handle: PlayerHandle) -> &Player {
    &self.players[player_handle.to_index()]
  }

  pub fn get_player_mut(&mut self, player_handle: PlayerHandle) -> &mut Player {
    &mut self.players[player_handle.to_index()]
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

  pub fn current_decision(&self) -> usize {
    self.current_decision
  }

  pub fn push_decision(&mut self, value: impl ToString) {
    self.decisions.push(value.to_string());
  }

  pub fn get_decision(&self, index: usize) -> Option<&str> {
    self.decisions.get(index).map(|x| x.as_ref())
  }

  fn enumerate_players(&self) -> impl Iterator<Item = (PlayerHandle, &Player)> {
    self.get_player_handles().zip(self.players.iter())
  }

  pub fn get_visible_set(&self, view_as_player_handle: PlayerHandle) -> HashSet<ObjectHandle> {
    let mut visible_set: HashSet<ObjectHandle> = HashSet::new();
    // let mut hidden_set: HashSet<ObjectHandle> = HashSet::new();

    for (player_handle, player) in self.enumerate_players() {
      merge_object_list(&mut visible_set, player.graveyard.iter());
      // merge_object_list(&mut hidden_set, player.library.iter());
      if player_handle == view_as_player_handle || player.controller == Some(view_as_player_handle)
      {
        merge_object_list(&mut visible_set, player.hand.iter());
      } else {
        // merge_object_list(&mut hidden_set, player.hand.iter());
      }
    }

    merge_object_list(&mut visible_set, self.battlefield.iter());
    merge_object_list(&mut visible_set, self.stack.iter());
    merge_object_list(&mut visible_set, self.exile.iter());
    merge_object_list(&mut visible_set, self.command.iter());

    visible_set
  }

  pub fn view_as_player(&self, view_as_player_handle: PlayerHandle) -> Self {
    let visible_set = self.get_visible_set(view_as_player_handle);

    let mut objects = self.objects.clone();

    for (k, _) in self.objects.iter_weak() {
      if !visible_set.contains(&k) {
        objects.remove(&k);
      }
    }

    Self {
      cards: self.cards.clone(),
      players: self.players.clone(),
      objects,
      active_player: self.active_player.clone(),

      battlefield: self.battlefield.clone(),
      stack: self.stack.clone(),
      exile: self.exile.clone(),
      command: self.command.clone(),

      log: self.log.clone(),
      current_decision: self.current_decision.clone(),
      decisions: self.decisions.clone(),
    }
  }
}

fn merge_object_list<'a>(
  set: &mut HashSet<ObjectHandle>,
  iter: impl Iterator<Item = &'a ObjectHandle>,
) {
  for item in iter {
    if !set.contains(item) {
      set.insert(item.weak_clone());
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
