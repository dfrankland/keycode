use crate::key_map::KeyMap;
use anyhow::{anyhow, Result};
use heck::CamelCase;
use proc_macro2::{TokenStream, TokenTree};
use std::{collections::HashSet, convert::TryFrom};

const HEX_START: &str = "0x";

fn parse_hex<T: TryFrom<u32>>(hex_string: String) -> Result<T, ()> {
    if hex_string.starts_with(HEX_START) {
        let num =
            u32::from_str_radix(hex_string.trim_start_matches(HEX_START), 16).map_err(|_| ())?;
        return T::try_from(num).map_err(|_| ());
    }

    Err(())
}

const STRING_LITERAL_QUOTE: &str = r#"""#;

fn parse_literal_string_to_ident_string(literal_string: String) -> Result<String, ()> {
    if literal_string.starts_with(STRING_LITERAL_QUOTE)
        && literal_string.ends_with(STRING_LITERAL_QUOTE)
    {
        return Ok(String::from(
            literal_string
                .trim_start_matches(STRING_LITERAL_QUOTE)
                .trim_end_matches(STRING_LITERAL_QUOTE),
        ));
    }

    Err(())
}

const USB_KEYMAP_IDENT: &str = "USB_KEYMAP";
const USB_KEYMAP_USB_CODE: &str = "USB_CODE";
const USB_KEYMAP_EVDEV_CODE: &str = "EVDEV_CODE";
const USB_KEYMAP_XKB_CODE: &str = "XKB_CODE";
const USB_KEYMAP_WIN_CODE: &str = "WIN_CODE";
const USB_KEYMAP_MAC_CODE: &str = "MAC_CODE";
const USB_KEYMAP_DOM_CODE: &str = "DOM_CODE";
const USB_KEYMAP_DOM_CODE_NULL_IDENT: &str = "NULL";
const USB_KEYMAP_VARIANT: &str = "VARIANT";
const USB_KEYMAP_ITEMS: usize = 7;

