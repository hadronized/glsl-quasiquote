//! A set of small traits that enable tokenizing some common types that get tokenizing erased
//! normally, such as `Option<T>` as `Some(_)` or `None`, `Box<T>` as `Box::new(_)`, etc.

use proc_macro2::TokenStream;
use quote::ToTokens;

// Quoted type.
pub trait Quoted {
  fn quote(self) -> TokenStream;
}

impl<T> Quoted for Option<T> where T: ToTokens {
  fn quote(self) -> TokenStream {
    if let Some(x) = self {
      quote!{ Some(#x) }
    } else {
      quote!{ None }
    }
  }
}

impl<T> Quoted for Box<T> where T: ToTokens {
  fn quote(self) -> TokenStream {
    quote!{ Box::new(#self) }
  }
}

impl<'a> Quoted for &'a str {
  fn quote(self) -> TokenStream {
    quote!{ String::from(#self) }
  }
}
