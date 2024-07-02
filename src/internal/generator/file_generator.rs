use crate::internal::generator::types::TypeGenerator;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write_to_file(
    type_generator: &mut TypeGenerator,
    content: &String,
    root_path: &Path,
    zserio_pkg_name: &str,
    file_name: &str,
) {
    let format_result = rustfmt_wrapper::rustfmt(content);
    if format_result.is_err() {
        panic!(
            "code formatting failed: error {:?}, content {:?}",
            format_result.err(),
            content
        );
    }
    // We need to run rustfmt twice to stabilize the formatting of vec! macros
    let formatted_code = rustfmt_wrapper::rustfmt(format_result.unwrap()).unwrap();
    let file_bytes = formatted_code.as_bytes();

    let mut file_path = root_path.to_owned();
    for dir in String::from(zserio_pkg_name).split('.') {
        file_path = file_path.join(type_generator.to_rust_module_name(dir));
    }
    fs::create_dir_all(file_path.as_path()).expect("mkdir failed");
    let full_path = file_path.join(String::from(file_name) + ".rs");
    println!("Writing file  {}", full_path.to_str().unwrap());
    let mut file_ref = std::fs::File::create(full_path).expect("create failed");
    file_ref.write_all(file_bytes).expect("write failed");
    file_ref.flush().expect("can not flush data to disk");
}
