use keycode::{KeyMap, KeyMapping, KeyMappingId, KeyModifiers};

#[test]
fn can_get_a_key_map() {
    let key_map = KeyMap::from_key_mapping(KeyMapping::Evdev(30)).unwrap();
    assert_eq!(key_map.usb, 4)
}

macro_rules! check_modifiers {
    [$($modifier:ident,)*] => {
        $(
            assert_eq!(
                KeyMap::from_key_mapping(KeyMapping::Id(KeyMappingId::$modifier))
                    .unwrap()
                    .modifier
                    .unwrap(),
                KeyModifiers::$modifier
            );
        )*
    }
}

#[test]
fn modifiers_are_set_correctly() {
    check_modifiers![
        CONTROL_LEFT,
        SHIFT_LEFT,
        ALT_LEFT,
        META_LEFT,
        CONTROL_RIGHT,
        SHIFT_RIGHT,
        ALT_RIGHT,
        META_RIGHT,
    ];
}
