use overseer_util::{make_handle, Handle};
use serde::{Deserialize, Serialize};

use super::Decision;

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

  pub fn from_entries(entries: Vec<Option<Decision>>) -> Self {
    DecisionList {
      entries,
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

  pub(crate) fn contains(&self, decision: DecisionHandle) -> bool {
    self.entries.len() > decision.to_index() && self.get(decision).is_some()
  }

  #[allow(dead_code)]
  pub(crate) fn get(&self, decision: DecisionHandle) -> Option<&Decision> {
    self
      .entries
      .get(decision.to_index())
      .and_then(Option::as_ref)
  }

  pub(crate) fn get_mut(&mut self, decision: DecisionHandle) -> Option<&mut Decision> {
    self
      .entries
      .get_mut(decision.to_index())
      .and_then(Option::as_mut)
  }

  pub(crate) fn set_decision<'a, T: 'a + Serialize + ?Sized>(
    &mut self,
    decision: DecisionHandle,
    question: impl ToString,
    server_value: &T,
    player_values: impl IntoIterator<Item = &'a T>,
  ) -> &mut Decision {
    let index = decision.to_index();
    self.entries.resize_with(index + 1, Default::default);
    self.entries[index] = Some(Decision::new(question, server_value, player_values));
    self.entries[index].as_mut().unwrap()
  }

  pub(crate) fn set_decision_public<'a, T: Serialize + ?Sized>(
    &mut self,
    decision: DecisionHandle,
    question: impl ToString,
    value: &T,
  ) -> &mut Decision {
    let index = decision.to_index();
    self.entries.resize_with(index + 1, Default::default);
    self.entries[index] = Some(Decision::new_public(question, value));
    self.entries[index].as_mut().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use im::hashset;
  use insta::{assert_debug_snapshot, assert_yaml_snapshot};

  use super::*;
  use crate::game::PlayerHandle;

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

    list.set_decision(handle_one, "Test one", &make_decision(1), [&make_decision(
      2,
    )]);
    list.set_decision_public(handle_two, "Test two", &make_decision(3));

    assert_yaml_snapshot!(list, @r###"
    ---
    entries:
      - Private:
          question: Test one
          server_value: "{\"a\":1,\"b\":\"1\"}"
          player_values:
            - "{\"a\":2,\"b\":\"2\"}"
          applied: false
      - Public:
          question: Test two
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
      question: Test one
      server_value: "{\"a\":1,\"b\":\"1\"}"
      player_values:
        - "{\"a\":2,\"b\":\"2\"}"
      applied: true
  - Public:
      question: Test two
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

    let get_decision_pair = |handle: DecisionHandle| {
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

    assert_debug_snapshot!(get_decision_pair(handle_one), @r###"
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

    assert_debug_snapshot!(get_decision_pair(handle_two), @r###"
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

    assert_debug_snapshot!(list.get(handle_three), @"None");
  }
}
