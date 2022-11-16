// use rand_chacha::
use dialoguer::Select;
use insta::{
  assert_debug_snapshot, assert_display_snapshot, assert_snapshot, assert_yaml_snapshot,
};
use rand::{
  distributions::Uniform,
  prelude::{Distribution, SliceRandom},
  rngs::OsRng,
};
use sandbox::{
  Action, ChoiceOption, ChoicePrompt, Game, GameState, Player, PlayerHandle, PromptKind,
  PromptResult, StartGame, StateAction,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_diff::Diff;

fn serde_clone<T: Serialize + DeserializeOwned>(value: &T) -> T {
  let serialized = rmp_serde::to_vec(value).unwrap();
  let deserialized: T = rmp_serde::from_read_ref(&serialized).unwrap();

  deserialized
}

#[test]
fn foo() {
  let players = vec![Player::default(), Player::default()];
  let mut game_state = GameState::new(vec![StartGame::new(players).into()]);

  println!("Foo!");
  while let Some(state_action) = game_state.stack.pop() {
    match state_action {
      StateAction::Do(action) => step(action, &mut game_state, PromptResult::None),
      StateAction::Prompt(action, mut choice_prompt) => match choice_prompt.kind {
        PromptKind::Select => {
          let choices: Vec<_> = choice_prompt
            .choices
            .iter()
            .map(|x| match x {
              ChoiceOption::Player(handle) => format!("{}", handle.number()),
              ChoiceOption::Yes => "Yes".into(),
              ChoiceOption::No => "No".into(),
              ChoiceOption::Object(_) => todo!(),
            })
            .collect();

          let choice: usize = if let Some(player) = choice_prompt.player_handle {
            Select::new()
              .with_prompt(format!(
                "Player {}, {}",
                player.number(),
                choice_prompt.prompt.clone()
              ))
              .items(&choices)
              .interact()
              .unwrap()
          } else {
            Uniform::from(0..choice_prompt.choices.len()).sample(&mut OsRng)
          };

          let x = choice_prompt.choices.into_iter().nth(choice).unwrap();

          step(action, &mut game_state, PromptResult::Selected(x));
        }
        PromptKind::MultiSelect => todo!(),
        PromptKind::Shuffle => {
          choice_prompt.choices.shuffle(&mut OsRng);

          step(
            action,
            &mut game_state,
            PromptResult::Shuffled(choice_prompt.choices),
          );
        }
        PromptKind::Sort => todo!(),
      },
    }
  }

  assert_eq!(game_state.game.active_player.number(), 1);
}

fn step(mut action: Box<dyn Action>, game_state: &mut GameState, choice: PromptResult) {
  match action.apply(&mut game_state.game, choice) {
    sandbox::ActionResult::Step => {
      game_state.stack.push(StateAction::Do(action));
    }
    sandbox::ActionResult::Prompt(choice_prompt) => game_state
      .stack
      .push(StateAction::Prompt(action, choice_prompt)),
    sandbox::ActionResult::Resolved => drop(game_state.stack.pop()),
  }
}

static START: std::sync::Once = std::sync::Once::new();
fn register() {
  START.call_once(|| {
    <StartGame as Action>::register();
  });
}
