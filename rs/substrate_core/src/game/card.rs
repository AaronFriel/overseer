use std::{borrow::Cow, collections::HashMap, sync::RwLock};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_diff::SerdeDiff;

use crate::game::{Characterized, ManaCost, ObjectColor, ObjectColored, ObjectHandle, TypeLine};

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(Serialize, Deserialize, SerdeDiff)]
pub struct Card {
  #[serde_diff(opaque)]
  pub name: Cow<'static, str>,
  #[serde_diff(opaque)]
  pub mana_cost: ManaCost,

  pub color_indicator: Option<ObjectColor>,

  pub type_line: TypeLine,

  #[serde_diff(opaque)]
  pub rules_text: Cow<'static, str>,

  pub power: u32,
  pub toughness: u32,

  pub loyalty: u32,

  #[cfg(feature = "vanguard")]
  /// 210.
  pub hand_modifier: i8,

  #[cfg(feature = "vanguard")]
  /// 211.
  pub life_modifier: i8,
  /* expansion symbol
   * illustration
   * illustration credit
   * legal text
   * collector number */
}

impl Card {
  pub const DEFAULT: Card = Card::const_default();

  pub const fn const_default() -> Self {
    Self {
      name: Cow::Borrowed(""),
      mana_cost: ManaCost::NONE,
      color_indicator: None,
      type_line: TypeLine::const_default(),
      rules_text: Cow::Borrowed(""),
      power: 0,
      toughness: 0,
      loyalty: 0,
      #[cfg(feature = "vanguard")]
      hand_modifier: 0,
      #[cfg(feature = "vanguard")]
      life_modifier: 0,
    }
  }
}

impl Characterized for Card {
  fn get_name(&self) -> &Cow<'static, str> {
    &self.name
  }

  fn get_mana_cost(&self) -> &ManaCost {
    &self.mana_cost
  }

  fn get_color(&self) -> ObjectColor {
    self
      .color_indicator
      .unwrap_or(self.mana_cost.get_object_color())
  }

  fn get_color_indicator(&self) -> Option<ObjectColor> {
    self.color_indicator
  }

  fn get_type_line(&self) -> &TypeLine {
    &self.type_line
  }

  fn get_rules_text(&self) -> &Cow<'static, str> {
    &self.rules_text
  }

  fn get_power(&self) -> u32 {
    self.power
  }

  fn get_toughness(&self) -> u32 {
    self.toughness
  }

  fn get_loyalty(&self) -> u32 {
    self.loyalty
  }

  #[cfg(feature = "vanguard")]
  fn get_hand_modifier(&self) -> i8 {
    self.hand_modifier
  }

  #[cfg(feature = "vanguard")]
  fn get_life_modifier(&self) -> i8 {
    self.life_modifier
  }
}

lazy_static::lazy_static! {
  static ref CARD_REGISTRY: RwLock<HashMap<String, Cow<'static, Card>>> = RwLock::new(HashMap::new());
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
#[derive(SerdeDiff)]
pub struct RegisteredCard(Cow<'static, Card>);

impl From<&'static Card> for RegisteredCard {
  fn from(card: &'static Card) -> Self {
    RegisteredCard(Cow::Borrowed(card))
  }
}

impl std::ops::Deref for RegisteredCard {
  type Target = Card;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl RegisteredCard {
  pub fn register(card: &'static Card) -> Self {
    register_card(&Cow::Borrowed(card)).unwrap()
  }
}

fn register_card(card: &Cow<'static, Card>) -> Result<RegisteredCard, ()> {
  match &card {
    Cow::Owned(_) => Ok(RegisteredCard(card.clone())),
    Cow::Borrowed(x) => {
      // We've got a static reference to a card, safe to use registry
      {
        // Try a read first
        let registry = CARD_REGISTRY
          .read()
          .map_err(|_| panic!("Card registry lock poisoned"))?;

        if let Some(registered_card) = registry.get(&*card.name) {
          if registered_card != card {
            panic!("Registry poisoned with invalid card data, card inserted twice",);
          } else {
            return Ok(RegisteredCard(card.clone()));
          }
        }
      }
      {
        {
          let mut registry = CARD_REGISTRY
            .write()
            .map_err(|_| panic!("Card registry lock poisoned"))?;

          let registered_card = registry
            .entry(card.name.to_string())
            .or_insert(Cow::Borrowed(*x));
          if registered_card != card {
            panic!("Registry poisoned with invalid card data, card inserted twice");
          } else {
            return Ok(RegisteredCard(registered_card.clone()));
          }
        }
      }
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub(crate) enum RegisteredCardSerde {
  Registered(Cow<'static, str>),
  Card(Card),
}

impl Serialize for RegisteredCard {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match &self.0 {
      Cow::Owned(x) => RegisteredCardSerde::Card(x.clone()).serialize(serializer),
      Cow::Borrowed(x) => {
        // We've got a static reference to a card, safe to use registry
        {
          // Try a read first
          let registry = CARD_REGISTRY
            .read()
            .map_err(|_| serde::ser::Error::custom("Card registry lock poisoned"))?;

          if let Some(registered_card) = registry.get(&*self.0.name) {
            if registered_card != &self.0 {
              return Err(serde::ser::Error::custom(
                "Registry poisoned with invalid card data, card inserted twice",
              ));
            } else {
              return RegisteredCardSerde::Registered(self.0.name.clone()).serialize(serializer);
            }
          }
        }
        {
          {
            let mut registry = CARD_REGISTRY
              .write()
              .map_err(|_| serde::ser::Error::custom("Card registry lock poisoned"))?;

            let registered_card = registry
              .entry(self.0.name.to_string())
              .or_insert(Cow::Borrowed(*x));
            if registered_card != &self.0 {
              return Err(serde::ser::Error::custom(
                "Registry poisoned with invalid card data, card inserted twice",
              ));
            } else {
              return RegisteredCardSerde::Registered(self.0.name.clone()).serialize(serializer);
            }
          }
        }
      }
    }
  }
}

impl<'de> Deserialize<'de> for RegisteredCard {
  fn deserialize<D>(deserializer: D) -> Result<RegisteredCard, D::Error>
  where
    D: Deserializer<'de>,
  {
    match RegisteredCardSerde::deserialize(deserializer)? {
      RegisteredCardSerde::Registered(c) => {
        use serde::de::Error;
        let registry = CARD_REGISTRY
          .read()
          .map_err(|_| Error::custom("Card registry lock poisoned"))?;

        if let Some(registered_card) = registry.get(&*c) {
          Ok(RegisteredCard(registered_card.clone()))
        } else {
          Err(Error::custom("Card not in registry"))
        }
      }
      RegisteredCardSerde::Card(card) => Ok(RegisteredCard(Cow::Owned(card))),
    }
  }
}

pub type CardList = Vec<ObjectHandle>;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn assert_sizeof() {
    let expected_size = 144;
    #[cfg(feature = "vanguard")]
    let expected_size = expected_size + 2;

    assert_eq!(std::mem::size_of::<Card>(), expected_size);
  }
}
