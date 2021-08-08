use std::num::NonZeroU128;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonZeroUuid(pub NonZeroU128);

impl Serialize for NonZeroUuid {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    Uuid::from_u128(self.0.get()).serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for NonZeroUuid {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de;
    if let Some(value) = NonZeroU128::new(Uuid::deserialize(deserializer)?.as_u128()) {
      Ok(NonZeroUuid(value))
    } else {
      Err(de::Error::invalid_value(
        de::Unexpected::Unsigned(0),
        &"non-zero value",
      ))
    }
  }
}
