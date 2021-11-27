use std::{cell::RefCell, iter, rc::Rc};

use dialoguer::{theme::ColorfulTheme, Select};
use insta::{assert_ron_snapshot, assert_yaml_snapshot};
use overseer_magistrate::{
  actions::StartGame,
  cards::{METALLIC_SLIVER, WASTES},
};
use overseer_substrate::{
  action::{ActionErr, SimpleAction},
  game::{ClientState, Game, Mode, ObjectPool, Player, PlayerHandle, RegisteredCard, ServerState},
  interface::{DecisionList, InterfaceError, InterfaceResult, Target, UserInterface, YesNo},
};
use serde::{Deserialize, Serialize};

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
    Player::new("Kyle", deck.clone(), vec![]),
    Player::new("Trevor", deck.clone(), vec![]),
    Player::new("Karl", deck.clone(), vec![]),
  ];

  let client_state = ClientState::new(vec![WASTES.clone(), METALLIC_SLIVER.clone()], players);

  let decisions: DecisionList = serde_yaml::from_str(
    r###"
entries:
- Public:
    question: Who wins the die roll?
    value: "4"
    applied: false
- Public:
    question: "Player 4, Karl, you've won the die roll, do you want to go first?"
    value: "\"No\""
    applied: false
- Public:
    question: "Player 1, Friel, you've won the die roll, do you want to go first?"
    value: "\"No\""
    applied: false
- Public:
    question: "Player 2, Kyle, you've won the die roll, do you want to go first?"
    value: "\"Yes\""
    applied: false
reserved: 0
  "###,
  )
  .unwrap();

  let server_state = ServerState::new(ObjectPool::new(), decisions, Vec::new());

  let test_interface = Rc::new(RefCell::new(TestInterface::new()));
  let mut game = Game::new(
    client_state,
    server_state,
    Mode::Local,
    // DecisionList::from_entries(vec![Some(Decision::Public {
    //   question: "Who wins the die roll?".to_string(),
    //   value: "1".to_string(),
    //   applied: false,
    // })]),
    RecordingInterface::new(test_interface.clone()),
  );

  let action: Box<dyn SimpleAction> = Box::new(StartGame::new(&mut game));

  game.server.actions.push(action);

  while let Some(state_action) = game.server.actions.pop() {
    println!("Handling state action {:?}", state_action);
    step(state_action, &mut game);
  }

  assert_yaml_snapshot!(game.server.decisions, @r###"
  ---
  entries:
    - Public:
        question: Who wins the die roll?
        value: "4"
        applied: true
    - Public:
        question: "Player 4, Karl, you've won the die roll, do you want to go first?"
        value: "\"No\""
        applied: true
    - Public:
        question: "Player 1, Friel, you've won the die roll, do you want to go first?"
        value: "\"No\""
        applied: true
    - Public:
        question: "Player 2, Kyle, you've won the die roll, do you want to go first?"
        value: "\"Yes\""
        applied: true
  reserved: 4
  "###);

  assert_ron_snapshot!(game.state(), {
    ".cards" => "$cards",
    ".players[].deck" => "$deck",
    ".players[].library.cards" => "$library",
    ".players[].hand.cards" => "$library",
  }, @r###"
  ClientState(
    cards: "$cards",
    players: [
      Player(
        name: "Friel",
        handle: None,
        controller: None,
        deck: "$deck",
        sideboard: [],
        library: Zone(
          objects: [],
          count: 0,
        ),
        hand: Zone(
          objects: [],
          count: 0,
        ),
        graveyard: Zone(
          objects: [],
          count: 0,
        ),
        revealed: [],
        life: 20,
        has_left_game: false,
        has_lost_game: false,
      ),
      Player(
        name: "Kyle",
        handle: None,
        controller: None,
        deck: "$deck",
        sideboard: [],
        library: Zone(
          objects: [],
          count: 0,
        ),
        hand: Zone(
          objects: [],
          count: 0,
        ),
        graveyard: Zone(
          objects: [],
          count: 0,
        ),
        revealed: [],
        life: 20,
        has_left_game: false,
        has_lost_game: false,
      ),
      Player(
        name: "Trevor",
        handle: None,
        controller: None,
        deck: "$deck",
        sideboard: [],
        library: Zone(
          objects: [],
          count: 0,
        ),
        hand: Zone(
          objects: [],
          count: 0,
        ),
        graveyard: Zone(
          objects: [],
          count: 0,
        ),
        revealed: [],
        life: 20,
        has_left_game: false,
        has_lost_game: false,
      ),
      Player(
        name: "Karl",
        handle: None,
        controller: None,
        deck: "$deck",
        sideboard: [],
        library: Zone(
          objects: [],
          count: 0,
        ),
        hand: Zone(
          objects: [],
          count: 0,
        ),
        graveyard: Zone(
          objects: [],
          count: 0,
        ),
        revealed: [],
        life: 20,
        has_left_game: false,
        has_lost_game: false,
      ),
    ],
    active_player: Some(PlayerHandle(2)),
    current_player: None,
    objects: {},
    battlefield: Zone(
      objects: [],
      count: 0,
    ),
    stack: Zone(
      objects: [],
      count: 0,
    ),
    exile: Zone(
      objects: [],
      count: 0,
    ),
    command: Zone(
      objects: [],
      count: 0,
    ),
  )
  "###);
}

fn step(mut action: Box<dyn SimpleAction>, mut game: &mut Game) {
  println!("Stepping {:?}", action);
  use ActionErr::*;
  match action.perform(&mut game) {
    Ok(()) => {}
    Err(Step) => {
      game.server.actions.push(action);
    }
    Err(Waiting) => todo!(),
  }
}

struct RecordingInterface {
  test_interface: Rc<RefCell<TestInterface>>,
  cli_interface: CliInterface,
}

impl RecordingInterface {
  fn new(test_interface: Rc<RefCell<TestInterface>>) -> Self {
    Self {
      test_interface,
      cli_interface: CliInterface,
    }
  }
}

impl UserInterface for RecordingInterface {
  fn prompt_yes_no(
    &mut self,
    state: &ClientState,
    player: overseer_substrate::game::PlayerHandle,
    prompt: &str,
  ) -> InterfaceResult<YesNo> {
    let mut test_interface = self.test_interface.borrow_mut();

    match test_interface.prompt_yes_no(state, player, prompt) {
      Ok(value) => Ok(value),
      Err(_) => {
        let value = self.cli_interface.prompt_yes_no(state, player, prompt)?;
        test_interface.push_yes_no(value);
        Ok(value)
      }
    }
  }

  fn prompt_target_select(
    &mut self,
    state: &ClientState,
    objects: &ObjectPool,
    player: overseer_substrate::game::PlayerHandle,
    targets: &[Target],
  ) -> InterfaceResult<Target> {
    let mut test_interface = self.test_interface.borrow_mut();
    match test_interface.prompt_target_select(state, objects, player, targets) {
      Ok(value) => Ok(value),
      Err(_) => {
        let value = self
          .cli_interface
          .prompt_target_select(state, objects, player, targets)
          .unwrap();
        test_interface.push_target_selection(value.clone());
        Ok(value)
      }
    }
  }
}

#[derive(Serialize, Deserialize)]
struct TestInterface {
  yes_no_idx: usize,
  yes_nos: Vec<YesNo>,
  target_selection_idx: usize,
  target_selections: Vec<Target>,
}

impl TestInterface {
  fn new() -> Self {
    Self {
      yes_no_idx: 0,
      yes_nos: Vec::new(),
      target_selection_idx: 0,
      target_selections: Vec::new(),
    }
  }

  pub fn get_yes_no(&self) -> &YesNo {
    &self.yes_nos[self.yes_no_idx]
  }

  pub fn push_yes_no(&mut self, value: YesNo) {
    self.yes_nos.push(value);
    self.yes_no_idx = self.yes_no_idx + 1;
  }

  pub fn get_target_selection(&self) -> &Target {
    &self.target_selections[self.target_selection_idx]
  }

  pub fn push_target_selection(&mut self, value: Target) {
    self.target_selections.push(value);
    self.target_selection_idx = self.target_selection_idx + 1;
  }
}

impl UserInterface for TestInterface {
  fn prompt_yes_no(
    &mut self,
    _state: &ClientState,
    _player: overseer_substrate::game::PlayerHandle,
    _prompt: &str,
  ) -> InterfaceResult<YesNo> {
    self
      .yes_nos
      .get(self.yes_no_idx)
      .copied()
      .ok_or(InterfaceError::Waiting)
  }

  fn prompt_target_select(
    &mut self,
    _state: &ClientState,
    _objects: &ObjectPool,
    _player: overseer_substrate::game::PlayerHandle,
    _choices: &[Target],
  ) -> InterfaceResult<Target> {
    self
      .target_selections
      .get(self.target_selection_idx)
      .cloned()
      .ok_or(InterfaceError::Waiting)
  }
}

struct CliInterface;

impl UserInterface for CliInterface {
  fn prompt_yes_no(
    &mut self,
    _state: &ClientState,
    _player: PlayerHandle,
    prompt: &str,
  ) -> InterfaceResult<YesNo> {
    const YES: &str = "Yes";
    const NO: &str = "No";
    const ITEMS: [&str; 2] = [YES, NO];

    let result = Select::with_theme(&ColorfulTheme::default())
      .with_prompt(prompt)
      .items(&ITEMS)
      .interact()
      .unwrap();

    if ITEMS[result] == YES {
      Ok(YesNo::Yes)
    } else {
      Ok(YesNo::No)
    }
  }

  fn prompt_target_select(
    &mut self,
    state: &ClientState,
    objects: &ObjectPool,
    _player: overseer_substrate::game::PlayerHandle,
    targets: &[Target],
  ) -> InterfaceResult<Target> {
    let options: InterfaceResult<Vec<String>> = targets
      .iter()
      .map(|o| match o {
        Target::Player(player_handle) => {
          let player = state.get_player(*player_handle);
          Ok(format!("{}", player.name))
        }
        Target::Object(object_handle) => {
          let object = objects.get(object_handle);
          match object {
            Some(_) => todo!(),
            None => Err(InterfaceError::Waiting),
          }
        }
      })
      .collect();
    let options = options?;

    let result = Select::with_theme(&ColorfulTheme::default())
      .items(&options)
      .interact()
      .unwrap();
    Ok(targets[result].clone())
  }
}
