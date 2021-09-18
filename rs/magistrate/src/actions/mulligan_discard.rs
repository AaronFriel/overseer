use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{action::*, game::*, interface::DecisionHandle};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize)]
pub struct MulliganDiscard {
  player: PlayerHandle,
  discard_to: usize,
  decision: DecisionHandle,
  discards: SmallVec<[ObjectHandle; 7]>,
}

impl MulliganDiscard {
  pub fn new(
    game: &mut Game,
    player: PlayerHandle,
    discard_to: usize,
    discards: SmallVec<[ObjectHandle; 7]>,
  ) -> Self {
    Self {
      player,
      discard_to,
      decision: game.reserve_decision(),
      discards,
    }
  }
}

#[typetag::serde]
impl SimpleAction for MulliganDiscard {
  fn perform(&mut self, _: &mut Game) -> ActionResult<()> {
    // use ActionResult::*;

    todo!()

    // match self.decision_handle {
    //   None => {
    //     self.decision_handle = Some(game.current_decision());

    //     Step
    //   }
    //   Some(decision_index) => {
    //     let mut handle = if let Some(outcome) =
    // game.get_decision(decision_index) {       let mut handle:
    // ObjectHandle = serde_json::from_str(outcome).unwrap();

    //       // Reassociate all handles that we can with our game state
    //       game.objects.reassociate(&mut handle);

    //       handle
    //     } else {
    //       // Server side assumed
    //       // let result = server_side_setup(game, self.player_handle);

    //       game.push_decision(serde_json::to_string(c & result).unwrap());

    //       result
    //     };

    //     // Set the player's library as the result:
    //     let player = game.get_player_mut(self.player_handle);
    //     player.library.cards.clear();
    //     player.library.cards.append(&mut handle);

    //     Resolved(())
    //   }
    // }

    // todo!()
    // let mut result: ObjectHandle = if let Some(outcome) =
    // game.get_decision(self.decision_index) {

    // }
    // match choice {
    //   PromptResult::None => {
    //     let player = game.get_player_mut(self.player);

    //     if player.hand.cards.len() > self.discard_to {
    //       todo!("Refactor prompt as an interface action")
    //       // let choices: Vec<ChoiceOption> = player
    //       //   .hand
    //       //   .cards
    //       //   .iter()
    //       //   .map(|x| ChoiceOption::Custom(x.name.to_string()))
    //       //   .collect();

    //       // Prompt(ChoicePrompt {
    //       //   player Some(self.player),
    //       //   kind: PromptKind::Select,
    //       //   visibility: Visibility::Players(vec![self.player]),
    //       //   choices,
    //       //   prompt: "Choose a card to place on the bottom of your
    // library".into(),       // })
    //     } else {
    //       Resolved(())
    //     }
    //   }
    //   PromptResult::Selected(card_index) => {
    //     let player = game.get_player_mut(self.player);

    //     let removed_card = player.hand.cards.remove(card_index);
    //     println!("Moving card {} to bottom of library", removed_card.name);
    //     player.library.cards.insert(0, removed_card);

    //     Step
    //   }
    //   uhoh => {
    //     unimplemented!("Error handling. Oops: {:?}", uhoh);
    //   }
    // }
  }
}
