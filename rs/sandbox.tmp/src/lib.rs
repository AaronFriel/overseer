mod choice_receiver;
mod prompt;
mod traits;
mod utils;

use std::{borrow::Cow, fmt::Debug, hash::Hash, usize};

use dyn_partial_eq::*;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
pub use traits::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/**
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(transparent)]
#[serde_diff(opaque)]
*/

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn diff_gamestate(old_value: JsValue, new_value: JsValue) -> JsValue {
  let old: Game = old_value.into_serde().unwrap();
  let new: Game = new_value.into_serde().unwrap();

  let diff = serde_diff::Diff::serializable(&old, &new);

  JsValue::from_serde(&diff).unwrap()
}

#[wasm_bindgen]
pub fn greet() {
  let _ = Game::default();

  alert("Hello, playground!");
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Player {
  hand: Vec<usize>,
  library: Vec<usize>,
  deck: Vec<usize>,
}

impl Player {
  fn new(hand: Vec<usize>, library: Vec<usize>, deck: Vec<usize>) -> Self {
    Self {
      hand,
      library,
      deck,
    }
  }
}

#[derive(PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct GameState {
  pub stack: Vec<StateAction>,
  pub log: Vec<StateAction>,
  pub game: Game,
}

impl GameState {
  pub fn new(stack: Vec<StateAction>) -> Self {
    Self {
      stack,
      log: Vec::default(),
      game: Game::default(),
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Game {
  pub active_player: PlayerHandle,
  pub players: Vec<Player>,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct PlayerHandle(usize);

impl PlayerHandle {
  pub fn number(&self) -> usize {
    return self.0 + 1;
  }
}

impl Game {
  pub fn new(players: Vec<Player>) -> Self {
    Self {
      players,
      active_player: PlayerHandle(0),
    }
  }

  pub fn get_player_handles(&self) -> impl Iterator<Item = PlayerHandle> {
    (0..self.players.len()).map(PlayerHandle)
  }

  pub fn enumerate_players(&self) -> impl Iterator<Item = (PlayerHandle, &Player)> {
    self.get_player_handles().zip(self.players.iter())
  }

  pub fn get_player(&self, player_handle: PlayerHandle) -> Option<&Player> {
    let PlayerHandle(idx) = player_handle;
    self.players.get(idx)
  }

  pub fn with_player_mut<F: FnOnce(&mut Player)>(&mut self, player_handle: PlayerHandle, f: F) {
    let PlayerHandle(idx) = player_handle;
    if let Some(player) = self.players.get_mut(idx) {
      f(player);
    }
  }

  pub fn get_next_active_player(&self) -> PlayerHandle {
    let i = self.active_player.0;

    PlayerHandle((i + 1) % self.players.len())
  }

  pub fn get_next_player_after(&self, player_handle: PlayerHandle) -> PlayerHandle {
    PlayerHandle((player_handle.0 + 1) % self.players.len())
  }

  pub fn set_active_player(&mut self, player_handle: PlayerHandle) {
    self.active_player = player_handle;
  }

  pub fn pass_to_next_player(&mut self) {
    self.active_player = self.get_next_active_player();
  }
}
#[derive(Debug, PartialEq, SerdeDiff, Serialize, Deserialize)]
#[serde_diff(opaque)]
/// Return value for actions.
pub enum StateAction {
  Do(Box<dyn Action>),
  Prompt(Box<dyn Action>, ChoicePrompt),
}

impl From<Box<dyn Action>> for StateAction {
  fn from(action: Box<dyn Action>) -> Self {
    StateAction::Do(action)
  }
}

impl<T> From<T> for StateAction
where
  T: Action + 'static,
{
  #[inline(always)]
  fn from(x: T) -> Self {
    StateAction::Do(Box::new(x))
  }
}

// #[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
// #[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
// pub struct EffectCanMulligan;

// #[typetag::serde]
// impl Action for EffectCanMulligan {
//   fn apply(&mut self, _game: &mut Game, choice: Option<usize>) ->
// ActionResult {     todo!()
//   }

//   fn apply_two(&mut self, game: &mut Game, interpreter: impl Interpreter) ->
// ActionResult   where
//     Self: Sized,
//   {
//     todo!()
//   }
// }

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub enum StartGame {
  DeclarePlayers(Vec<Player>),
  DiceRoll,
  DeclareMulligans {
    discard_to: usize,
    current_player: PlayerHandle,
    players_declared: Vec<PlayerHandle>,
    players_accepting_hands: Vec<PlayerHandle>,
  },
  DrawHand {
    discard_to: usize,
    current_player: PlayerHandle,
    players_declared: Vec<PlayerHandle>,
    players_accepting_hands: Vec<PlayerHandle>,
  },
}

impl StartGame {
  pub fn new(players: Vec<Player>) -> StartGame {
    StartGame::DeclarePlayers(players)
  }
}

#[typetag::serde]
impl Action for StartGame {
  fn apply(&mut self, game: &mut Game, choice: PromptResult) -> ActionResult {
    use ActionResult::*;
    use StartGame::*;

    match (&mut *self, choice) {
      (DeclarePlayers(ref mut players), _) => {
        game.players.append(players);

        *self = DiceRoll;
        Prompt(ChoicePrompt {
          player_handle: None,
          kind: PromptKind::Shuffle,
          visibility: Visibility::Players(Vec::new()),
          prompt: "Which player will go first?".into(),
          choices: game
            .get_player_handles()
            .map(|x| ChoiceOption::Player(x))
            .collect(),
        })
      }
      (DiceRoll, PromptResult::Shuffled(mut choices)) => {
        if let Some(ChoiceOption::Player(player_handle)) = choices.pop() {
          game.set_active_player(player_handle);
          *self = DeclareMulligans {
            current_player: player_handle,
            discard_to: 6,
            players_declared: Vec::new(),
            players_accepting_hands: Vec::new(),
          };
          Step
        } else {
          todo!()
        }
      }
      (DeclareMulligans { current_player, .. }, PromptResult::None) => Prompt(ChoicePrompt {
        player_handle: Some(*current_player),
        kind: PromptKind::Select,
        visibility: Visibility::Players(Vec::new()),
        prompt: "Would you like to take a mulligan?".into(),
        choices: CHOICE_YES_NO.into(),
      }),
      (
        DeclareMulligans {
          ref discard_to,
          ref current_player,
          ref mut players_declared,
          ref mut players_accepting_hands,
        },
        PromptResult::Selected(choice),
      ) => {
        let mut players_declared = std::mem::take(players_declared);
        let mut players_accepting_hands = std::mem::take(players_accepting_hands);
        if choice == ChoiceOption::Yes {
          players_declared.push(*current_player);
        } else {
          players_accepting_hands.push(*current_player);
        }

        let next_player = game.get_next_player_after(*current_player);
        if next_player != game.active_player {
          *self = DeclareMulligans {
            discard_to: *discard_to,
            current_player: next_player,
            players_declared,
            players_accepting_hands,
          }
        } else {
          *self = DrawHand {
            discard_to: *discard_to,
            current_player: next_player,
            players_declared,
            players_accepting_hands,
          };
        }

        Step
      }
      (
        DrawHand {
          discard_to,
          current_player,
          players_declared,
          players_accepting_hands,
        },
        PromptResult::None,
      ) => {
        let next_player = game.get_next_player_after(*current_player);
        if next_player != game.active_player {
        } else {
          todo!()
        }

        todo!()
        // DiscardHand {}
      }
      // (
      //   DoMulligans {
      //     discard_to,
      //     starting_player,
      //     players_declared,
      //     players_accepting_hands,
      //   },
      //   Some(_),
      // ) => todo!(),
      uhoh => {
        println!("uh oh {:?}", uhoh);
        todo!()
      }
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub enum Visibility {
  AllPlayers,
  Players(Vec<PlayerHandle>),
}

impl Default for Visibility {
  fn default() -> Self {
    Visibility::AllPlayers
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub enum ChoiceOption {
  Player(PlayerHandle),
  Object(ObjectHandle),
  Yes,
  No,
  // Custom(String),
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(transparent)]
pub struct ObjectHandle(usize);

impl ChoiceOption {
  /// Returns `true` if the choice_option is [`Yes`].
  pub fn is_yes(&self) -> bool {
    matches!(self, Self::Yes)
  }

  /// Returns `true` if the choice_option is [`No`].
  pub fn is_no(&self) -> bool {
    matches!(self, Self::No)
  }
}

const CHOICE_YES_NO: [ChoiceOption; 2] = [ChoiceOption::Yes, ChoiceOption::No];

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct ChoicePrompt {
  /// When player_handle is none, it's a system (random) choice.
  pub player_handle: Option<PlayerHandle>,
  pub kind: PromptKind,
  pub prompt: Cow<'static, str>,
  pub choices: Vec<ChoiceOption>,
  pub visibility: Visibility,
}

impl Hash for Box<dyn Choice> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.typetag_name().hash(state);
    let json_repr = serde_json::to_string(&self).unwrap_or("unhashable".to_string());

    json_repr.hash(state)
  }
}

#[derive(PartialEq, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct Prompt {
  choice_prompt: ChoicePrompt,
  receiver: Box<dyn Choice>,
}

impl SerdeDiff for Box<dyn Choice + 'static> {
  fn diff<'a, S: serde::ser::SerializeSeq>(
    &self,
    ctx: &mut serde_diff::DiffContext<'a, S>,
    other: &Self,
  ) -> Result<bool, S::Error> {
    if self != other {
      ctx.save_value(other)?;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  fn apply<'de, A>(
    &mut self,
    seq: &mut A,
    ctx: &mut serde_diff::ApplyContext,
  ) -> Result<bool, <A as serde_diff::_serde::de::SeqAccess<'de>>::Error>
  where
    A: serde_diff::_serde::de::SeqAccess<'de>,
  {
    ctx.read_value(seq, self)
  }
}
