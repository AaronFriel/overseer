use serde::{Deserialize, Serialize};

use crate::{action::SimpleAction, game::ObjectPool, interface::DecisionList};

#[derive(Clone, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ServerState {
  pub objects: ObjectPool,
  pub decisions: DecisionList,
  pub actions: Vec<Box<dyn SimpleAction>>,
}

impl ServerState {
  pub fn new(
    objects: ObjectPool,
    decisions: DecisionList,
    actions: Vec<Box<dyn SimpleAction>>,
  ) -> Self {
    Self {
      objects,
      decisions,
      actions,
    }
  }
}