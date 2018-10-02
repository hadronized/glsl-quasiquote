extern crate glsl;
extern crate proc_macro;

use glsl::parser::parse_str;
use glsl::parsers::external_declaration;
use proc_macro::TokenStream;

#[proc_macro]
pub fn glsl(input: TokenStream) -> TokenStream {
  let s = format!("{}", input);
  let parsed = parse_str(s.as_str(), external_declaration);

  input
}
