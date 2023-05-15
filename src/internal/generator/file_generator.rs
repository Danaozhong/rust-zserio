use rust_format::{Formatter, RustFmt};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write_to_file(content: &String, root_path: &Path, zserio_pkg_name: &str, file_name: &str) {
    let formatted_code = RustFmt::default()
        .format_str(content)
        .expect("code formatting failed");
    let file_bytes = formatted_code.as_bytes();

    let mut file_path = root_path.to_owned();
    for dir in String::from(zserio_pkg_name).split(".") {
        file_path = file_path.join(dir);
    }
    fs::create_dir_all(file_path.as_path()).expect("mkdir failed");
    let filename = file_path.join(String::from(file_name) + ".rs");
    let mut fileRef = std::fs::File::create(filename).expect("create failed");
    fileRef.write_all(file_bytes).expect("write failed");
}
