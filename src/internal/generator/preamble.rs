
use codegen::Scope;
use crate::internal::ast::package::ZPackage;

pub fn add_standard_imports(scope: &mut Scope) {
    scope.import("rust_bitwriter", "BitWriter");
    scope.import("bitreader", "BitReader");
    scope.import( "rust_zserio", "ztype");
}

pub fn get_default_scope(package: &ZPackage) -> Scope {
    let mut scope = Scope::new();
    // TODO Add a header comment
    for import in &package.imports {
        let mut import_str = String::from("crate");
        for import_part in &import.package_dir {
            import_str = import_str + "::" + import_part.as_str();
        }
        scope.import(import_str.as_str(), import.symbol_name.as_str());
    }

    // Add the import to the current (own) module
    scope.import((String::from("crate::") + package.name.replace(".", "::").as_str()).as_str(), "*");

    scope

}