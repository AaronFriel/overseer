mod mana_cost;
mod type_line;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

use self::{mana_cost::*, type_line::*};

#[proc_macro]
pub fn mana_cost(tokens: TokenStream) -> TokenStream {
  let lit = parse_macro_input!(tokens as LitStr);

  match mana_cost_impl(&lit.value()) {
    Ok(output) => TokenStream::from(output),
    Err(err) => syn::Error::new(lit.span(), err).to_compile_error().into(),
  }
}

#[proc_macro]
pub fn type_line(tokens: TokenStream) -> TokenStream {
  let lit = parse_macro_input!(tokens as LitStr);

  match type_line_impl(&lit.value()) {
    Ok(output) => TokenStream::from(output),
    Err(err) => syn::Error::new(lit.span(), err).to_compile_error().into(),
  }
}
