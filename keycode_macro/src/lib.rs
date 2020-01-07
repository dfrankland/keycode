extern crate proc_macro;

mod generate;
mod key_map;
mod parse;

use self::{generate::*, parse::*};
use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;

#[proc_macro]
pub fn parse_keycode_converter_data(_input: TokenStream) -> TokenStream {
    let output = if let Ok(input) =
        TokenStream::from_str(&include_str!("../keycode_converter_data.inc").to_string())
    {
        let input = proc_macro2::TokenStream::from(input);

        match parse(input) {
            Ok(key_maps) => generate(key_maps),
            Err(err) => {
                let err_string = err.to_string();
                quote! {
                    compile_error!(#err_string)
                }
            }
        }
    } else {
        quote! {
            compile_error!("Could not parse `keycode_converter_data.inc` file into a `TokenStream`")
        }
    };

    TokenStream::from(output)
}
