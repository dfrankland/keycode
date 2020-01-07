use crate::key_map::KeyMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

#[allow(clippy::cognitive_complexity)]
pub fn generate(key_maps: HashSet<KeyMap>) -> TokenStream {
    let (usbs, evdevs, xkbs, wins, macs, codes, code_matches, ids) = key_maps.iter().fold(
        (
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ),
        |(
            mut usbs,
            mut evdevs,
            mut xkbs,
            mut wins,
            mut macs,
            mut codes,
            mut code_matches,
            mut ids,
        ),
         key_map| {
            ids.push(format_ident!("{}", key_map.variant));
            if let Some(code) = &key_map.dom_code {
                let code_ident = format_ident!("{}", code);
                codes.push(code_ident.clone());
                code_matches.push(quote! {
                    Some(KeyMappingCode::#code_ident)
                });
            } else {
                code_matches.push(quote! {
                    None
                });
            }
            usbs.push(key_map.usb_code);
            evdevs.push(key_map.evdev_code);
            xkbs.push(key_map.xkb_code);
            wins.push(key_map.win_code);
            macs.push(key_map.mac_code);
            (usbs, evdevs, xkbs, wins, macs, codes, code_matches, ids)
        },
    );

    quote! {
        use bitflags::bitflags;
        use core::convert::TryFrom;

        bitflags! {
            /// Bitmask for key modifiers based on the USB HID standard
            ///
            /// See the stardard here:
            ///
            /// <https://www.usb.org/sites/default/files/documents/hid1_11.pdf>
            ///
            /// Go to page 56, "8.3 Report Format for Array Items"
            pub struct KeyModifiers: u8 {
                /// Control left key bitmask
                const ControlLeft  = 0b0000_0001;
                /// Shift left key bitmask
                const ShiftLeft    = 0b0000_0010;
                /// Alt left key bitmask
                const AltLeft      = 0b0000_0100;
                /// Meta left key bitmask
                const MetaLeft     = 0b0000_1000;
                /// Control right key bitmask
                const ControlRight = 0b0001_0000;
                /// Shift right key bitmask
                const ShiftRight   = 0b0010_0000;
                /// Alt right key bitmask
                const AltRight     = 0b0100_0000; // 👎
                /// Meta right key bitmask
                const MetaRight    = 0b1000_0000;
            }
        }

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
            Code(Option<KeyMappingCode>),
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
            pub code: Option<KeyMappingCode>,
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

        fn get_key_map(key_mapping: KeyMapping) -> Result<KeyMap, ()> {
            #[allow(unreachable_patterns)]
            match key_mapping {
                #(
                    KeyMapping::Usb(#usbs) | KeyMapping::Evdev(#evdevs) | KeyMapping::Xkb(#xkbs) | KeyMapping::Win(#wins) | KeyMapping::Mac(#macs) | KeyMapping::Id(KeyMappingId::#ids) => {
                        let id = KeyMappingId::#ids;
                        let keymap = KeyMap {
                            usb: #usbs,
                            evdev: #evdevs,
                            xkb: #xkbs,
                            win: #wins,
                            mac: #macs,
                            code: #code_matches,
                            modifier: match id {
                                KeyMappingId::ControlLeft => Some(KeyModifiers::ControlLeft),
                                KeyMappingId::ShiftLeft => Some(KeyModifiers::ShiftLeft),
                                KeyMappingId::AltLeft => Some(KeyModifiers::AltLeft),
                                KeyMappingId::MetaLeft => Some(KeyModifiers::MetaLeft),
                                KeyMappingId::ControlRight => Some(KeyModifiers::ControlRight),
                                KeyMappingId::ShiftRight => Some(KeyModifiers::ShiftRight),
                                KeyMappingId::AltRight => Some(KeyModifiers::AltRight),
                                KeyMappingId::MetaRight => Some(KeyModifiers::MetaRight),
                                _ => None,
                            },
                            id,
                        };
                        Ok(keymap)
                    },
                )*
                #(
                    KeyMapping::Code(#code_matches) => {
                        let id = KeyMappingId::#ids;
                        let keymap = KeyMap {
                            usb: #usbs,
                            evdev: #evdevs,
                            xkb: #xkbs,
                            win: #wins,
                            mac: #macs,
                            code: #code_matches,
                            modifier: match id {
                                KeyMappingId::ControlLeft => Some(KeyModifiers::ControlLeft),
                                KeyMappingId::ShiftLeft => Some(KeyModifiers::ShiftLeft),
                                KeyMappingId::AltLeft => Some(KeyModifiers::AltLeft),
                                KeyMappingId::MetaLeft => Some(KeyModifiers::MetaLeft),
                                KeyMappingId::ControlRight => Some(KeyModifiers::ControlRight),
                                KeyMappingId::ShiftRight => Some(KeyModifiers::ShiftRight),
                                KeyMappingId::AltRight => Some(KeyModifiers::AltRight),
                                KeyMappingId::MetaRight => Some(KeyModifiers::MetaRight),
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
            #(
                #[doc = "W3 browser event code for a specific key"]
                #codes,
            )*
        }

        impl core::fmt::Display for KeyMappingCode {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    #(
                        KeyMappingCode::#codes => write!(f, stringify!(#codes)),
                    )*
                }
            }
        }

        impl From<KeyMappingCode> for KeyMap {
            fn from(code: KeyMappingCode) -> KeyMap {
                get_key_map(KeyMapping::Code(Some(code))).unwrap()
            }
        }

        /// Id for a specific key
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
        pub enum KeyMappingId {
            #(
                #[doc = "Id for a specific key"]
                #[allow(non_camel_case_types)]
                #ids,
            )*
        }

        impl core::fmt::Display for KeyMappingId {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    #(
                        KeyMappingId::#ids => write!(f, stringify!(#ids)),
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
