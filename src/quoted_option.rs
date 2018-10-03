//! A small trait that enables tokenizing Option<T> as Some(t) or None.

use proc_macro2::TokenStream;
use quote::ToTokens;

pub trait QuotedOption {
  fn quote(self) -> TokenStream;
}

impl<T> QuotedOption for Option<T> where T: ToTokens {
  fn quote(self) -> TokenStream {
    if let Some(x) = self {
      quote!{ Some(#x) }
    } else {
      quote!{ None }
    }
  }
}
