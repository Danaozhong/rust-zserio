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
