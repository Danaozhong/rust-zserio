use crate::internal::ast::package::ZPackage;
use codegen::Scope;

pub fn add_standard_imports(scope: &mut Scope) {
    scope.import("rust_bitwriter", "BitWriter");
    scope.import("bitreader", "BitReader");
    scope.import("rust_zserio", "ztype");
    scope.import(
        "rust_zserio::ztype::array_traits::packing_context_node",
        "PackingContextNode",
    );
}

pub fn get_default_scope(package: &ZPackage, root_package: &String) -> Scope {
    let mut scope = Scope::new();
    // TODO Add a header comment

    let mut root_import = String::from("crate");
    if !root_package.is_empty() {
        root_import = root_import + "::" + root_package.as_str();
    }

    for import in &package.imports {
        let mut import_str = root_import.clone();
        for import_part in &import.package_dir {
            import_str = import_str + "::" + import_part.as_str();
        }

        let symbol_import = match &import.symbol_name {
            None => "*".into(),
            Some(symbol_name) => symbol_name.clone(),
        };
        scope.import(import_str.as_str(), symbol_import.as_str());
    }

    // Add the import to the current (own) module
    scope.import(
        (root_import + "::" + package.name.replace('.', "::").as_str()).as_str(),
        "*",
    );

    scope
}
