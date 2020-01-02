use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct KeyMap {
    pub usb_page_code: u16,
    pub usb_code: u16,
    pub evdev_code: u16,
    pub xkb_code: u16,
    pub win_code: u16,
    pub mac_code: u16,
    pub dom_code: Option<String>,
    pub variant: String,
}

impl PartialEq for KeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.variant == other.variant
    }
}

impl Eq for KeyMap {}

impl Hash for KeyMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.variant.hash(state);
    }
}
