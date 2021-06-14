use crate::game::{ObjectHandle, PlayerHandle};

pub enum Target {
  Object(ObjectHandle),
  Player(PlayerHandle),
}