pub fn parse_keycode_list(input: TokenStream) -> Result<HashSet<KeyMap>> {
    let mut key_maps = HashSet::new();
    let mut list_iter = input.into_iter();
    while let Some(item) = list_iter.next() {
        if item.to_string() != USB_KEYMAP_IDENT {
            continue;
        }
        if let Some(TokenTree::Group(usb_keymap_group)) = list_iter.next() {
            let mut usb_keymap: Vec<TokenTree> = usb_keymap_group
                .stream()
                .into_iter()
                .filter(|i| {
                    if let TokenTree::Punct(_) = i {
                        false
                    } else {
                        true
                    }
                })
                .take(USB_KEYMAP_ITEMS)
                .collect();

            if usb_keymap.len() != USB_KEYMAP_ITEMS {
                return Err(anyhow!(
                    "`{}` does not contain a full group of `({}, {}, {}, {}, {}, {}, {})`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_USB_CODE,
                    USB_KEYMAP_EVDEV_CODE,
                    USB_KEYMAP_XKB_CODE,
                    USB_KEYMAP_WIN_CODE,
                    USB_KEYMAP_MAC_CODE,
                    USB_KEYMAP_DOM_CODE,
                    USB_KEYMAP_VARIANT,
                ));
            }

            usb_keymap.reverse();

            let usb_code;
            let usb_page_code;
            if let Some(TokenTree::Literal(literal)) = usb_keymap.pop() {
                if let Ok(code) = parse_hex::<u32>(literal.to_string()) {
                    let usb_page_and_code_bytes = code.to_le_bytes();
                    usb_code = u16::from_le_bytes([
                        usb_page_and_code_bytes[0],
                        usb_page_and_code_bytes[1],
                    ]);
                    usb_page_code = u16::from_le_bytes([
                        usb_page_and_code_bytes[2],
                        usb_page_and_code_bytes[3],
                    ]);
                } else {
                    return Err(anyhow!(
                        "`{}` could not be parsed into a `u32`",
                        USB_KEYMAP_USB_CODE
                    ));
                }
            } else {
                return Err(anyhow!(
                    "`{}` does not contain a literal value for `{}`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_USB_CODE
                ));
            }

            let evdev_code;
            if let Some(TokenTree::Literal(literal)) = usb_keymap.pop() {
                if let Ok(code) = parse_hex::<u16>(literal.to_string()) {
                    evdev_code = code;
                } else {
                    return Err(anyhow!(
                        "`{}` could not be parsed into a `u16`",
                        USB_KEYMAP_EVDEV_CODE
                    ));
                }
            } else {
                return Err(anyhow!(
                    "`{}` does not contain a literal value for `{}`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_EVDEV_CODE
                ));
            }

            let xkb_code;
            if let Some(TokenTree::Literal(literal)) = usb_keymap.pop() {
                if let Ok(code) = parse_hex::<u16>(literal.to_string()) {
                    xkb_code = code;
                } else {
                    return Err(anyhow!(
                        "`{}` could not be parsed into a `u16`",
                        USB_KEYMAP_XKB_CODE
                    ));
                }
            } else {
                return Err(anyhow!(
                    "`{}` does not contain a literal value for `{}`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_XKB_CODE
                ));
            }

            let win_code;
            if let Some(TokenTree::Literal(literal)) = usb_keymap.pop() {
                if let Ok(code) = parse_hex::<u16>(literal.to_string()) {
                    win_code = code;
                } else {
                    return Err(anyhow!(
                        "`{}` could not be parsed into a `u16`",
                        USB_KEYMAP_WIN_CODE
                    ));
                }
            } else {
                return Err(anyhow!(
                    "`{}` does not contain a literal value for `{}`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_WIN_CODE
                ));
            }

            let mac_code;
            if let Some(TokenTree::Literal(literal)) = usb_keymap.pop() {
                if let Ok(code) = parse_hex::<u16>(literal.to_string()) {
                    mac_code = code;
                } else {
                    return Err(anyhow!(
                        "`{}` could not be parsed into a `u16`",
                        USB_KEYMAP_MAC_CODE
                    ));
                }
            } else {
                return Err(anyhow!(
                    "`{}` does not contain a literal value for `{}`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_MAC_CODE
                ));
            }

            let dom_code = match usb_keymap.pop() {
                Some(TokenTree::Literal(literal)) => {
                    if let Ok(ident_string) =
                        parse_literal_string_to_ident_string(literal.to_string())
                    {
                        Some(ident_string)
                    } else {
                        return Err(anyhow!(
                            "`{}` contains a non-string literal",
                            USB_KEYMAP_DOM_CODE
                        ));
                    }
                }
                Some(TokenTree::Ident(ident)) => {
                    if ident.to_string() != USB_KEYMAP_DOM_CODE_NULL_IDENT {
                        return Err(anyhow!("`{}` contains an ident that is not equal to `{}` (alternatively provide a string)", USB_KEYMAP_DOM_CODE, USB_KEYMAP_DOM_CODE_NULL_IDENT));
                    }
                    None
                }
                _ => {
                    return Err(anyhow!(
                        "`{}` does not contain a literal value or `{}` for `{}`",
                        USB_KEYMAP_IDENT,
                        USB_KEYMAP_DOM_CODE_NULL_IDENT,
                        USB_KEYMAP_DOM_CODE
                    ));
                }
            };

            let variant;
            if let Some(TokenTree::Ident(ident)) = usb_keymap.pop() {
                variant = ident.to_string().to_camel_case();
            } else {
                return Err(anyhow!(
                    "`{}` does not contain an ident for `{}`",
                    USB_KEYMAP_IDENT,
                    USB_KEYMAP_VARIANT
                ));
            }

            key_maps.insert(KeyMap {
                usb_page_code,
                usb_code,
                evdev_code,
                xkb_code,
                win_code,
                mac_code,
                dom_code,
                variant,
            });
        } else {
            return Err(anyhow!("Missing `{}` declaration", USB_KEYMAP_IDENT));
        }
    }

    Ok(key_maps)
}
