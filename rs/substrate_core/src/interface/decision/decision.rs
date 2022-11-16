use overseer_util::Handle;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::game::PlayerHandle;

#[derive(Clone, Hash, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct DecisionValue {
  pub(crate) serialized: String,
}

impl DecisionValue {
  pub fn get<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
    serde_json::from_str(&self.serialized)
  }
}

#[derive(Clone, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub enum Decision {
  Public {
    question: String,
    value: DecisionValue,
    applied: bool,
  },
  Private {
    question: String,
    server_value: DecisionValue,
    player_values: Vec<DecisionValue>,
    applied: bool,
  },
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_micro_game() {}
}

impl Decision {
  pub fn new<'a, T>(
    question: impl ToString,
    server_value: &T,
    player_values: impl IntoIterator<Item = &'a T>,
  ) -> Self
  where
    T: 'a + Serialize + ?Sized,
  {
    let server_value = serde_json::to_string(server_value).unwrap();
    let player_values = player_values
      .into_iter()
      .map(|x| serde_json::to_string(x).unwrap())
      .collect();

    Self::Private {
      question: question.to_string(),
      server_value,
      player_values,
      applied: false,
    }
  }

  pub fn new_public<T>(question: impl ToString, value: &T) -> Self
  where
    T: Serialize + ?Sized,
  {
    let value = serde_json::to_string(value).unwrap();

    Self::Public {
      question: question.to_string(),
      value,
      applied: false,
    }
  }

  pub fn applied(&self) -> bool {
    match *self {
      Decision::Public { applied, .. } => applied,
      Decision::Private { applied, .. } => applied,
      Decision::Reserved => false,
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
