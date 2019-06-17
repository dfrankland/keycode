# keycode

A Rust crate for translating keycodes based on Chrome's mapping of keys.

## Current Source File Commit

### Repo

`https://chromium.googlesource.com/chromium/src.git`

### File

`ui/events/keycodes/dom/keycode_converter_data.inc`

### Commit Hash

`1fe838d`

## How to Update Source File

```bash
curl -sL 'https://chromium.googlesource.com/chromium/src/+/master/ui/events/keycodes/dom/keycode_converter_data.inc?format=TEXT' | base64 --decode > keycode_converter_data.inc
```

## Supported Rust Versions

Requires Rust 1.34.0 or newer due to use of [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html).
