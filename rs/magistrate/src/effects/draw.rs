use std::vec;

use async_trait::async_trait;
use overseer_substrate_core::{
  game::{Game, PlayerHandle},
  session::Session,
};

use super::{Event, EventList, Lose};

struct Draw {
  player_handle: PlayerHandle,
}

#[async_trait]
impl<S: Session> Event<S> for Draw {
  async fn apply(self, game: &mut Game) -> EventList<S> {
    // TODO: fix this?
    let player = game.get_player_mut(self.player_handle);

    if let Some(top_card) = player.library.cards.pop() {
      player.hand.cards.push(top_card);
    } else {
      return Some(vec![Box::new(DrawEmpty {
        player: self.player_handle,
      })]);
    }

    None
  }
}

pub struct DrawEmpty {
  pub player: PlayerHandle,
}

#[async_trait]
impl<S: Session> Event<S> for DrawEmpty {
  async fn apply(self, _: &mut Game) -> EventList<S> {
    Some(vec![Box::new(Lose {
      player_handle: self.player,
    })])
  }
}
