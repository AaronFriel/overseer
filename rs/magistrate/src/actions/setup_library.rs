use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::{PlayerHandle, *},
  interface::DecisionHandle,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
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
    ComplexAction::apply(self, game)
  }
}

impl ComplexAction for SetupLibrary {
  type Result = ();

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use ActionError::*;

    if let Ok(_) = game.wrap_decision_public(
      self.decision,
      "",
      |client, server| server_side_setup(client, &mut server.objects, self.player),
      |state, library| {
        let player = state.get_player_mut(self.player);
        player.library = library.take();
      },
    ) {
      // let player = game.client.get_player(self.player);
      // game.server.objects.reassociate(player.library.iter());
      Ok(())
    } else {
      Err(Waiting)
    }
  }
}

#[cfg(feature = "server")]
fn server_side_setup(
  state: &ClientState,
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
fn server_side_setup(game: &ClientState, player: PlayerHandle) -> Zone<Library> {
  unimplemented!()
}
