use crate::internal::ast::field::Field;
use crate::internal::ast::zstruct::ZStruct;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::array::instantiate_zserio_arrays;
use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field,
    file_generator::write_to_file, function::generate_function, new::new_field, new::new_param,
    preamble::add_standard_imports, types::convert_field_name, types::to_rust_module_name,
    types::to_rust_type_name, types::TypeGenerator,
};
use codegen::Scope;
use codegen::Struct;
use std::cell::RefCell;
use std::rc::Rc;

use std::path::Path;

pub fn generate_struct_member_for_field(
    type_generator: &TypeGenerator,
    gen_struct: &mut Struct,
    field: &Field,
) {
    let mut field_type = type_generator.ztype_to_rust_type(field.field_type.as_ref());

    if field.array.is_some() {
        field_type = format!("Vec<{}>", field_type.as_str());
    }
    if field.is_optional {
        field_type = format!("Option<{}>", field_type.as_str());
    }
    let gen_field = gen_struct.new_field(&convert_field_name(&field.name), &field_type);
    gen_field.vis("pub");
}

pub fn generate_struct(
    symbol_scope: &ModelScope,
    type_generator: &TypeGenerator,
    codegen_scope: &mut Scope,
    zstruct: &ZStruct,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zstruct.name);
    let rust_type_name = to_rust_type_name(&zstruct.name);

    add_standard_imports(codegen_scope);
    // add the imports

    // generate the struct itself
    let gen_struct = codegen_scope.new_struct(&rust_type_name);
    gen_struct.vis("pub");
    gen_struct.derive("Clone");
    gen_struct.derive("PartialEq");

    // if the field is parameterized, add the parameters as member variables
    for param in &zstruct.type_parameters {
        let param_type =
            type_generator.ztype_to_rust_type(param.as_ref().borrow().zserio_type.as_ref());
        // Possible improvement: currently the parameters are copied instead of taken the reference.
        // It would be great to change that to references to avoid unnecessary copying, but this is
        // painful in rust due to the lifetime checks.
        // Because I am lazy, this implementation will just copy values over.
        let gen_param_field = gen_struct.new_field(
            convert_field_name(&param.as_ref().borrow().name),
            param_type,
        );
        gen_param_field.vis("pub");
    }

    // Add the data fields to the struct
    for field in &zstruct.fields {
        generate_struct_member_for_field(type_generator, gen_struct, &field.borrow());
    }

    // generate the functions to serialize/deserialize
    let struct_impl = codegen_scope.new_impl(&rust_type_name);
    struct_impl.impl_trait("ztype::ZserioPackableOject");

    // Generate a function to create a new instance of the struct
    let new_fn = struct_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!("{} {{", &rust_type_name));

    for param in &zstruct.type_parameters {
        new_param(
            symbol_scope,
            type_generator,
            new_fn,
            &param.as_ref().borrow(),
        );
    }

    for field in &zstruct.fields {
        new_field(symbol_scope, type_generator, new_fn, &field.borrow());
    }
    new_fn.line("}");

    // Generate the functions to read, write and bitcount the data to/from zserio format.
    generate_zserio_read(symbol_scope, type_generator, struct_impl, &zstruct.fields);
    generate_zserio_write(symbol_scope, type_generator, struct_impl, &zstruct.fields);
    generate_zserio_bitsize(symbol_scope, type_generator, struct_impl, &zstruct.fields);

    // Generate all the zserio functions.
    let pub_impl = codegen_scope.new_impl(&rust_type_name);
    for zserio_function in &zstruct.functions {
        generate_function(pub_impl, type_generator, &zserio_function.as_ref().borrow());
    }

    write_to_file(
        &codegen_scope.to_string(),
        path,
        package_name,
        &rust_module_name,
    );
    rust_module_name
}

fn generate_zserio_read(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    struct_impl: &mut codegen::Impl,
    fields: &Vec<Rc<RefCell<Field>>>,
) {
    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    instantiate_zserio_arrays(scope, type_generator, zserio_read_fn, fields, false);
    for field in fields {
        decode_field(scope, type_generator, zserio_read_fn, &field.borrow(), None);
    }

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    instantiate_zserio_arrays(scope, type_generator, zserio_read_packed_fn, fields, true);
    for field in fields {
        decode_field(
            scope,
            type_generator,
            zserio_read_packed_fn,
            &field.borrow(),
            Option::from(0),
        ); // TODO node index
    }
}

fn generate_zserio_write(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    struct_impl: &mut codegen::Impl,
    fields: &Vec<Rc<RefCell<Field>>>,
) {
    let zserio_write_fn = struct_impl.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");

    instantiate_zserio_arrays(scope, type_generator, zserio_write_fn, fields, false);
    for field_rc in fields {
        let field = field_rc.borrow();
        encode_field(scope, type_generator, zserio_write_fn, &field, None);
    }

    let zserio_write_packed_fn = struct_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    instantiate_zserio_arrays(scope, type_generator, zserio_write_packed_fn, fields, true);
    for field in fields {
        encode_field(
            scope,
            type_generator,
            zserio_write_packed_fn,
            &field.borrow(),
            Option::from(0),
        ); // TODO node index
    }
}

fn generate_zserio_bitsize(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    struct_impl: &mut codegen::Impl,
    fields: &Vec<Rc<RefCell<Field>>>,
) {
    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    instantiate_zserio_arrays(scope, type_generator, bitsize_fn, fields, false);
    bitsize_fn.line("let mut end_position = bit_position;");
    for field in fields {
        bitsize_field(scope, type_generator, bitsize_fn, &field.borrow(), None);
    }
    bitsize_fn.line("end_position - bit_position");

    let bitsize_packed_fn = struct_impl.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("u64");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    instantiate_zserio_arrays(scope, type_generator, bitsize_packed_fn, fields, true);
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    for field in fields {
        bitsize_field(
            scope,
            type_generator,
            bitsize_packed_fn,
            &field.borrow(),
            Option::from(0),
        );
    }
    bitsize_packed_fn.line("end_position - bit_position");
}
