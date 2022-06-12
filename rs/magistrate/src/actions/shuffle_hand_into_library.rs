use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::{PlayerHandle, *},
  interface::DecisionHandle,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use super::shuffle_library::ShuffleLibrary;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct ShuffleHandIntoLibrary {
  player: PlayerHandle,
  decision: DecisionHandle,
  shuffle: Option<ShuffleLibrary>,
}

impl ShuffleHandIntoLibrary {
  pub fn new(game: &mut Game, player: PlayerHandle) -> Self {
    Self {
      player,
      decision: game.reserve_decision(),
      shuffle: None,
    }
  }
}

#[typetag::serde]
impl SimpleAction for ShuffleHandIntoLibrary {
  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    ComplexAction::apply(self, game)
  }
}

impl ComplexAction for ShuffleHandIntoLibrary {
  type Result = ();

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use ActionError::*;

    let player_handle = self.player;

    if let Ok(_) = game.wrap_decision_public(
      self.decision,
      "",
      |client, server| perform(client, &mut server.objects, player_handle),
      |game, library| {
        let player = game.get_player_mut(player_handle);
        player.library = library.take();
        player.hand.clear();
      },
    ) {
      let shuffle = self
        .shuffle
        .get_or_insert_with(|| ShuffleLibrary::new(player_handle, game));

      SimpleAction::perform(shuffle, game)
    } else {
      Err(Waiting)
    }
  }
}

#[cfg(feature = "server")]
fn perform(client: &ClientState, objects: &mut ObjectPool, player: PlayerHandle) -> Zone<Library> {
  #[cfg(feature = "server")]
  use rand::{prelude::SliceRandom, rngs::OsRng};

  let player = client.get_player(player);
  let mut library: Vec<_> = (player.hand.iter().map(ObjectHandle::weak_clone))
    .chain(player.library.iter().map(ObjectHandle::weak_clone))
    .collect();
  library.shuffle(&mut OsRng);

  library
    .into_iter()
    .map(|handle| objects.reinsert(&handle).unwrap())
    .collect()
}

#[cfg(not(feature = "server"))]
fn perform(game: &ClientState, player: PlayerHandle) -> Zone<Library> {
  unimplemented!()
}
