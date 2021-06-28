use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::{PlayerHandle, *},
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;


#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]

pub struct Draw {
  pub player: PlayerHandle,
  pub count: u16,
}



#[typetag::serde]
impl Action for Draw {
  fn apply(&mut self, _: &mut Game, _: PromptResult) -> ActionResult {
    use ActionResult::*;

    if self.count > 0 {
      println!("  DRAW: Drawing {} cards", self.count);

      let collect: ActionList = std::iter::repeat(
          (DrawOne {
            player: self.player,
          })
          .into(),
        )
        .take(self.count as usize)
        .collect();

      self.count = 0;
      DoMulti(
        collect,
      )
    } else {
      println!("  DRAW: Complete");
      Resolved
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct DrawOne {
  pub player: PlayerHandle,
}

#[typetag::serde]
impl Action for DrawOne {
  fn apply(&mut self, game: &mut Game, _: PromptResult) -> ActionResult {

    use ActionResult::*;

    let player = game.get_player_mut(self.player);

    if let Some(card) = player.library.cards.pop() {
      player.hand.cards.push(card);
    } else {
      // TODO: emit game loss effect
    }

    Resolved
  }
}
