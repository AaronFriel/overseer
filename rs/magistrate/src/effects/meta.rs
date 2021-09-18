use async_trait::async_trait;

use overseer_substrate_core::{
  game::{Game, PlayerHandle},
  session::Session,
};

use super::{Event, EventList};

pub struct Lose {
  pub player: PlayerHandle,
}

#[async_trait]
impl<S: Session> Event<S> for Lose {
  async fn apply(self, game: &mut Game) -> EventList<S> {
    game.get_player_mut(self.player_handle).has_lost_game = true;

    None
  }
}

pub struct Leave {
  pub player: PlayerHandle,
}

#[async_trait]
impl<S: Session> Event<S> for Leave {
  async fn apply(self, game: &mut Game) -> EventList<S> {
    game.get_player_mut(self.player_handle).has_left_game = true;

    None
  }
}
