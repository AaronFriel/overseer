use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use overseer_substrate_core as overseer_substrate;
use quote::{format_ident, quote};

pub(crate) fn type_line_impl(lit: &str) -> Result<proc_macro2::TokenStream> {
  let type_line = overseer_substrate::game::TypeLine::from_str(lit).map_err(|err| anyhow!(err))?;

  let supertypes: Result<Vec<_>, _> = type_line.supertypes().map(tokenize_supertype).collect();
  let supertypes = supertypes?;

  let types: Result<Vec<_>, _> = type_line.types().map(tokenize_type).collect();
  let types = types?;

  let subtypes: Result<Vec<_>, _> = type_line.subtypes().map(tokenize_subtype).collect();
  let subtypes = subtypes?;

  let items = std::iter::empty::<proc_macro2::TokenStream>()
    .chain(supertypes)
    .chain(types)
    .chain(subtypes);

  Ok(quote! {
    overseer_substrate::game::TypeLine::from_static(&[#(#items),*])
  })
}

fn tokenize_supertype(
  v: &overseer_substrate::game::CardSupertype,
) -> Result<proc_macro2::TokenStream> {
  let tokenized = match v {
    overseer_substrate::game::CardSupertype::Predefined(v) => {
      let x = format_ident!("{}", as_ident(v));

      quote! {
        overseer_substrate::game::TypeLineValue::Supertype(
          overseer_substrate::game::CardSupertype::Predefined(
            overseer_substrate::game::PredefinedSupertype::#x
          )
        )
      }
    }
    _ => bail!(
      "Cannot serialize a non-predefined type {} into a constant",
      v
    ),
  };

  Ok(tokenized)
}

fn tokenize_type(v: &overseer_substrate::game::CardType) -> Result<proc_macro2::TokenStream> {
  let tokenized = match v {
    overseer_substrate::game::CardType::Predefined(v) => {
      let x = format_ident!("{}", as_ident(v));

      quote! {
        overseer_substrate::game::TypeLineValue::JustAType(
          overseer_substrate::game::CardType::Predefined(
            overseer_substrate::game::PredefinedType::#x
          )
        )
      }
    }
    _ => bail!(
      "Cannot serialize a non-predefined type {} into a constant",
      v
    ),
  };

  Ok(tokenized)
}

fn tokenize_subtype(v: &overseer_substrate::game::CardSubtype) -> Result<proc_macro2::TokenStream> {
  let tokenized = match v {
    overseer_substrate::game::CardSubtype::Predefined(v) => match v {
      overseer_substrate::game::PredefinedSubtype::Artifact(v) => {
        let formatted = as_ident(v);
        as_ident(formatted);

        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Artifact(
            overseer_substrate::game::ArtifactType::#x
          )
        }
      }
      overseer_substrate::game::PredefinedSubtype::Enchantment(v) => {
        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Enchantment(
            overseer_substrate::game::EnchantmentType::#x
          )
        }
      }
      overseer_substrate::game::PredefinedSubtype::Land(v) => {
        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Land(
            overseer_substrate::game::LandType::#x
          )
        }
      }
      overseer_substrate::game::PredefinedSubtype::Planeswalker(v) => {
        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Planeswalker(
            overseer_substrate::game::PlaneswalkerType::#x
          )
        }
      }
      overseer_substrate::game::PredefinedSubtype::Spell(v) => {
        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Spell(
            overseer_substrate::game::SpellType::#x
          )
        }
      }
      overseer_substrate::game::PredefinedSubtype::Creature(v) => {
        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Creature(
            overseer_substrate::game::CreatureType::#x
          )
        }
      }
      overseer_substrate::game::PredefinedSubtype::Planar(v) => {
        let x = format_ident!("{}", as_ident(v));

        quote! {
          overseer_substrate::game::PredefinedSubtype::Planar(
            overseer_substrate::game::PlanarType::#x
          )
        }
      }
    },
    _ => bail!(
      "Cannot serialize a non-predefined type {} into a constant",
      v
    ),
  };

  let tokenized = quote! {
    overseer_substrate::game::TypeLineValue::Subtype(
      overseer_substrate::game::CardSubtype::Predefined(
        #tokenized
      )
    )
  };

  Ok(tokenized)
}

fn as_ident(value: impl AsRef<str>) -> String {
  value.as_ref().replace(|c: char| !c.is_alphabetic(), "")
}
