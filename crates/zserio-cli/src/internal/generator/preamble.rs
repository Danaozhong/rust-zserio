use codegen::Scope;

pub fn add_standard_imports(scope: &mut Scope) {
    scope.import("rust_bitwriter", "BitWriter");
    scope.import("bitreader", "BitReader");
    scope.import("zserio", "Result");
    scope.import("zserio", "ZserioPackableObject");
    scope.import("zserio", "ztype");

    scope.import(
        "zserio::ztype::array_traits::packing_context_node",
        "PackingContextNode",
    );
    scope.import("zserio::ztype::array_traits::delta_context", "DeltaContext");
}
