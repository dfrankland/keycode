use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lib.rs");

    let keycode_converter_data = format!(
        "
            {}
            {}
        ",
        r#"
            #[derive(Debug, Clone)]
            pub enum KeyMapping<'a, 'b> {
                Usb(u16),
                Evdev(u16),
                Xkb(u16),
                Win(u16),
                Mac(u16),
                Code(&'a str),
                Id(&'b str),
            }

            #[derive(Debug, Clone)]
            pub struct KeyMap {
                pub usb: u16,
                pub evdev: u16,
                pub xkb: u16,
                pub win: u16,
                pub mac: u16,
                pub code: String,
                pub id: String,
            }

            macro_rules! USB_KEYMAP_DECLARATION {
                {
                    $(USB_KEYMAP($usb:expr, $evdev:expr, $xkb:expr, $win:expr, $mac:expr, $code:expr, $id:ident),)*
                } => {
                    pub fn get_key_map(key_mapping: KeyMapping) -> Result<KeyMap, String> {
                        match key_mapping {
                            $(
                                KeyMapping::Usb($usb) | KeyMapping::Evdev($evdev) | KeyMapping::Xkb($xkb) | KeyMapping::Win($win) | KeyMapping::Mac($mac) | KeyMapping::Code($code) | KeyMapping::Id(stringify!($id)) => {
                                    Ok(KeyMap {
                                        usb: $usb,
                                        evdev: $evdev,
                                        xkb: $xkb,
                                        win: $win,
                                        mac: $mac,
                                        code: String::from($code),
                                        id: String::from(stringify!($id))
                                    })
                                },
                            )*
                            _ => Err(String::from("No key mapping found."))
                        }
                    }
                }
            }
        "#,
        include_str!("keycode_converter_data.inc")
            .replace("USB_KEYMAP_DECLARATION", "USB_KEYMAP_DECLARATION!") // Make variable into macro
            .replace("};", "}") // Macros don't have semicolons
            .replace("NULL", "\"NULL\"") // Make code string consistent
            .replace("USB_KEYMAP(0x07", "USB_KEYMAP(0x") // Ignore HID usage page
            .replace("USB_KEYMAP(0x0c", "USB_KEYMAP(0x") // Ignore HID usage page
            .replace("USB_KEYMAP(0x01", "USB_KEYMAP(0x"), // Ignore HID usage page
    );

    println!("{}", keycode_converter_data);

    let mut file = File::create(&dest_path).unwrap();
    file.write_all(keycode_converter_data.as_bytes()).unwrap();
}
