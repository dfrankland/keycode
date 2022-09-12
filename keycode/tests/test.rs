use keycode::{
    KeyMap, KeyMapping, KeyMappingCode, KeyMappingId, KeyModifiers, KeyState, KeyboardState,
};
use std::str::FromStr;

#[test]
fn can_get_a_key_map() {
    let a = KeyMap::from(KeyMappingId::UsA);
    assert_eq!(a.evdev, 30);
    assert_eq!(a.usb, 4);

    let key_map = KeyMap::from_key_mapping(KeyMapping::Evdev(a.evdev)).unwrap();
    assert_eq!(key_map.usb, a.usb)
}

#[test]
fn can_get_a_key_map_from_code_str() {
    let a = KeyMap::from(KeyMappingCode::from_str("KeyA").unwrap());
    assert_eq!(a.evdev, 30);
    assert_eq!(a.usb, 4);

    let key_map = KeyMap::from_key_mapping(KeyMapping::Evdev(a.evdev)).unwrap();
    assert_eq!(key_map.usb, a.usb)
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
        ControlLeft,
        ShiftLeft,
        AltLeft,
        MetaLeft,
        ControlRight,
        ShiftRight,
        AltRight,
        MetaRight,
    ];
}

#[test]
fn keyboard_state_works_for_usb_input_report() {
    let mut keyboard_state = KeyboardState::new(Some(6));
    assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);

    let a = KeyMap::from(KeyMappingId::UsA);
    let b = KeyMap::from(KeyMappingId::UsB);
    let c = KeyMap::from(KeyMappingId::UsC);
    let d = KeyMap::from(KeyMappingId::UsD);
    let e = KeyMap::from(KeyMappingId::UsE);
    let f = KeyMap::from(KeyMappingId::UsF);
    let g = KeyMap::from(KeyMappingId::UsG);
    let shift = KeyMap::from(KeyMappingId::ShiftLeft);

    // Press and release the "A" key
    keyboard_state.update_key(a, KeyState::Pressed);
    assert_eq!(
        keyboard_state.usb_input_report(),
        &[0, 0, a.usb as u8, 0, 0, 0, 0, 0]
    );
    keyboard_state.update_key(a, KeyState::Released);
    assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);

    // Press and release the "A" + "Shift" (left) keys
    keyboard_state.update_key(a, KeyState::Pressed);
    assert_eq!(
        keyboard_state.usb_input_report(),
        &[0, 0, a.usb as u8, 0, 0, 0, 0, 0]
    );
    keyboard_state.update_key(shift, KeyState::Pressed);
    assert_eq!(
        keyboard_state.usb_input_report(),
        &[
            KeyModifiers::ShiftLeft.bits(),
            0,
            a.usb as u8,
            0,
            0,
            0,
            0,
            0
        ]
    );
    keyboard_state.update_key(shift, KeyState::Released);
    assert_eq!(
        keyboard_state.usb_input_report(),
        &[0, 0, a.usb as u8, 0, 0, 0, 0, 0]
    );
    keyboard_state.update_key(a, KeyState::Released);
    assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);

    // Can't exceed key rollover
    keyboard_state.update_key(a, KeyState::Pressed);
    keyboard_state.update_key(b, KeyState::Pressed);
    keyboard_state.update_key(c, KeyState::Pressed);
    keyboard_state.update_key(d, KeyState::Pressed);
    keyboard_state.update_key(e, KeyState::Pressed);
    keyboard_state.update_key(f, KeyState::Pressed);
    keyboard_state.update_key(g, KeyState::Pressed);
    keyboard_state.update_key(shift, KeyState::Pressed);
    assert_eq!(
        keyboard_state.usb_input_report(),
        &[
            KeyModifiers::ShiftLeft.bits(),
            0,
            a.usb as u8,
            b.usb as u8,
            c.usb as u8,
            d.usb as u8,
            e.usb as u8,
            f.usb as u8
        ]
    );
}
