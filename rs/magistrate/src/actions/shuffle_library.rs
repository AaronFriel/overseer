use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::{PlayerHandle, *},
  interface::DecisionHandle,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct ShuffleLibrary {
  player: PlayerHandle,
  decision: DecisionHandle,
}

impl ShuffleLibrary {
  pub fn new(player: PlayerHandle, game: &mut Game) -> Self {
    Self {
      player,
      decision: game.reserve_decision(),
    }
  }
}

#[typetag::serde]
impl SimpleAction for ShuffleLibrary {
  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    use ActionErr::*;

    if let Ok(_) = game.wrap_decision_public(
      self.decision,
      "",
      |game, objects| perform(game, objects, self.player),
      |game, library| {
        let player = game.get_player_mut(self.player);
        player.library = library.take();
      },
    ) {
      Ok(())
    } else {
      Err(Waiting)
    }
  }
}

#[cfg(feature = "server")]
fn perform(game: &ClientState, objects: &mut ObjectPool, player: PlayerHandle) -> Zone<Library> {
  use rand::{prelude::SliceRandom, rngs::OsRng};

  let mut library: Vec<_> = game
    .get_player(player)
    .library
    .iter()
    .map(ObjectHandle::weak_clone)
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