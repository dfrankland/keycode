use keycode::{get_key_map, KeyMapping};

#[test]
fn can_get_a_key_map() {
    let key_map = get_key_map(KeyMapping::Usb(4)).unwrap();
    println!("{:?}", key_map);
    assert_eq!(key_map.evdev, 30)
}
