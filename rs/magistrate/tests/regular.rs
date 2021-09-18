use std::iter;

use dialoguer::{theme::ColorfulTheme, Select};
use insta::assert_yaml_snapshot;
use overseer_magistrate::{
  actions::StartGame,
  cards::{METALLIC_SLIVER, WASTES},
};
use overseer_substrate::{
  action::{ActionErr, ActionResult, PromptKind, PromptResult, SimpleAction},
  game::{Game, Mode, ObjectPool, Player, RegisteredCard, State, StateAction},
  interface::{ChoiceOption, DecisionList, InterfaceError, InterfaceResult, UserInterface, YesNo},
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
  let deck: Vec<RegisteredCard> = iter::repeat(RegisteredCard::from(WASTES))
    .take(17)
    .chain(iter::repeat(RegisteredCard::from(METALLIC_SLIVER)).take(23))
    .collect();

  let players = vec![
    Player::new("Friel", deck.clone(), vec![]),
    Player::new("Trevor", deck.clone(), vec![]),
  ];

  let state = State::new(vec![WASTES.clone(), METALLIC_SLIVER.clone()], players);

  let mut game = Game::new(
    state,
    Mode::Local,
    ObjectPool::new(),
    DecisionList::new(),
    Vec::new(),
    RecordingInterface::new(TestInterface::new()),
  );

  while let Some(state_action) = game.actions.pop() {
    println!("Handling state action {:?}", state_action);
    step(state_action, &mut game);
  }

  assert_yaml_snapshot!(game.state(), {
    ".cards" => "$cards",
    ".players[].deck" => "$deck",
    ".players[].library.cards" => "$library",
    ".players[].hand.cards" => "$library",
  }, @r###"
  ---
  cards: $cards
  players:
    - name: Friel
      handle: ~
      controller: ~
      deck: $deck
      sideboard: []
      library:
        objects: []
        count: 0
      hand:
        objects: []
        count: 0
      graveyard:
        objects: []
        count: 0
      revealed: []
      life: 20
      has_left_game: false
      has_lost_game: false
    - name: Trevor
      handle: ~
      controller: ~
      deck: $deck
      sideboard: []
      library:
        objects: []
        count: 0
      hand:
        objects: []
        count: 0
      graveyard:
        objects: []
        count: 0
      revealed: []
      life: 20
      has_left_game: false
      has_lost_game: false
  active_player: 1
  current_player: ~
  battlefield:
    objects: []
    count: 0
  stack:
    objects: []
    count: 0
  exile:
    objects: []
    count: 0
  command:
    objects: []
    count: 0
  "###);
}

fn step(mut action: Box<dyn SimpleAction>, mut game_state: &mut Game) {
  println!("Stepping {:?}", action);
  use ActionErr::*;
  match action.perform(&mut game_state) {
    Ok(_) => {
      // By default actions are popped off the action stack.
    }
    Err(Step) => {
      game_state.actions.push(action);
    }
    Err(Waiting) => todo!(),
  }
}

struct RecordingInterface {
  test_interface: TestInterface,
  cli_interface: CliInterface,
}

impl RecordingInterface {
  fn new(test_interface: TestInterface) -> Self {
    Self {
      test_interface,
      cli_interface: CliInterface,
    }
  }
}

impl UserInterface for RecordingInterface {
  fn prompt_yes_no(
    &mut self,
    player: overseer_substrate::game::PlayerHandle,
  ) -> InterfaceResult<YesNo> {
    match self.test_interface.prompt_yes_no(player) {
      Ok(value) => Ok(value),
      Err(_) => {
        let value = self.cli_interface.prompt_yes_no(player).unwrap();
        self.test_interface.yes_nos.push(value);
        Ok(value)
      }
    }
  }
}

#[derive(Serialize, Deserialize)]
struct TestInterface {
  pub yes_no_idx: usize,
  pub yes_nos: Vec<YesNo>,
}

impl TestInterface {
  fn new() -> Self {
    Self {
      yes_no_idx: 0,
      yes_nos: Vec::new(),
    }
  }
}

impl UserInterface for TestInterface {
  fn prompt_yes_no(
    &mut self,
    _player: overseer_substrate::game::PlayerHandle,
  ) -> InterfaceResult<YesNo> {
    self
      .yes_nos
      .get(self.yes_no_idx)
      .copied()
      .ok_or(InterfaceError::Waiting)
  }
}

struct CliInterface;

impl UserInterface for CliInterface {
  fn prompt_yes_no(
    &mut self,
    player: overseer_substrate::game::PlayerHandle,
  ) -> InterfaceResult<YesNo> {
    const YES: &str = "Yes";
    const NO: &str = "No";
    const ITEMS: [&str; 2] = [YES, NO];

    let result = Select::with_theme(&ColorfulTheme::default())
      .items(&ITEMS)
      .interact()
      .unwrap();

    if ITEMS[result] == YES {
      Ok(YesNo::Yes)
    } else {
      Ok(YesNo::No)
    }
  }
}
