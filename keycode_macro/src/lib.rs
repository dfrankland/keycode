extern crate proc_macro;

mod generate;
mod key_map;
mod parse;

use self::{generate::*, parse::*};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn parse_keycode_converter_data(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = match parse(input.clone()) {
        Ok(key_maps) => generate(key_maps),
        Err(err) => {
            let err_string = err.to_string();
            quote! {
                compile_error!(#err_string)
            }
        }
    };

    TokenStream::from(output)
}
