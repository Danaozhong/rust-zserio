use crate::internal::ast::package::ZPackage;
use crate::internal::model::package::package_from_file;
use std::borrow::BorrowMut;
use std::path::Path;
use walkdir::WalkDir;

pub struct Model {
    pub packages: Vec<Box<ZPackage>>,
}

/// Loads a complete zserio model from a directory.
pub fn from_filesystem(directory: &Path) -> Box<Model> {
    let mut packages = Vec::new();

    for entry in WalkDir::new(directory) {
        let path = entry.unwrap().path().to_owned();
        if path.is_dir() {
            continue;
        }
        let mut file_name: String = path.file_name().unwrap().to_str().unwrap().into();
        file_name = file_name.to_ascii_lowercase();
        if file_name.ends_with(".zs") || file_name.ends_with(".zserio") {
            let package = package_from_file(&path);
            packages.push(package);
        }
    }
    Box::new(Model { packages: packages })
}
