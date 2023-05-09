
use std::io::Write;

use codegen::Scope;

use crate::internal::ast::zenum::ZEnum;
use crate::internal::generator::{
    file_generator::write_to_file,
    encode::encode_field,
    bitsize::bitsize_type_reference,
    types::zserio_to_rust_type,
    types::convert_name,
};
use std::path::{Path, PathBuf};

pub fn generate_enum(zenum: &ZEnum, path: &Path, package_name: &String) {
    let mut scope = Scope::new();

    // generate the struct itself
    let gen_enum = scope.new_enum(&zenum.name);
    gen_enum.vis("pub");
    
    for item in &zenum.items {
        gen_enum.new_variant(&item.name);
    }

    // generate the functions to serialize/deserialize
    let mut z_impl = scope.new_impl(&zenum.name);

    let mut marshal_fn = z_impl.new_fn("marshal_zserio");
    marshal_fn.arg_ref_self();

    let mut unmarshal_fn = z_impl.new_fn("unmarshal_zserio");
    unmarshal_fn.arg_ref_self();
    
    let mut bitsize_fn = z_impl.new_fn("zserio_bitsize");
    bitsize_fn.vis("pub");
    bitsize_fn.ret("int");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "int");

    bitsize_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(bitsize_fn, "this".into(), false, &*zenum.enum_type);

    write_to_file(
        &scope.to_string(), 
        path,
        package_name,
        &zenum.name,
    );
}