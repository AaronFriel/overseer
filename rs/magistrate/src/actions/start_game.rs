use dyn_partial_eq::DynPartialEq;
use overseer_substrate::{action::*, game::*, interface::*};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use crate::actions::{Draw, ShuffleDeckIntoLibrary, ShuffleHandIntoLibrary};

const CHOICE_YES_NO: [ChoiceOption; 2] = [ChoiceOption::Yes, ChoiceOption::No];

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(DynPartialEq, Serialize, Deserialize, SerdeDiff)]
pub enum StartGame {
  DeclarePlayers(Vec<Player>),
  DiceRoll,
  DrawHands {
    discard_to: usize,
    players_drawing: Vec<PlayerHandle>,
  },
  DeclareMulligans {
    discard_to: usize,
    current_player: PlayerHandle,
    players_declaring_mulligans: Vec<PlayerHandle>,
  },
  TakeMulligansShuffle {
    discard_to: usize,
    players_declared: Vec<PlayerHandle>,
  },
  TakeMulligansDiscard {
    discard_to: usize,
    current_player: PlayerHandle,
    players_declared: Vec<PlayerHandle>,
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
        println!("Adding players");
        game.players.append(players);

        let player_handles = game.get_player_handles();

        let shuffle_actions: ActionList = player_handles
          .map(|player| (ShuffleDeckIntoLibrary { player }).into())
          .collect();

        *self = DiceRoll;
        DoMulti(shuffle_actions)
      }
      (DiceRoll, PromptResult::None) => {
        let choices = game
          .get_player_handles()
          .map(|x| ChoiceOption::Player(x))
          .collect();

        Prompt(ChoicePrompt {
          player_handle: None,
          kind: PromptKind::Shuffle,
          visibility: Visibility::Players(Vec::new()),
          prompt: "Which player will go first?".into(),
          choices,
        })
      }
      (DiceRoll, PromptResult::Ordered(rolls)) => {
        let choices: Vec<_> = game
          .get_player_handles()
          .map(|x| ChoiceOption::Player(x))
          .collect();

        let choice = rolls.get(0).and_then(|x| choices.get(*x));

        if let Some(ChoiceOption::Player(player_handle)) = choice {
          game.set_active_player(*player_handle);
          *self = DrawHands {
            discard_to: 6,
            players_drawing: game.get_player_handles().collect(),
          };
          Step
        } else {
          todo!()
        }
      }
      (
        DrawHands {
          ref discard_to,
          ref mut players_drawing,
        },
        PromptResult::None,
      ) => {
        if let Some(player) = players_drawing.first() {
          let draw_actions: ActionList = players_drawing
            .iter()
            .map(|player| {
              (Draw {
                player: *player,
                count: 7,
              })
              .into()
            })
            .collect();

          *self = DeclareMulligans {
            discard_to: *discard_to,
            current_player: *player,
            players_declaring_mulligans: std::mem::take(players_drawing),
          };
          DoMulti(draw_actions)
        } else {
          Resolved
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
          current_player,
          discard_to,
          players_declaring_mulligans,
        },
        PromptResult::Selected(choice),
      ) => {
        let next_player = players_declaring_mulligans
          .iter()
          .skip_while(|x| **x != *current_player)
          .skip(1)
          .cloned()
          .next();

        if choice == 1 {
          players_declaring_mulligans.retain(|x| x != current_player);
        }

        if let Some(next_player) = next_player {
          *self = DeclareMulligans {
            discard_to: *discard_to,
            current_player: next_player,
            players_declaring_mulligans: std::mem::take(players_declaring_mulligans),
          };

          Step
        } else if players_declaring_mulligans.len() == 0 {
          Resolved
        } else {
          *self = TakeMulligansShuffle {
            discard_to: *discard_to,
            players_declared: std::mem::take(players_declaring_mulligans),
          };

          Step
        }
      }
      (
        TakeMulligansShuffle {
          discard_to,
          players_declared,
        },
        _,
      ) => {
        let actions: ActionList = players_declared
          .iter()
          // Remember: LIFO order
          .map(|player| {
            vec![
              Draw {
                player: *player,
                count: 7,
              }
              .into(),
              ShuffleHandIntoLibrary { player: *player }.into(),
            ]
          })
          .flatten()
          .collect();

        *self = TakeMulligansDiscard {
          discard_to: *discard_to,
          current_player: *players_declared.first().unwrap(),
          players_declared: std::mem::take(players_declared),
        };

        DoMulti(actions)
      }
      (
        TakeMulligansDiscard {
          ref discard_to,
          ref current_player,
          ref mut players_declared,
        },
        PromptResult::None,
      ) => {
        let player = game.get_player_mut(*current_player);
        let next_player = players_declared
          .iter()
          .skip_while(|x| **x != *current_player)
          .skip(1)
          .cloned()
          .next();

        if player.hand.cards.len() > *discard_to {
          let choices: Vec<ChoiceOption> = player
            .hand
            .cards
            .iter()
            .map(|x| ChoiceOption::Custom(x.name.to_string()))
            .collect();

          Prompt(ChoicePrompt {
            player_handle: Some(*current_player),
            kind: PromptKind::Select,
            // TODO: Incorrect, forging ahead
            visibility: Visibility::AllPlayers,
            choices,
            prompt: "Choose a card to place on the bottom of your library".into(),
          })
        } else if let Some(next_player) = next_player {
          *self = TakeMulligansDiscard {
            discard_to: *discard_to,
            current_player: next_player,
            players_declared: std::mem::take(players_declared),
          };

          Step
        } else if *discard_to > 0 {
          // Return to declaring mulligans
          *self = DeclareMulligans {
            discard_to: *discard_to - 1,
            current_player: *current_player,
            players_declaring_mulligans: std::mem::take(players_declared),
          };

          Step
        } else {
          Resolved
        }
      }
      (TakeMulligansDiscard { current_player, .. }, PromptResult::Selected(card_idx)) => {
        let player = game.get_player_mut(*current_player);

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

pub enum TakeMulligan {
  ShuffleLibrary {
    player: PlayerHandle,
    discard_to: usize,
  },
  DiscardCard {
    player: PlayerHandle,
    discard_to: usize,
  },
}
