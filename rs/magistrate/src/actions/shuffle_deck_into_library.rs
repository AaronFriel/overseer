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
pub struct ShuffleDeckIntoLibrary {
  pub player: PlayerHandle,
}

#[typetag::serde]
impl Action for ShuffleDeckIntoLibrary {
  fn apply(&mut self, game: &mut Game, choice: PromptResult) -> ActionResult {
    use ActionResult::*;

    match choice {
      PromptResult::None => {
        let player = game.get_player_mut(self.player);

        Prompt(ChoicePrompt {
          choices: (0..player.deck.len())
            .map(|x| ChoiceOption::Custom(x.to_string()))
            .collect(),
          kind: PromptKind::Shuffle,
          prompt: "".into(),
          player_handle: None,
          visibility: Visibility::Players(vec![]),
        })
      }
      PromptResult::Ordered(choices) => {
        let player = game.get_player_mut(self.player);

        let source = &mut player.deck;
        let dest = &mut player.library.cards;
        let mut temp = Vec::<Card>::with_capacity(source.len());

        for idx in choices.iter() {
          temp.push(source[*idx].clone());
        }

        std::mem::swap(&mut temp, dest);

        Resolved
      }
      _ => todo!(),
    }
  }
}
