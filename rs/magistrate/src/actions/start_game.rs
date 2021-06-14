use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{action::*, game::*, interface::*};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

const CHOICE_YES_NO: [ChoiceOption; 2] = [ChoiceOption::Yes, ChoiceOption::No];

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
          ..
          // discard_to,
          // current_player,
          // players_declared,
          // players_accepting_hands,
        },
        PromptResult::None,
      ) => {
        todo!()
        // let next_player = game.get_next_player_after(*current_player);
        // if next_player != game.active_player {
        // } else {
        // }
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
