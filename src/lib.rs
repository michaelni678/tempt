use std::convert;

use proc_macro::TokenStream;

mod core;

#[proc_macro]
pub fn tempt(input: TokenStream) -> TokenStream {
    core::expand(input.into())
        .unwrap_or_else(convert::identity)
        .into()
}
