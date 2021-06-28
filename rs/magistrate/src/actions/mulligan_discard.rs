use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{action::*, game::*, interface::*};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct MulliganDiscard {
  pub player: PlayerHandle,
  pub discard_to: usize,
}

#[typetag::serde]
impl Action for MulliganDiscard {
  fn apply(&mut self, game: &mut Game, choice: PromptResult) -> ActionResult {
    use ActionResult::*;

    match choice {
      PromptResult::None => {
        let player = game.get_player_mut(self.player);

        if player.hand.cards.len() > self.discard_to {
          let choices: Vec<ChoiceOption> = player
            .hand
            .cards
            .iter()
            .map(|x| ChoiceOption::Custom(x.name.to_string()))
            .collect();

          Prompt(ChoicePrompt {
            player_handle: Some(self.player),
            kind: PromptKind::Select,
            visibility: Visibility::Players(vec![self.player]),
            choices,
            prompt: "Choose a card to place on the bottom of your library".into(),
          })
        } else {
          Resolved
        }
      }
      PromptResult::Selected(card_idx) => {
        let player = game.get_player_mut(self.player);

        let removed_card = player.hand.cards.remove(card_idx);
        println!("Moving card {} to bottom of library", removed_card.name);
        player.library.cards.insert(0, removed_card);

        Step
      }
      uhoh => {
        unimplemented!("Error handling. Oops: {:?}", uhoh);
      }
    }
  }
}
