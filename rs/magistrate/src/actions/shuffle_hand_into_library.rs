use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{
  action::*,
  game::{PlayerHandle, *},
  interface::*,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct ShuffleHandIntoLibrary {
  pub player: PlayerHandle,
}

#[typetag::serde]
impl Action for ShuffleHandIntoLibrary {
  fn apply(&mut self, game: &mut Game, choice: PromptResult) -> ActionResult {
    use ActionResult::*;

    match choice {
      PromptResult::None => {
        let player = game.get_player_mut(self.player);

        // Place all cards from hand into library:
        player.library.cards.append(&mut player.hand.cards);

        Prompt(ChoicePrompt {
          choices: (0..player.library.cards.len())
            .map(|x| ChoiceOption::Custom(x.to_string()))
            .collect(),
          kind: PromptKind::Shuffle,
          prompt: "".into(),
          player_handle: None,
          visibility: Visibility::Players(vec![]),
        })
      }
      PromptResult::Ordered(sequence) => {
        let library = &mut game.get_player_mut(self.player).library.cards;
        let mut temp = Vec::<Card>::with_capacity(library.len());

        for idx in sequence.iter() {
          temp.push(library[*idx].clone());
        }

        std::mem::swap(&mut temp, library);

        Resolved
      }
      _ => todo!(),
    }
  }
}
