use std::iter;

use dialoguer::{theme::ColorfulTheme, Select};
use insta::assert_yaml_snapshot;
use overseer_magistrate::{
  actions::StartGame,
  cards::{METALLIC_SLIVER, WASTES},
};
use overseer_substrate::{
  action::{Action, ActionResult, PromptKind, PromptResult},
  game::{Game, Player, StateAction},
  interface::ChoiceOption,
};
use rand::{prelude::SliceRandom, rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

/*
use serde::de::DeserializeOwned;
fn serde_clone<T: Serialize + DeserializeOwned>(value: &T) -> T {
  let serialized = rmp_serde::to_vec(value).unwrap();
  let deserialized: T = rmp_serde::from_read_ref(&serialized).unwrap();

  deserialized
}
*/

#[test]
fn foo() {
  let game = Game::new(vec![WASTES, METALLIC_SLIVER], vec![]);

  let deck: Vec<_> = iter::repeat(WASTES)
    .take(17)
    .chain(iter::repeat(METALLIC_SLIVER).take(23))
    .collect();

  let players = vec![
    Player::new("Friel", deck.clone(), vec![]),
    Player::new("Trevor", deck.clone(), vec![]),
  ];
  let mut game_state = GameState {
    game,
    log: vec![],
    stack: vec![StateAction::Do(StartGame::DeclarePlayers(players).into())],
  };

  while let Some(state_action) = game_state.stack.pop() {
    println!("Handling state action {:?}", state_action);
    match state_action {
      StateAction::Do(action) => step(action, &mut game_state, PromptResult::None),
      StateAction::Prompt(action, choice_prompt) => match choice_prompt.kind {
        PromptKind::Select => {
          let choices: Vec<_> = choice_prompt
            .choices
            .iter()
            .map(|x| match x {
              ChoiceOption::Player(handle) => format!("{}", handle.number()),
              ChoiceOption::Yes => "Yes".into(),
              ChoiceOption::No => "No".into(),
              ChoiceOption::Object(_) => todo!(),
              ChoiceOption::Custom(string) => string.clone(),
            })
            .collect();

          let choice: usize = if let Some(player) = choice_prompt.player_handle {
            Select::with_theme(&ColorfulTheme::default())
              .with_prompt(format!(
                "Player {}, {}",
                player.number(),
                choice_prompt.prompt.clone()
              ))
              .items(&choices)
              .interact()
              .unwrap()
          } else {
            OsRng.gen_range(0..choice_prompt.choices.len())
          };

          step(action, &mut game_state, PromptResult::Selected(choice));
        }
        PromptKind::MultiSelect => todo!(),
        PromptKind::Shuffle => {
          let mut result = (0..choice_prompt.choices.len()).collect::<Vec<_>>();

          result.shuffle(&mut OsRng);

          step(action, &mut game_state, PromptResult::Ordered(result));
        }
        PromptKind::Sort => todo!(),
      },
    }
  }

  assert_yaml_snapshot!(&game_state, {
    ".game.cards" => "$cards",
    ".game.players[].deck" => "$deck",
    ".game.players[].library.cards" => "$library",
    ".game.players[].hand.cards" => "$library",
  }, @r###"
  ---
  stack: []
  log: []
  game:
    cards: $cards
    players:
      - name: Friel
        deck: $deck
        sideboard: []
        library:
          cards: $library
          kind: Library
        hand:
          cards: $library
          kind: Graveyard
        graveyard:
          cards: []
          kind: Graveyard
        life: 20
        has_left_game: false
        has_lost_game: false
      - name: Trevor
        deck: $deck
        sideboard: []
        library:
          cards: $library
          kind: Library
        hand:
          cards: $library
          kind: Graveyard
        graveyard:
          cards: []
          kind: Graveyard
        life: 20
        has_left_game: false
        has_lost_game: false
    active_player: 0
    log: []
  "###);
}

#[derive(PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct GameState {
  pub stack: Vec<StateAction>,
  pub log: Vec<StateAction>,
  pub game: Game,
}

fn step(mut action: Box<dyn Action>, game_state: &mut GameState, choice: PromptResult) {
  println!("Stepping {:?}, choice: {:?}", action, choice);

  match action.apply(&mut game_state.game, choice) {
    ActionResult::Step => {
      game_state.stack.push(StateAction::Do(action));
    }
    ActionResult::Prompt(choice_prompt) => game_state
      .stack
      .push(StateAction::Prompt(action, choice_prompt)),
    ActionResult::Resolved => {
      // By default actions are popped off the action stack.
    }
    ActionResult::Do(next_action) => {
      println!("Result: Adding 1 actions to do as well");
      game_state.stack.push(StateAction::Do(action));
      game_state.stack.push(StateAction::Do(next_action));
    }
    ActionResult::DoMulti(next_actions) => {
      println!(
        "Result: Adding {} actions to do as well",
        next_actions.len()
      );
      game_state.stack.push(StateAction::Do(action));
      game_state
        .stack
        .extend(next_actions.into_iter().map(StateAction::Do))
    }
  }
}
