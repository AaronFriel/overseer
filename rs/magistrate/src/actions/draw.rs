use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::{ClientState, PlayerHandle, *},
  interface::DecisionHandle,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]

pub struct Draw {
  pub player: PlayerHandle,
  pub draw_actions: SmallVec<[DrawOne; 7]>,
  pub drawn_cards: Vec<ObjectHandle>,
}

impl Draw {
  pub fn new(game: &mut Game, player: PlayerHandle, count: u16) -> Self {
    Self {
      player,
      draw_actions: std::iter::repeat_with(|| DrawOne::new(game, player).into())
        .take(count as usize)
        .collect(),
      drawn_cards: Vec::new(),
    }
  }
}

#[typetag::serde]
impl SimpleAction for Draw {
  fn perform(&mut self, game: &mut Game) -> ActionResult<()> {
    ComplexAction::perform(self, game)
  }
}

impl ComplexAction for Draw {
  type Result = SmallVec<[ObjectHandle; 7]>;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use ActionError::*;
    if let Some(mut draw) = self.draw_actions.pop() {
      match ComplexAction::apply(&mut draw, game) {
        Ok(Some(handle)) => {
          self.drawn_cards.push(handle);
          Err(Step)
        }
        Ok(None) => Err(Step),
        Err(Step | Waiting) => {
          self.draw_actions.push(draw);
          Err(Step)
        }
      }
    } else {
      Ok(self.drawn_cards.iter().cloned().collect())
    }
  }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct DrawOne {
  pub player: PlayerHandle,
  pub decision: DecisionHandle,
}

impl DrawOne {
  pub fn new(game: &mut Game, player: PlayerHandle) -> Self {
    Self {
      player,
      decision: game.reserve_decision(),
    }
  }
}

impl ComplexAction for DrawOne {
  type Result = Option<ObjectHandle>;

  #[tracing::instrument(skip(game))]
  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    use ActionError::*;

    if let Ok(result) = game.wrap_decision_public(
      self.decision,
      format!(
        "Player {} draws?",
        game.state().get_player(self.player).name,
      ),
      |client, server| server_side_draw(client, &mut server.objects, self.player),
      |client, result| {
        if let Some((card_to_remove, hand)) = result.cloned() {
          let player = client.get_player_mut(self.player);

          dbg!(&player.hand);
          dbg!(&card_to_remove);
          tracing::debug!(cards_in_library = ?player.library.count(), cards_in_hand = ?player.hand.count());
          // player.library.remove(&card_to_remove);
          player.hand = hand;
        } else {
          todo!()
        }
      },
    ) {
      if let Some((handle, _)) = result.cloned() {
        Ok(Some(handle))
      } else {
        Ok(None)
      }
    } else {
      Err(Waiting)
    }
  }
}

#[cfg(feature = "server")]
fn server_side_draw(
  client: &ClientState,
  objects: &mut ObjectPool,
  player: PlayerHandle,
) -> Option<(ObjectHandle, Zone<Hand>)> {
  use std::iter::once;

  use rand::{prelude::SliceRandom, rngs::OsRng};

  let player = client.get_player(player);

  if let Some(card) = player.library.get_top(0).cloned() {
    let mut hand: Vec<_> = player
      .hand
      .iter()
      .cloned()
      .chain(once(card.clone()))
      .collect();
    hand.shuffle(&mut OsRng);
    let hand = hand
      .into_iter()
      .map(|handle| objects.reinsert(&handle).unwrap())
      .collect();

    Some((card, hand))
  } else {
    None
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct DrawEmpty {
  pub player: PlayerHandle,
  pub decision: DecisionHandle,
}

impl DrawEmpty {
  pub fn new(game: &mut Game, player: PlayerHandle) -> Self {
    Self {
      player,
      decision: game.reserve_decision(),
    }
  }
}
