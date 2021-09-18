use overseer_util::{make_handle, Handle};
use serde::{Deserialize, Serialize};

use crate::game::PlayerHandle;

#[derive(Clone, Hash, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct DecisionList {
  pub entries: Vec<Option<Decision>>,
  reserved: usize,
}

make_handle!(DecisionHandle, usize);

impl DecisionList {
  pub fn new() -> Self {
    DecisionList {
      entries: Vec::new(),
      reserved: 0,
    }
  }

  pub fn reserve(&mut self) -> DecisionHandle {
    let reserved = self.reserved;
    self.reserved += 1;
    DecisionHandle::from_index(reserved)
  }

  #[cfg(test)]
  pub(crate) fn reset_reserved(&mut self) {
    self.reserved = 0;
  }

  pub(crate) fn contains(&self, decision: &DecisionHandle) -> bool {
    self.entries.len() > decision.to_index()
  }

  pub(crate) fn get(&self, decision: &DecisionHandle) -> Option<&Decision> {
    self
      .entries
      .get(decision.to_index())
      .and_then(Option::as_ref)
  }

  pub(crate) fn get_mut(&mut self, decision: &DecisionHandle) -> Option<&mut Decision> {
    self
      .entries
      .get_mut(decision.to_index())
      .and_then(Option::as_mut)
  }

  pub(crate) fn set_decision<'a, T: 'a + Serialize + ?Sized>(
    &mut self,
    decision: &DecisionHandle,
    server_value: &T,
    player_values: impl IntoIterator<Item = &'a T>,
  ) -> &mut Decision {
    let index = decision.to_index();
    self.entries.resize_with(index + 1, Default::default);
    self.entries[index] = Some(Decision::new(server_value, player_values));
    self.entries[index].as_mut().unwrap()
  }

  pub(crate) fn set_decision_public<'a, T: Serialize + ?Sized>(
    &mut self,
    decision: &DecisionHandle,
    value: &T,
  ) -> &mut Decision {
    let index = decision.to_index();
    self.entries.resize_with(index + 1, Default::default);
    self.entries[index] = Some(Decision::new_public(value));
    self.entries[index].as_mut().unwrap()
  }
}

#[derive(Clone, Hash, Debug)]
#[derive(Serialize, Deserialize/* , SerdeDiff */)]
pub enum Decision {
  Public {
    value: String,
    applied: bool,
  },
  Private {
    server_value: String,
    player_values: Vec<String>,
    applied: bool,
  },
}

impl Decision {
  pub fn new<'a, T>(server_value: &T, player_values: impl IntoIterator<Item = &'a T>) -> Self
  where
    T: 'a + Serialize + ?Sized,
  {
    let server_value = serde_json::to_string(server_value).unwrap();
    let player_values = player_values
      .into_iter()
      .map(|x| serde_json::to_string(x).unwrap())
      .collect();

    Self::Private {
      server_value,
      player_values,
      applied: false,
    }
  }

  pub fn new_public<T>(value: &T) -> Self
  where
    T: Serialize + ?Sized,
  {
    let value = serde_json::to_string(value).unwrap();

    Self::Public {
      value,
      applied: false,
    }
  }

  pub fn applied(&self) -> bool {
    match *self {
      Decision::Public { applied, .. } => applied,
      Decision::Private { applied, .. } => applied,
    }
  }

  pub fn set_applied(&mut self) {
    match self {
      Decision::Public { applied, .. } => *applied = true,
      Decision::Private { applied, .. } => *applied = true,
    }
  }

  pub(self) fn get_server_value(&self) -> &String {
    match self {
      Decision::Public { value, .. } => value,
      Decision::Private { server_value, .. } => server_value,
    }
  }

  pub(self) fn get_player_value(&self, player: &PlayerHandle) -> &String {
    match self {
      Decision::Public { value, .. } => value,
      Decision::Private { player_values, .. } => player_values.get(player.to_index()).unwrap(),
    }
  }

  pub fn get_server_decision<'a, T: Deserialize<'a>>(&'a self) -> Result<T, serde_json::Error> {
    let value = self.get_server_value();

    serde_json::from_str(value)
  }

  pub fn get_player_decision<'a, T: Deserialize<'a>>(
    &'a self,
    player: &PlayerHandle,
  ) -> Result<T, serde_json::Error> {
    let value = self.get_player_value(player);

    serde_json::from_str(&value)
  }
}

#[cfg(test)]
mod tests {
  use im::hashset;
  use insta::{assert_debug_snapshot, assert_yaml_snapshot};

  use super::*;

  #[derive(Debug, Serialize, Deserialize)]
  struct SimpleDecision {
    a: usize,
    b: String,
  }

  fn make_decision(value: usize) -> SimpleDecision {
    SimpleDecision {
      a: value,
      b: value.to_string(),
    }
  }

  #[test]
  fn can_create() {
    let mut list = DecisionList::new();

    let handle_one = list.reserve();
    let handle_two = list.reserve();

    list.set_decision(&handle_one, &make_decision(1), [&make_decision(2)]);
    list.set_decision_public(&handle_two, &make_decision(3));

    assert_yaml_snapshot!(list, @r###"
    ---
    entries:
      - Private:
          server_value: "{\"a\":1,\"b\":\"1\"}"
          player_values:
            - "{\"a\":2,\"b\":\"2\"}"
          applied: false
      - Public:
          value: "{\"a\":3,\"b\":\"3\"}"
          applied: false
    reserved: 2
    "###);
  }

  #[test]
  fn can_restore() {
    let string = r#"
---
entries:
  - Private:
      server_value: "{\"a\":1,\"b\":\"1\"}"
      player_values:
        - "{\"a\":2,\"b\":\"2\"}"
      applied: true
  - Public:
      value: "{\"a\":3,\"b\":\"3\"}"
      applied: false
reserved: 2
"#;

    let mut list: DecisionList = serde_yaml::from_str(string).unwrap();
    list.reset_reserved();

    let handle_one = list.reserve();
    let handle_two = list.reserve();
    let handle_three = list.reserve();

    let handles = hashset![&handle_one, &handle_two, &handle_three];
    assert_eq!(handles.len(), 3);

    let get_decision_pair = |handle: &DecisionHandle| {
      (
        list
          .get(handle)
          .unwrap()
          .get_server_decision::<SimpleDecision>()
          .unwrap(),
        list
          .get(handle)
          .unwrap()
          .get_player_decision::<SimpleDecision>(&PlayerHandle::from_index(0))
          .unwrap(),
      )
    };

    assert_debug_snapshot!(get_decision_pair(&handle_one), @r###"
    (
        SimpleDecision {
            a: 1,
            b: "1",
        },
        SimpleDecision {
            a: 2,
            b: "2",
        },
    )
    "###
    );

    assert_debug_snapshot!(get_decision_pair(&handle_two), @r###"
    (
        SimpleDecision {
            a: 3,
            b: "3",
        },
        SimpleDecision {
            a: 3,
            b: "3",
        },
    )
    "###
    );

    assert_debug_snapshot!(list.get(&handle_three), @"None");
  }
}
