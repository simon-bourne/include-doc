use std::fmt::Display;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Error, Result};

pub trait Tokens {
    fn tokens(self) -> TokenStream;
}

impl Tokens for Result<proc_macro2::TokenStream> {
    fn tokens(self) -> TokenStream {
        self.unwrap_or_else(syn::Error::into_compile_error).into()
    }
}

pub fn call_site(message: impl Display) -> Error {
    Error::new(Span::call_site(), message)
}
