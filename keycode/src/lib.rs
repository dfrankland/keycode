//! A Rust crate for translating keycodes based on Chrome's mapping of keys.
//!
//! Easily convert, generate, listen for, or map keycodes for Linux, Windows, Mac, USB, and
//! browsers! Includes a `struct` to manage the state of pressed keys and generate USB HID reports.
//! Can be used for `#![no_std]` crates.
//!
//! Source of keycodes data:
//! *   Repo: <https://chromium.googlesource.com/chromium/src.git>
//! *   File: <https://chromium.googlesource.com/chromium/src.git/+/master/ui/events/keycodes/dom/keycode_converter_data.inc>
//! *   Git commit: `2b6022954b9fb600f15e08002a148187f4f986da`
//!
//! # Example: get a key mapping
//!
//! ```
//! use keycode::{KeyMap, KeyMappingId};
//!
//! // Check the USB HID value of the "a" key
//! let a = KeyMap::from(KeyMappingId::UsA);
//! assert_eq!(a.usb, 0x0004);
//! assert_eq!(a.evdev, 0x001e);
//! assert_eq!(a.xkb, 0x0026);
//! assert_eq!(a.win, 0x001e);
//! assert_eq!(a.mac, 0x0000);
//! ```
//!
//! # Example: generate a USB HID report
//!
//! ```
//! use keycode::{KeyboardState, KeyMap, KeyMappingId, KeyState};
//!
//! // Press and release the "A" key
//! // Generate a keyboard state with n-key rollover
//! let mut keyboard_state = KeyboardState::new(None);
//!
//! // Get key mappings
//! let a = KeyMap::from(KeyMappingId::UsA);
//! let shift = KeyMap::from(KeyMappingId::ShiftLeft);
//!
//! // USB HID report for "no keys pressed"
//! assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);
//!
//! // Press "shift" and "a" keys
//! keyboard_state.update_key(a, KeyState::Pressed);
//! keyboard_state.update_key(shift, KeyState::Pressed);
//!
//! // USB HID report for "'A' is pressed"
//! assert_eq!(
//!     keyboard_state.usb_input_report(),
//!     &[shift.modifier.unwrap().bits(), 0, a.usb as u8, 0, 0, 0, 0, 0]
//! );
//!
//! // Release "shift" and "a" keys
//! keyboard_state.update_key(a, KeyState::Released);
//! keyboard_state.update_key(shift, KeyState::Released);
//!
//! // USB HID report for "no keys pressed"
//! assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);
//! ```

#![no_std]
#![deny(missing_docs)]

use arraydeque::ArrayDeque;
use arrayvec::ArrayVec;

use keycode_macro::parse_keycode_converter_data;

parse_keycode_converter_data!();

/// State of any key, whether it is pressed or not
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyState {
    /// Pressed key
    Pressed,
    /// Released key
    Released,
}

/// Max keys is 235, but this is the size of array used to manage state
pub const NUM_KEYS: usize = 256;

/// Keyboard state that helps manage pressed keys, rollover, and generating USB HID reports
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyboardState {
    key_rollover: Option<usize>,
    key_state: ArrayDeque<[Option<KeyMap>; NUM_KEYS]>,
    modifier_state: KeyModifiers,
    input_report: ArrayVec<[u8; NUM_KEYS]>,
}

impl<'a> KeyboardState {
    /// Create a new keyboard state
    pub fn new(key_rollover: Option<usize>) -> KeyboardState {
        KeyboardState {
            key_rollover,
            key_state: ArrayDeque::new(),
            modifier_state: KeyModifiers::empty(),
            input_report: ArrayVec::new(),
        }
    }

    /// Update the keyboard state with a key's new state
    pub fn update_key(self: &mut Self, key: KeyMap, state: KeyState) {
        match state {
            KeyState::Pressed => {
                if let Some(key_modifier) = key.modifier {
                    self.modifier_state.insert(key_modifier);
                    return;
                }

                // Already contains key
                if self.key_state.contains(&Some(key)) {
                    return;
                }

                // Key state can't store anymore keys
                if self.key_state.is_full() {
                    return;
                }

                // Key rollover limit is met
                if let Some(key_rollover) = self.key_rollover {
                    if self.key_state.len() >= key_rollover {
                        return;
                    }
                }

                // We check if the `key_state` is full above, so this should be safe.
                self.key_state.push_back(Some(key)).unwrap();
            }
            KeyState::Released => {
                if let Some(key_modifier) = key.modifier {
                    self.modifier_state.remove(key_modifier);
                    return;
                }

                if self.key_state.is_empty() {
                    return;
                }

                self.key_state.retain(|k| *k != Some(key));
            }
        }
    }

    /// Generate a USB HID report
    pub fn usb_input_report(self: &mut Self) -> &[u8] {
        let mut input_report: ArrayVec<[u8; NUM_KEYS]> = ArrayVec::new();

        // Key modifiers
        input_report.push(self.modifier_state.bits());
        input_report.push(0);

        // Normal keys
        for possible_key in self.key_state.iter() {
            if let Some(key) = possible_key {
                input_report.push(key.usb as u8);
            }
        }

        // Default (not pressed)
        let min_input_report_size = self
            .key_rollover
            .map(|key_rollover_without_modifiers| key_rollover_without_modifiers + 2)
            .unwrap_or(8);
        if input_report.len() < min_input_report_size {
            for _ in input_report.len()..min_input_report_size {
                input_report.push(0);
            }
        }

        self.input_report = input_report;
        self.input_report.as_slice()
    }
}
