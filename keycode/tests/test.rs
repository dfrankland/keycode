use keycode::{KeyMap, KeyMapping, KeyMappingId, KeyModifiers, KeyState, KeyboardState};

#[test]
fn can_get_a_key_map() {
    let a = KeyMap::from(KeyMappingId::US_A);
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

#[test]
fn keyboard_state_works_for_usb_input_report() {
    let mut keyboard_state = KeyboardState::new(Some(6));
    assert_eq!(keyboard_state.usb_input_report(), &[0; 8]);

    let a = KeyMap::from(KeyMappingId::US_A);
    let b = KeyMap::from(KeyMappingId::US_B);
    let c = KeyMap::from(KeyMappingId::US_C);
    let d = KeyMap::from(KeyMappingId::US_D);
    let e = KeyMap::from(KeyMappingId::US_E);
    let f = KeyMap::from(KeyMappingId::US_F);
    let g = KeyMap::from(KeyMappingId::US_G);
    let shift = KeyMap::from(KeyMappingId::SHIFT_LEFT);

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
            KeyModifiers::SHIFT_LEFT.bits(),
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
            KeyModifiers::SHIFT_LEFT.bits(),
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
