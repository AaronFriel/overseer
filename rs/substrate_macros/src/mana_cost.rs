use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use overseer_substrate_core as overseer_substrate;
use quote::quote;

pub(crate) fn mana_cost_impl(lit: &str) -> Result<proc_macro2::TokenStream> {
  let mana_cost = overseer_substrate::game::ManaCost::try_from(lit).map_err(|err| anyhow!(err))?;

  let pips: Vec<_> = mana_cost
    .pips()
    .iter()
    .map(|x| match x {
      overseer_substrate::game::ManaCostPip::Generic(n) => {
        quote! { overseer_substrate::game::ManaCostPip::Generic(#n) }
      }
      overseer_substrate::game::ManaCostPip::Colorless => quote! {
        overseer_substrate::game::ManaCostPip::Colorless
      },
      overseer_substrate::game::ManaCostPip::Color(c) => {
        let color = color_ctor(&c);
        quote! {
          overseer_substrate::game::ManaCostPip::Color(#color)
        }
      }
      overseer_substrate::game::ManaCostPip::X => {
        quote! { overseer_substrate::game::ManaCostPip::X }
      }
      overseer_substrate::game::ManaCostPip::Hybrid(h) => {
        let h = hybrid_color_ctor(h);
        quote! { overseer_substrate::game::ManaCostPip::Hybrid(#h) }
      }
      overseer_substrate::game::ManaCostPip::MonoHybrid(c) => {
        let c = color_ctor(&c);
        quote! { overseer_substrate::game::ManaCostPip::MonoHybrid(#c) }
      }
      overseer_substrate::game::ManaCostPip::Phyrexian(c) => {
        let c = color_ctor(&c);
        quote! { overseer_substrate::game::ManaCostPip::Phyrexian(#c) }
      }
      overseer_substrate::game::ManaCostPip::Snow => {
        quote! { overseer_substrate::game::ManaCostPip::Snow }
      }
    })
    .collect();

  Ok(quote! {
    overseer_substrate::game::ManaCost::from_static(&[#(#pips),*])
  })
}

fn color_ctor(c: &overseer_substrate::game::Color) -> proc_macro2::TokenStream {
  match c {
    overseer_substrate::game::Color::W => quote! { overseer_substrate::game::Color::W },
    overseer_substrate::game::Color::U => quote! { overseer_substrate::game::Color::U },
    overseer_substrate::game::Color::B => quote! { overseer_substrate::game::Color::B },
    overseer_substrate::game::Color::R => quote! { overseer_substrate::game::Color::R },
    overseer_substrate::game::Color::G => quote! { overseer_substrate::game::Color::G },
  }
}

fn hybrid_color_ctor(c: &overseer_substrate::game::HybridColor) -> proc_macro2::TokenStream {
  match *c {
    overseer_substrate::game::HybridColor::WU => {
      quote! { overseer_substrate::game::HybridColor::WU }
    }
    overseer_substrate::game::HybridColor::WB => {
      quote! { overseer_substrate::game::HybridColor::WB }
    }
    overseer_substrate::game::HybridColor::UB => {
      quote! { overseer_substrate::game::HybridColor::UB }
    }
    overseer_substrate::game::HybridColor::UR => {
      quote! { overseer_substrate::game::HybridColor::UR }
    }
    overseer_substrate::game::HybridColor::BR => {
      quote! { overseer_substrate::game::HybridColor::BR }
    }
    overseer_substrate::game::HybridColor::BG => {
      quote! { overseer_substrate::game::HybridColor::BG }
    }
    overseer_substrate::game::HybridColor::RG => {
      quote! { overseer_substrate::game::HybridColor::RG }
    }
    overseer_substrate::game::HybridColor::RW => {
      quote! { overseer_substrate::game::HybridColor::RW }
    }
    overseer_substrate::game::HybridColor::GW => {
      quote! { overseer_substrate::game::HybridColor::GW }
    }
    overseer_substrate::game::HybridColor::GU => {
      quote! { overseer_substrate::game::HybridColor::GU }
    }
  }
}
