#![recursion_limit = "256"]

use quote::quote;
use regex::{Captures, Regex};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lib.rs");

    let keycode_converter_data = format!(
        "
            {}
            {}
        ",
        quote! {
            /// The mapping of values between platforms for a specific key
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
            pub enum KeyMapping {
                /// USB HID value for a specific key
                Usb(u16),
                /// Linux kernel evdev value for a specific key
                Evdev(u16),
                /// X11 value for a specific key
                Xkb(u16),
                /// Windows value for a specific key
                Win(u16),
                /// Mac value for a specific key
                Mac(u16),
                /// W3 browser event code for a specific key
                Code(KeyMappingCode),
                /// Id for a specific key
                Id(KeyMappingId),
            }

            /// Ergonomic access to a specific key's mapping of values
            #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
            pub struct KeyMap {
                /// USB HID value for a specific key
                pub usb: u16,
                /// Linux kernel evdev value for a specific key
                pub evdev: u16,
                /// X11 value for a specific key
                pub xkb: u16,
                /// Windows value for a specific key
                pub win: u16,
                /// Mac value for a specific key
                pub mac: u16,
                /// W3 browser event code for a specific key
                pub code: KeyMappingCode,
                /// Id for a specific key
                pub id: KeyMappingId,
                /// USB HID bitmask
                pub modifier: Option<KeyModifiers>,
            }

            impl KeyMap {
                /// If you don't want to use TryFrom, until it is stabilized
                pub fn from_key_mapping(key_mapping: KeyMapping) -> Result<KeyMap, ()> {
                    get_key_map(key_mapping)
                }
            }

            impl TryFrom<KeyMapping> for KeyMap {
                type Error = ();
                fn try_from(key_mapping: KeyMapping) -> Result<KeyMap, Self::Error> {
                    get_key_map(key_mapping)
                }
            }

            macro_rules! USB_KEYMAP_DECLARATION {
                {
                    $(USB_KEYMAP($usb:expr, $evdev:expr, $xkb:expr, $win:expr, $mac:expr, $code:ident, $id:ident),)*
                } => {
                    fn get_key_map(key_mapping: KeyMapping) -> Result<KeyMap, ()> {
                        #[allow(unreachable_patterns)]
                        match key_mapping {
                            $(
                                KeyMapping::Usb($usb) | KeyMapping::Evdev($evdev) | KeyMapping::Xkb($xkb) | KeyMapping::Win($win) | KeyMapping::Mac($mac) | KeyMapping::Code(KeyMappingCode::$code) | KeyMapping::Id(KeyMappingId::$id) => {
                                    let id = KeyMappingId::$id;
                                    let keymap = KeyMap {
                                        usb: $usb,
                                        evdev: $evdev,
                                        xkb: $xkb,
                                        win: $win,
                                        mac: $mac,
                                        code: KeyMappingCode::$code,
                                        modifier: match id {
                                            KeyMappingId::CONTROL_LEFT => Some(KeyModifiers::CONTROL_LEFT),
                                            KeyMappingId::SHIFT_LEFT => Some(KeyModifiers::SHIFT_LEFT),
                                            KeyMappingId::ALT_LEFT => Some(KeyModifiers::ALT_LEFT),
                                            KeyMappingId::META_LEFT => Some(KeyModifiers::META_LEFT),
                                            KeyMappingId::CONTROL_RIGHT => Some(KeyModifiers::CONTROL_RIGHT),
                                            KeyMappingId::SHIFT_RIGHT => Some(KeyModifiers::SHIFT_RIGHT),
                                            KeyMappingId::ALT_RIGHT => Some(KeyModifiers::ALT_RIGHT),
                                            KeyMappingId::META_RIGHT => Some(KeyModifiers::META_RIGHT),
                                            _ => None,
                                        },
                                        id,
                                    };
                                    Ok(keymap)
                                },
                            )*
                            _ => Err(())
                        }
                    }

                    /// W3 browser event code for a specific key
                    ///
                    /// <https://www.w3.org/TR/uievents-code/>
                    #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
                    pub enum KeyMappingCode {
                        $(
                            #[doc = "W3 browser event code for a specific key"]
                            $code,
                        )*
                    }

                    impl core::fmt::Display for KeyMappingCode {
                        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                            match *self {
                                $(
                                    KeyMappingCode::$code => write!(f, stringify!($code)),
                                )*
                            }
                        }
                    }

                    impl From<KeyMappingCode> for KeyMap {
                        fn from(code: KeyMappingCode) -> KeyMap {
                            get_key_map(KeyMapping::Code(code)).unwrap()
                        }
                    }

                    /// Id for a specific key
                    #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
                    pub enum KeyMappingId {
                        $(
                            #[doc = "Id for a specific key"]
                            #[allow(non_camel_case_types)]
                            $id,
                        )*
                    }

                    impl core::fmt::Display for KeyMappingId {
                        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                            match *self {
                                $(
                                    KeyMappingId::$id => write!(f, stringify!($id)),
                                )*
                            }
                        }
                    }

                    impl From<KeyMappingId> for KeyMap {
                        fn from(id: KeyMappingId) -> KeyMap {
                            get_key_map(KeyMapping::Id(id)).unwrap()
                        }
                    }
                }
            }
        },
        {
            let mut file = include_str!("keycode_converter_data.inc").to_string();

            // Remove any existing macros
            file = Regex::new("(?m)^#(if|define|include|undef|endif).*?$")
                .unwrap()
                .replace_all(&file, "")
                .to_string();

            // Make variable into macro
            file = Regex::new("(USB_KEYMAP_DECLARATION)")
                .unwrap()
                .replace_all(&file, "$1!")
                .to_string();

            // Macros don't have semicolons
            file = Regex::new(r"(\});")
                .unwrap()
                .replace_all(&file, "$1")
                .to_string();

            // Ignore HID usage page + fix for linting
            file = Regex::new(r"(USB_KEYMAP\(0x)..(....)")
                .unwrap()
                .replace_all(&file, "$1$2")
                .to_string();

            // Make codes idents, but don't replace the quotes in comments
            let comment_prefix = r"//";
            let quote_char = "\"";
            let quote_placeholder = r"#####";
            let comment_quote_regex =
                Regex::new(format!("{}(?P<a>.*?){}", comment_prefix, quote_char).as_str()).unwrap();
            let comment_quote_placeholder_regex =
                Regex::new(format!("{}(?P<a>.*?){}", comment_prefix, quote_placeholder).as_str())
                    .unwrap();
            let all_quotes_regex = Regex::new(quote_char).unwrap();

            while comment_quote_regex.is_match(&file) {
                file = comment_quote_regex
                    .replace_all(
                        &file,
                        format!("{}$a{}", comment_prefix, quote_placeholder).as_str(),
                    )
                    .to_string();
            }
            file = all_quotes_regex.replace_all(&file, "").to_string();
            while comment_quote_placeholder_regex.is_match(&file) {
                file = comment_quote_placeholder_regex
                    .replace_all(
                        &file,
                        format!("{}$a{}", comment_prefix, quote_char).as_str(),
                    )
                    .to_string();
            }

            // Make NULL into a unique ident
            let mut counter = 0;
            file = Regex::new("NULL")
                .unwrap()
                .replace_all(&file, move |_: &Captures| {
                    counter += 1;
                    format!("Null{}", counter)
                })
                .to_string();

            file
        }
    );

    println!("{}", keycode_converter_data);

    let mut file = File::create(&dest_path).unwrap();
    file.write_all(keycode_converter_data.as_bytes()).unwrap();
}
