#![feature(try_from)]

use bitflags::bitflags;
use std::convert::TryFrom;

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
