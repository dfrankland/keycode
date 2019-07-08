#![no_std]

use arraydeque::ArrayDeque;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use core::convert::TryFrom;

// https://www.usb.org/sites/default/files/documents/hid1_11.pdf
// Page 56
// 8.3 Report Format for Array Items
bitflags! {
    pub struct KeyModifiers: u8 {
        const CONTROL_LEFT  = 0b0000_0001;
        const SHIFT_LEFT    = 0b0000_0010;
        const ALT_LEFT      = 0b0000_0100;
        const META_LEFT     = 0b0000_1000;
        const CONTROL_RIGHT = 0b0001_0000;
        const SHIFT_RIGHT   = 0b0010_0000;
        const ALT_RIGHT     = 0b0100_0000; // 👎
        const META_RIGHT    = 0b1000_0000;
    }
}

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

// Max keys is 232
pub const NUM_KEYS: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyboardState {
    key_rollover: Option<usize>,
    key_state: ArrayDeque<[Option<KeyMap>; NUM_KEYS]>,
    modifier_state: KeyModifiers,
    input_report: ArrayVec<[u8; NUM_KEYS]>,
}

impl<'a> KeyboardState {
    pub fn new(key_rollover: Option<usize>) -> KeyboardState {
        KeyboardState {
            key_rollover,
            key_state: ArrayDeque::new(),
            modifier_state: KeyModifiers::empty(),
            input_report: ArrayVec::new(),
        }
    }

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
            .and_then(|key_rollover_without_modifiers| Some(key_rollover_without_modifiers + 2))
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
