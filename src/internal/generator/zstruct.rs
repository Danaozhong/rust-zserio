use crate::internal::ast::zstruct::ZStruct;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::array::instantiate_zserio_array;
use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field,
    file_generator::write_to_file, function::generate_function, new::new_field, new::new_param,
    packed_contexts::generate_init_packed_context_for_field,
    packed_contexts::generate_packed_context_for_field, packed_contexts::FieldDetails,
    preamble::add_standard_imports, types::convert_field_name, types::to_rust_module_name,
    types::to_rust_type_name, types::TypeGenerator,
};
use codegen::Scope;
use codegen::Struct;

use std::path::Path;

pub fn generate_struct_member_for_field(gen_struct: &mut Struct, field: &FieldDetails) {
    let mut field_type = field.rust_type.clone();
    if field.field.borrow().array.is_some() {
        field_type = format!("Vec<{}>", &field_type);
    }
    if field.field.borrow().is_optional {
        field_type = format!("Option<{}>", &field_type);
    }
    gen_struct
        .new_field(&field.field_name, &field_type)
        .vis("pub");
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

    let mut field_details = vec![];
    for (field_index, field_rc) in zstruct.fields.iter().enumerate() {
        field_details.push(FieldDetails::from_field(
            field_rc,
            field_index,
            symbol_scope,
            type_generator,
        ));
    }

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
    for field in &field_details {
        generate_struct_member_for_field(gen_struct, field);
    }

    // generate the functions to serialize/deserialize
    let struct_impl = codegen_scope.new_impl(&rust_type_name);
    struct_impl.impl_trait("ztype::ZserioPackableObject");

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
    generate_zserio_read(symbol_scope, type_generator, struct_impl, &field_details);
    generate_zserio_write(symbol_scope, type_generator, struct_impl, &field_details);
    generate_zserio_bitsize(symbol_scope, type_generator, struct_impl, &field_details);

    // Generate the packed contexts.
    let create_packing_context_fn = struct_impl.new_fn("zserio_create_packing_context");
    create_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    for field in &field_details {
        generate_packed_context_for_field(symbol_scope, create_packing_context_fn, field);
    }

    let init_packing_context_fn = struct_impl.new_fn("zserio_init_packing_context");
    init_packing_context_fn.arg_ref_self();
    init_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    for field in &field_details {
        generate_init_packed_context_for_field(
            symbol_scope,
            type_generator,
            init_packing_context_fn,
            field,
        );
    }

    // Generate all the zserio functions (defined in the zserio language).
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
    fields: &Vec<FieldDetails>,
) {
    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    for field in fields {
        instantiate_zserio_array(
            scope,
            type_generator,
            zserio_read_fn,
            &field.field.borrow(),
            false,
        );
        decode_field(
            scope,
            type_generator,
            zserio_read_fn,
            &field.field.borrow(),
            None,
        );
    }

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    for field in fields {
        instantiate_zserio_array(
            scope,
            type_generator,
            zserio_read_packed_fn,
            &field.field.borrow(),
            true,
        );
        decode_field(
            scope,
            type_generator,
            zserio_read_packed_fn,
            &field.field.borrow(),
            Option::from(field.field_index as u8),
        );
    }
}

fn generate_zserio_write(
    symbol_scope: &ModelScope,
    type_generator: &TypeGenerator,
    struct_impl: &mut codegen::Impl,
    fields: &Vec<FieldDetails>,
) {
    let zserio_write_fn = struct_impl.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");

    for field_rc in fields {
        let field = field_rc.field.borrow();
        instantiate_zserio_array(symbol_scope, type_generator, zserio_write_fn, &field, false);
        encode_field(symbol_scope, type_generator, zserio_write_fn, &field, None);
    }

    let zserio_write_packed_fn = struct_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    for field in fields {
        instantiate_zserio_array(
            symbol_scope,
            type_generator,
            zserio_write_packed_fn,
            &field.field.borrow(),
            true,
        );
        encode_field(
            symbol_scope,
            type_generator,
            zserio_write_packed_fn,
            &field.field.borrow(),
            Option::from(field.field_index as u8),
        );
    }
}

fn generate_zserio_bitsize(
    symbol_scope: &ModelScope,
    type_generator: &TypeGenerator,
    struct_impl: &mut codegen::Impl,
    fields: &Vec<FieldDetails>,
) {
    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    for field in fields {
        instantiate_zserio_array(
            symbol_scope,
            type_generator,
            bitsize_fn,
            &field.field.borrow(),
            false,
        );
        bitsize_field(
            symbol_scope,
            type_generator,
            bitsize_fn,
            &field.field.borrow(),
            None,
        );
    }
    bitsize_fn.line("end_position - bit_position");

    let bitsize_packed_fn = struct_impl.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("u64");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    for field in fields {
        instantiate_zserio_array(
            symbol_scope,
            type_generator,
            bitsize_packed_fn,
            &field.field.borrow(),
            true,
        );
        bitsize_field(
            symbol_scope,
            type_generator,
            bitsize_packed_fn,
            &field.field.borrow(),
            Option::from(field.field_index as u8),
        );
    }
    bitsize_packed_fn.line("end_position - bit_position");
}
