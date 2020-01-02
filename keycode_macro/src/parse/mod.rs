mod parse_keycode_list;

use self::parse_keycode_list::*;
use crate::key_map::KeyMap;
use anyhow::{anyhow, Result};
use proc_macro2::{TokenStream, TokenTree};
use std::collections::HashSet;

const USB_KEYMAP_DECLARATION_IDENT: &str = "USB_KEYMAP_DECLARATION";
const USB_KEYMAP_DECLARATION_END_PUNC: &str = ";";

pub fn parse(input: TokenStream) -> Result<HashSet<KeyMap>> {
    let mut iter = input.clone().into_iter();

    // Check for ident
    if let Some(usb_keymap_declaration_ident) = iter.next() {
        if usb_keymap_declaration_ident.to_string() != USB_KEYMAP_DECLARATION_IDENT {
            return Err(anyhow!(
                "Not used on a `{}` ident",
                USB_KEYMAP_DECLARATION_IDENT
            ));
        }
    } else {
        return Err(anyhow!("Missing `{}` ident", USB_KEYMAP_DECLARATION_IDENT));
    }

    // Parse through the keycodes
    let key_maps;
    if let Some(TokenTree::Group(usb_keymap_declaration_list)) = iter.next() {
        key_maps = parse_keycode_list(usb_keymap_declaration_list.stream())?;
        if key_maps.len() == 0 {
            return Err(anyhow!("`{}` list is empty", USB_KEYMAP_DECLARATION_IDENT));
        }
    } else {
        return Err(anyhow!("Missing `{}` list", USB_KEYMAP_DECLARATION_IDENT));
    }

    // Check for ending punc
    if let Some(usb_keymap_declaration_list_semicolon) = iter.next() {
        if usb_keymap_declaration_list_semicolon.to_string() != USB_KEYMAP_DECLARATION_END_PUNC {
            return Err(anyhow!(
                "`{}` does not end with a `{}`",
                USB_KEYMAP_DECLARATION_IDENT,
                USB_KEYMAP_DECLARATION_END_PUNC
            ));
        }
    } else {
        return Err(anyhow!(
            "`{}` is missing an ending `{}`",
            USB_KEYMAP_DECLARATION_IDENT,
            USB_KEYMAP_DECLARATION_END_PUNC
        ));
    }

    return Ok(key_maps);
}
