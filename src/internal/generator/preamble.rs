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

pub fn get_default_scope(package: &ZPackage) -> Scope {
    let mut scope = Scope::new();
    // TODO Add a header comment
    for import in &package.imports {
        let mut import_str = String::from("crate");
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
        (String::from("crate::") + package.name.replace('.', "::").as_str()).as_str(),
        "*",
    );

    scope
}
