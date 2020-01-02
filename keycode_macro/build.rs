#[cfg(test)]
fn main() {
    use std::{env, fs::File, io::Write, path::Path};

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("keycode_converter.rs");

    let keycode_converter_data = format!(
        "parse_keycode_converter_data!{{{}}}",
        include_str!("keycode_converter_data.inc"),
    );

    let mut file = File::create(&dest_path).unwrap();
    file.write_all(keycode_converter_data.as_bytes()).unwrap();
}

#[cfg(not(test))]
fn main() {}
