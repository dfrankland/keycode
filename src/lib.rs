use bitflags::bitflags;
use std::{collections::VecDeque, convert::TryFrom};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyboardState {
    key_rollover: Option<u16>,
    key_state: VecDeque<KeyMap>,
    modifier_state: KeyModifiers,
}

impl KeyboardState {
    pub fn new(key_rollover: Option<u16>) -> KeyboardState {
        KeyboardState {
            key_rollover,
            key_state: VecDeque::new(),
            modifier_state: KeyModifiers::empty(),
        }
    }

    pub fn update_key(self: &mut Self, key: KeyMap, state: KeyState) {
        match state {
            KeyState::Pressed => {
                if let Some(key_modifier) = key.modifier {
                    self.modifier_state.insert(key_modifier);
                    return;
                }

                if self.key_state.contains(&key) {
                    return;
                }

                if let Some(key_rollover) = self.key_rollover {
                    if key_rollover as usize <= self.key_state.len() {
                        return;
                    }
                }

                self.key_state.push_back(key);
            }
            KeyState::Released => {
                if let Some(key_modifier) = key.modifier {
                    self.modifier_state.remove(key_modifier);
                    return;
                }

                if !self.key_state.is_empty() {
                    let key_state_position = self.key_state.iter().position(|k| *k == key);
                    if let Some(index) = key_state_position {
                        self.key_state.remove(index);
                    }
                }
            }
        };
    }

    pub fn usb_input_report(self: &Self) -> Vec<u8> {
        let mut input_report = vec![];

        input_report.push(self.modifier_state.bits());
        input_report.push(0);

        for key in self.key_state.iter() {
            input_report.push(key.usb as u8);
        }

        if let Some(key_rollover) = self.key_rollover {
            for _ in 0..(key_rollover as usize - self.key_state.len()) {
                input_report.push(0);
            }
        }

        input_report
    }
}
