# keycode

A Rust crate for translating keycodes based on Chrome's mapping of keys.

Easily convert, generate, listen for, or map keycodes for Linux, Windows, Mac,
USB, and browsers! Includes a `struct` to manage the state of pressed keys and
generate USB HID reports. Can be used for `#![no_std]` crates.

## Source Data

Source of keycodes data:

*   Repo: <https://chromium.googlesource.com/chromium/src.git>
*   File: <https://chromium.googlesource.com/chromium/src.git/+/master/ui/events/keycodes/dom/keycode_converter_data.inc>
*   Git commit: `2b6022954b9fb600f15e08002a148187f4f986da`

How to update source file:

```bash
curl -sL 'https://chromium.googlesource.com/chromium/src/+/master/ui/events/keycodes/dom/keycode_converter_data.inc?format=TEXT' | base64 --decode > keycode_converter_data.inc
```

## Examples

### Get a key mapping

```rust
use keycode::{KeyMap, KeyMappingId};

// Check the USB HID value of the "a" key
fn main() {
    let a = KeyMap::from(KeyMappingId::UsA);
    assert_eq!(a.usb, 0x0004);
    assert_eq!(a.evdev, 0x001e);
    assert_eq!(a.xkb, 0x0026);
    assert_eq!(a.win, 0x001e);
    assert_eq!(a.mac, 0x0000);
}
```

### Generate a USB HID report

```rust
use keycode::{KeyboardState, KeyMap, KeyMappingId, KeyState};

// Press and release the "A" key
fn main() {
    // Generate a keyboard state with n-key rollover
    let mut keyboard_state = KeyboardState::new(None);

    // Get key mappings
    let a = KeyMap::from(KeyMappingId::UsA);
    let shift = KeyMap::from(KeyMappingId::ShiftLeft);

    // USB HID report for "no keys pressed"
    assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);

    // Press "shift" and "a" keys
    keyboard_state.update_key(a, KeyState::Pressed);
    keyboard_state.update_key(shift, KeyState::Pressed);

    // USB HID report for "'A' is pressed"
    assert_eq!(
        keyboard_state.usb_input_report(),
        &[shift.modifier.unwrap().bits(), 0, a.usb as u8, 0, 0, 0, 0, 0]
    );

    // Release "shift" and "a" keys
    keyboard_state.update_key(a, KeyState::Released);
    keyboard_state.update_key(shift, KeyState::Released);

    // USB HID report for "no keys pressed"
    assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);
}
```

## Supported Rust Versions

Requires Rust 1.34.0 or newer due to use of
[TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html).

## Developing

I recommend to use Nix and the flake within this repo:

```bash
nix develop
```
