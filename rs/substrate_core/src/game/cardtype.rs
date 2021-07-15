mod subtype;

use std::{
  borrow::Cow,
  fmt::{Display, Write},
  str::FromStr,
};

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use strum::{AsRefStr, Display, EnumString, EnumVariantNames, VariantNames};

pub use self::subtype::*;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(untagged)]
#[serde_diff(opaque)]
pub enum TypeLineValue {
  Supertype(CardSupertype),
  JustAType(CardType),
  Subtype(CardSubtype),
}

impl TypeLineValue {
  /// Returns `true` if the type_line_value is [`Supertype`].
  pub fn is_supertype(&self) -> bool {
    matches!(self, Self::Supertype(..))
  }

  /// Returns `true` if the type_line_value is [`JustAType`].
  pub fn is_just_a_type(&self) -> bool {
    matches!(self, Self::JustAType(..))
  }

  /// Returns `true` if the type_line_value is [`Subtype`].
  pub fn is_subtype(&self) -> bool {
    matches!(self, Self::Subtype(..))
  }

  pub fn as_supertype(&self) -> Option<&CardSupertype> {
    if let Self::Supertype(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_just_a_type(&self) -> Option<&CardType> {
    if let Self::JustAType(v) = self {
      Some(v)
    } else {
      None
    }
  }

  pub fn as_subtype(&self) -> Option<&CardSubtype> {
    if let Self::Subtype(v) = self {
      Some(v)
    } else {
      None
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde_diff(opaque)]
pub struct TypeLine(Cow<'static, [TypeLineValue]>);

impl TypeLine {
  pub const NONE: TypeLine = TypeLine(Cow::Borrowed(&[]));

  pub const fn from_static(value: &'static [TypeLineValue]) -> Self {
    TypeLine(Cow::Borrowed(value))
  }

  pub const fn const_default() -> TypeLine {
    TypeLine(Cow::Borrowed(&[]))
  }

  pub fn supertypes(&self) -> impl Iterator<Item = &CardSupertype> {
    self.iter().filter_map(|x| x.as_supertype())
  }

  pub fn types(&self) -> impl Iterator<Item = &CardType> {
    self.iter().filter_map(|x| x.as_just_a_type())
  }

  pub fn subtypes(&self) -> impl Iterator<Item = &CardSubtype> {
    self.iter().filter_map(|x| x.as_subtype())
  }
}

impl std::ops::Deref for TypeLine {
  type Target = Cow<'static, [TypeLineValue]>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Default for TypeLine {
  fn default() -> Self {
    TypeLine(Cow::Borrowed(&[]))
  }
}

const LONG_DASH: char = '—';

impl Display for TypeLine {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut printed_dash = false;

    let mut iter = self.iter().peekable();
    while let Some(type_value) = iter.next() {
      match type_value {
        TypeLineValue::Supertype(v) => v.fmt(f)?,
        TypeLineValue::JustAType(v) => v.fmt(f)?,
        TypeLineValue::Subtype(v) => {
          if printed_dash == false {
            f.write_char(LONG_DASH)?;
            f.write_char(' ')?;
            printed_dash = true;
          }
          v.fmt(f)?;
        }
      }

      if iter.peek().is_some() {
        f.write_char(' ')?;
      }
    }

    Ok(())
  }
}

const DASHES: &'static [&'static str] = &[
  "-", "‐", "‑", "‒", "–", "—", "―", "⸺", "⸻", "﹘", "﹣", "－",
];

impl FromStr for TypeLine {
  type Err = String;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let mut planar_types: Vec<PlanarType> = Vec::new();
    let mut value = value.to_owned();

    if value.split_ascii_whitespace().any(|x| x == "Plane") {
      let variants_with_spaces = PlanarType::VARIANTS
        .iter()
        .filter(|x| x.contains(char::is_whitespace));
      for variant in variants_with_spaces {
        let current_len = value.len();
        value = value.replace(variant, "");

        if value.len() != current_len {
          planar_types.push(PlanarType::from_str(variant).unwrap());
        }
      }
    }

    let value = value.split_ascii_whitespace();

    let mut result: Vec<TypeLineValue> = Vec::new();

    for part in value {
      if part.is_empty() || DASHES.contains(&part) {
        continue;
      }

      let parsed_item = PredefinedSupertype::from_str(part)
        .map(CardSupertype::Predefined)
        .map(TypeLineValue::Supertype)
        .or(
          PredefinedType::from_str(part)
            .map(CardType::Predefined)
            .map(TypeLineValue::JustAType),
        )
        .or(
          PredefinedSubtype::from_str(part)
            .map(CardSubtype::Predefined)
            .map(TypeLineValue::Subtype),
        )
        .or(Err(format!("Invalid type line value {}", part)))?;

      result.push(parsed_item);
    }

    for planar_type in planar_types {
      result.push(TypeLineValue::Subtype(CardSubtype::Predefined(
        PredefinedSubtype::Planar(planar_type),
      )));
    }

    Ok(TypeLine(Cow::Owned(result)))
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(untagged)]
#[serde_diff(opaque)]
pub enum CardSupertype {
  Predefined(PredefinedSupertype),
  Custom(Cow<'static, str>),
}

impl Display for CardSupertype {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CardSupertype::Predefined(v) => v.fmt(f),
      CardSupertype::Custom(v) => v.fmt(f),
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, EnumVariantNames)] // From strum
#[serde_diff(opaque)]
pub enum PredefinedSupertype {
  Basic,
  Legendary,
  Ongoing,
  Snow,
  World,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[serde(untagged)]
#[serde_diff(opaque)]
pub enum CardType {
  Predefined(PredefinedType),
  Custom(Cow<'static, str>),
}

impl Display for CardType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CardType::Predefined(v) => v.fmt(f),
      CardType::Custom(v) => v.fmt(f),
    }
  }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
#[derive(Serialize, Deserialize, SerdeDiff)]
#[derive(Display, EnumString, AsRefStr, EnumVariantNames)] // From strum
#[serde_diff(opaque)]
pub enum PredefinedType {
  Artifact,
  Conspiracy,
  Creature,
  Enchantment,
  Instant,
  Land,
  Phenomenon,
  Plane,
  Planeswalker,
  Scheme,
  Sorcery,
  Tribal,
  Vanguard,
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn assert_sizeof() {
    assert_eq!(
      std::mem::size_of::<TypeLine>(),
      std::mem::size_of::<usize>() * 4
    );
  }
}
