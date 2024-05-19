use crate::internal::ast::type_reference::TypeReference;
use convert_case::{Case, Casing};
use std::{collections::HashMap, result::Result};

const RESERVED_RUST_KEYWORDS: &[&str] = &["type", "struct", "self"];

pub struct TypeGenerator {
    pub root_package_name: String,

    // rust is super-slow in converting strings (e.g.
    // upper/lower/camel/snake case), we keep track of
    // all conversions already done, and simply reuse them
    package_convert_cache: HashMap<String, String>,
    field_name_cache: HashMap<String, String>,
    module_name_cache: HashMap<String, String>,
    type_name_cache: HashMap<String, String>,
}

impl TypeGenerator {
    pub fn new(root_package_name: String) -> Self {
        Self {
            root_package_name,
            package_convert_cache: HashMap::new(),
            field_name_cache: HashMap::new(),
            module_name_cache: HashMap::new(),
            type_name_cache: HashMap::new(),
        }
    }
    pub fn zserio_package_to_rust_module(&mut self, package: &str) -> String {
        assert!(!package.is_empty(), "package type has not been resolved");

        // Try to read from cache, if it already exists
        if let Some(converted_package) = self.package_convert_cache.get(package) {
            return converted_package.clone();
        }

        let mut root_package = String::from("crate");
        if !self.root_package_name.is_empty() {
            root_package = format!("crate::{}", &self.root_package_name);
        }
        for module in package.split('.') {
            root_package = format!("{}::{}", root_package, self.to_rust_module_name(module));
        }
        // Keep the converted string in cache
        self.package_convert_cache
            .insert(package.to_owned(), root_package.clone());
        root_package
    }

    pub fn convert_field_name(&mut self, name: &str) -> String {
        if let Some(converted_field_name) = self.field_name_cache.get(name) {
            return converted_field_name.clone();
        }
        let converted_field_name = remove_reserved_identifier(name).to_case(Case::Snake);
        self.field_name_cache
            .insert(name.to_owned(), converted_field_name.clone());
        converted_field_name
    }

    pub fn ztype_to_rust_type(&mut self, ztype: &TypeReference) -> String {
        if ztype.is_builtin {
            // the type is a zserio built-in type, such as int32, string, bool
            return zserio_to_rust_type(&ztype.name)
                .unwrap_or_else(|_| panic!("type mapping failed {:?}", &ztype.name));
        }
        // the type is a custom type, defined in some zserio file.
        assert!(!ztype.package.is_empty(), "package must be resolved");
        format!(
            "{}::{}",
            self.zserio_package_to_rust_module(&ztype.package),
            self.custom_type_to_rust_type(&ztype.name)
        )
    }

    pub fn get_full_module_path(&mut self, package: &str, rust_symbol_name: &str) -> String {
        format!(
            "{}::{}",
            self.zserio_package_to_rust_module(package),
            rust_symbol_name,
        )
    }

    pub fn to_rust_module_name(&mut self, name: &str) -> String {
        // Try to read from cache, if it already exists
        if let Some(converted_module_name) = self.module_name_cache.get(name) {
            return converted_module_name.clone();
        }

        let rust_module_name = remove_reserved_identifier(name).to_case(Case::Snake);
        self.module_name_cache
            .insert(name.to_owned(), rust_module_name.clone());
        rust_module_name
    }

    pub fn to_rust_type_name(&mut self, name: &str) -> String {
        if let Some(converted_rust_type_name) = self.type_name_cache.get(name) {
            return converted_rust_type_name.clone();
        }
        let rust_type_name = remove_reserved_identifier(name).to_case(Case::UpperCamel);
        self.type_name_cache
            .insert(name.to_owned(), rust_type_name.clone());
        rust_type_name
    }

    pub fn custom_type_to_rust_type(&mut self, name: &str) -> String {
        format!(
            "{}::{}",
            self.to_rust_module_name(name),
            self.to_rust_type_name(name)
        )
    }

    pub fn constant_type_to_rust_type(&mut self, name: &str) -> String {
        format!(
            "{}::{}",
            self.to_rust_module_name(name),
            to_rust_constant_name(name)
        )
    }
}

pub fn remove_reserved_identifier(name: &str) -> String {
    if RESERVED_RUST_KEYWORDS.contains(&name.to_lowercase().as_str()) {
        return format!("z_{}", name);
    }
    name.into()
}

/// Translates a zserio name to a rust constant name.
pub fn to_rust_constant_name(name: &str) -> String {
    remove_reserved_identifier(name).to_ascii_uppercase()
}

pub fn convert_to_enum_field_name(name: &str) -> String {
    remove_reserved_identifier(name).to_case(Case::UpperCamel)
}

pub fn convert_to_union_selector_name(field_name: &str) -> String {
    remove_reserved_identifier(field_name).to_case(Case::UpperCamel)
}

pub fn convert_to_function_name(name: &str) -> String {
    // Converts a function name from zserio style to rust style (snake case).
    remove_reserved_identifier(name).to_case(Case::Snake)
}

pub fn zserio_to_rust_type(name: &str) -> Result<String, &'static str> {
    match name {
        "int8" => Ok("i8".into()),
        "int16" => Ok("i16".into()),
        "int32" => Ok("i32".into()),
        "int64" => Ok("i64".into()),
        "varint16" => Ok("i16".into()),
        "varint32" => Ok("i32".into()),
        "varint64" => Ok("i64".into()),
        "varint" => Ok("i64".into()),
        "uint8" => Ok("u8".into()),
        "uint16" => Ok("u16".into()),
        "uint32" => Ok("u32".into()),
        "uint64" => Ok("u64".into()),
        "varuint16" => Ok("u16".into()),
        "varuint32" => Ok("u32".into()),
        "varuint64" => Ok("u64".into()),
        "varsize" => Ok("u32".into()),
        "varuint" => Ok("u64".into()),
        "string" => Ok("String".into()),
        "float16" => Ok("f32".into()),
        "float32" => Ok("f32".into()),
        "float64" => Ok("f64".into()),
        "bool" => Ok("bool".into()),
        "bit" => Ok("u64".into()),
        "int" => Ok("i64".into()),
        "extern" => Ok("ztype::ExternType".into()),
        "bytes" => Ok("ztype::BytesType".into()),
        _ => Err("not found"),
    }
}

pub fn zserio_type_bit_size(name: &str) -> Result<u8, &'static str> {
    match name {
        "int8" => Ok(8),
        "int16" => Ok(16),
        "int32" => Ok(32),
        "int64" => Ok(64),
        "varint16" => Ok(16),
        "varint32" => Ok(32),
        "varint64" => Ok(64),
        "varint" => Ok(64),
        "uint8" => Ok(8),
        "uint16" => Ok(16),
        "uint32" => Ok(32),
        "varuint16" => Ok(16),
        "varuint32" => Ok(32),
        "varuint64" => Ok(64),
        "varuint" => Ok(64),
        "varsize" => Ok(32),
        "float16" => Ok(16),
        "float32" => Ok(32),
        "float64" => Ok(64),
        "bool" => Ok(1),
        _ => Err("not found"),
    }
}
