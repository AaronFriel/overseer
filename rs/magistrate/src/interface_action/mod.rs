// pub mod decision;

use std::{fmt::Debug, hash::Hash};

use overseer_substrate::{
  action::{ActionResult, ComplexAction},
  game::Game,
};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Shuffle<T>
where
  for<'a> T: SerdeDiff + Serialize + Deserialize<'a>,
{
  #[serde(bound(deserialize = "Vec<T>: Deserialize<'de>"))]
  items: Vec<T>,
}

impl<T> ComplexAction for Shuffle<T>
where
  for<'a> T: Clone + Debug + SerdeDiff + Serialize + Deserialize<'a>,
{
  type Result = Vec<T>;

  fn apply(&mut self, game: &mut Game) -> ActionResult<Self::Result> {
    todo!()
  }
}

// Prompt(ChoicePrompt {
//   choices: (0..player.deck.len())
//     .map(|x| ChoiceOption::Custom(x.to_string()))
//     .collect(),
//   kind: PromptKind::Shuffle,
//   prompt: "".into(),
//   player: None,
//   visibility: Visibility::Players(vec![]),
// })
