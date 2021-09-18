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
pub struct SetupLibrary {
  player: PlayerHandle,
  decision: DecisionHandle,
}

impl SetupLibrary {
  pub fn new(player: PlayerHandle, game: &mut Game) -> Self {
    Self {
      player,
      decision: game.reserve_decision(),
    }
  }
}

#[typetag::serde]
impl SimpleAction for SetupLibrary {
  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    use ActionErr::*;

    if let Ok(_) = game.wrap_decision_public(
      &self.decision,
      |state, objects| server_side_setup(state, objects, self.player),
      |state, library| {
        let player = state.get_player_mut(self.player);
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
fn server_side_setup(
  state: &State,
  objects: &mut ObjectPool,
  player: PlayerHandle,
) -> Zone<Library> {
  use rand::{prelude::SliceRandom, rngs::OsRng};

  let mut deck = state.get_player(player).deck.clone();
  deck.shuffle(&mut OsRng);

  deck
    .into_iter()
    .map(|card| Object {
      owner: Some(player),
      ..(card.into())
    })
    .map(|object| objects.insert(object))
    .collect()
}

#[cfg(not(feature = "server"))]
fn server_side_setup(game: &State, player: PlayerHandle) -> Zone<Library> {
  unimplemented!()
}
