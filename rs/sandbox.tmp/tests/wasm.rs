use playground::{Action, Game, GameState, Player, StartGame, StateAction};

use serde::{de::DeserializeOwned, Serialize};
use serde_diff::Diff;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn foo() {
  register();
  let old: Game = Game::new(vec![Player::default(), Player::default()]);
  let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
  new.pass_to_next_player();

  let diff = Diff::serializable(&old, &new);

  let j = serde_json::to_string(&diff).unwrap();

  assert_eq!(j, "[{\"Enter\":{\"Field\":\"active_player\"}},{\"Enter\":{\"CollectionIndex\":0}},{\"Enter\":{\"FieldIndex\":0}},{\"Value\":1},\"Exit\",\"Exit\"]");
}

fn serde_clone<T: Serialize + DeserializeOwned>(value: &T) -> T {
  let serialized = rmp_serde::to_vec(value).unwrap();
  let deserialized: T = rmp_serde::from_read_ref(&serialized).unwrap();

  deserialized
}

#[wasm_bindgen_test]
fn bar() {
  register();
  let players = vec![Player::default(), Player::default()];
  let game_state = GameState::new(vec![StartGame::new(players).into()]);

  let mut game_state_2 = serde_clone(&game_state);
  game_state_2.step();

  let diff = Diff::serializable(&game_state, &&game_state_2);

  let j = serde_json::to_string(&diff).unwrap();

  assert_eq!(j, "[{\"Enter\":{\"Field\":\"stack\"}},{\"Enter\":{\"CollectionIndex\":0}},{\"Value\":{\"Do\":{\"kind\":\"StartGame\",\"Mulligan\":null}}},{\"Enter\":\"AddToCollection\"},{\"Value\":{\"Prompt\":{\"kind\":\"GoFirstChoice\",\"players\":[0,1]}}},\"Exit\",{\"Enter\":{\"Field\":\"game\"}},{\"Enter\":{\"Field\":\"players\"}},{\"Enter\":\"AddToCollection\"},{\"Value\":{\"hand\":[],\"library\":[],\"deck\":[]}},{\"Enter\":\"AddToCollection\"},{\"Value\":{\"hand\":[],\"library\":[],\"deck\":[]}},\"Exit\",\"Exit\"]");
}

// #[wasm_bindgen_test]
// fn diff_size_flexbuffers() {
//   register();
//   let old: Game = Game::new(vec![Player::default(), Player::default()]);
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let diff = Diff::serializable(&old, &new);

//   let mut s = flexbuffers::FlexbufferSerializer::new();
//   diff.serialize(&mut s).unwrap();

//   assert_eq!(s.view().len(), 321);
// }

// #[wasm_bindgen_test]
// fn diff_size_messagepack() {
//   register();
//   let old: Game = Game::new(vec![Player::default(), Player::default()]);
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let diff = Diff::serializable(&old, &new);

//   let r = rmp_serde::to_vec(&diff).unwrap();

//   assert_eq!(r.len(), 115);
// }

// #[wasm_bindgen_test]
// fn diff_size_bincode() {
//   register();
//   let old: Game = Game::new(vec![Player::default(), Player::default()]);
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let diff = Diff::serializable(&old, &new);

//   let r = bincode::serialize(&diff).unwrap();

//   assert_eq!(r.len(), 249);
// }

// #[wasm_bindgen_test]
// fn serialize_size_flexbuffers() {
//   register();
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let mut s = flexbuffers::FlexbufferSerializer::new();
//   new.serialize(&mut s).unwrap();

//   assert_eq!(s.view().len(), 195);
// }

// #[wasm_bindgen_test]
// fn serialize_size_messagepack() {
//   register();
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let r = rmp_serde::to_vec(&new).unwrap();

//   assert_eq!(r.len(), 63);
// }

// #[wasm_bindgen_test]
// fn serialize_size_bincode() {
//   register();
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let r = bincode::serialize(&new).unwrap();

//   assert_eq!(r.len(), 198);
// }

// #[wasm_bindgen_test]
// fn test_serialize() {
//   register();
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let j = serde_json::to_string(&new).unwrap();

//   assert_eq!(j, "{\"active_player\":1,\"players\":[{\"hand\":[],\"library\":[],\"deck\":[]},{\"hand\":[],\"library\":[],\"deck\":[]}],\"queue\":[{\"Actions\":[{\"kind\":\"Pass\"}]},{\"Actions\":[{\"kind\":\"StartGame\",\"london_mulligan\":true}]}],\"log\":[]}");
// }

// #[wasm_bindgen_test]
// fn test_roundtrip() {
//   register();

//   let mut old: Game = Game::new(vec![Player::default(), Player::default()]);
//   old.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   old
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   old.pass_to_next_player();

//   let stringy = rmp_serde::to_vec(&old).unwrap();

//   let new: Game = rmp_serde::from_read_ref(&stringy).unwrap();

//   assert_eq!(old == new, true);
// }

// #[wasm_bindgen_test]
// fn test_roundtrip_diff() {
//   register();

//   // old is immutable while we generate a diff
//   let old: Game = Game::new(vec![Player::default(), Player::default()]);
//   let mut new: Game = Game::new(vec![Player::default(), Player::default()]);
//   new.state.push(StateAction::BatchedActions(vec![Box::new(Pass)]));
//   new
//     .state
//     .push(StateAction::BatchedActions(vec![Box::new(StartGame {
//       london_mulligan: true,
//     })]));
//   new.pass_to_next_player();

//   let diff = Diff::serializable(&old, &new);

//   let diff_data = rmp_serde::to_vec_named(&diff).unwrap();

//   let mut deserializer = rmp_serde::Deserializer::new(diff_data.as_slice());

//   // apply the diff:
//   let mut old = old;
//   serde_diff::Apply::apply(&mut deserializer, &mut old).unwrap();

//   assert_eq!(old == new, true);
// }

// #[wasm_bindgen_test]
// fn test_typetag_name() {
//   register();

//   let foo: Box<dyn Action> = Box::new(Pass);

//   assert_eq!(foo.typetag_name(), "Pass");
// }

static START: std::sync::Once = std::sync::Once::new();
fn register() {
  START.call_once(|| {
    <StartGame as Action>::register();
  });
}
