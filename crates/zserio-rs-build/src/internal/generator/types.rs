use crate::internal::ast::type_reference::TypeReference;
use stringcase::Caser;

pub struct TypeGenerator {
    pub root_package_name: String,
}

impl TypeGenerator {
    pub fn new(root_package_name: String) -> Self {
        Self { root_package_name }
    }

    pub fn zserio_package_to_rust_module(&mut self, package: &str) -> String {
        assert!(!package.is_empty(), "package type has not been resolved");

        let mut root_package = String::from("crate");
        if !self.root_package_name.is_empty() {
            root_package = format!("crate::{}", &self.root_package_name);
        }
        for module in package.split('.') {
            root_package = format!("{}::{}", root_package, Self::to_rust_module_name(module));
        }
        root_package
    }

    pub fn convert_field_name(name: &str) -> String {
        remove_reserved_identifier(name).to_snake_case_with_nums_as_word()
    }

    pub fn ztype_to_rust_type(&mut self, ztype: &TypeReference) -> String {
        match ztype.is_builtin {
            true => zserio_to_rust_type(&ztype.name)
                .unwrap_or_else(|_| panic!("type mapping failed {:?}", &ztype.name)),
            false => {
                assert!(!ztype.package.is_empty(), "package must be resolved");
                format!(
                    "{}::{}",
                    self.zserio_package_to_rust_module(&ztype.package),
                    Self::custom_type_to_rust_type(&ztype.name)
                )
            }
        }
    }

    pub fn get_full_module_path(&mut self, package: &str, rust_symbol_name: &str) -> String {
        format!(
            "{}::{}",
            self.zserio_package_to_rust_module(package),
            rust_symbol_name,
        )
    }

    pub fn to_rust_module_name(name: &str) -> String {
        remove_reserved_identifier(name).to_snake_case_with_nums_as_word()
    }

    pub fn to_rust_type_name(name: &str) -> String {
        remove_reserved_identifier(name).to_pascal_case()
    }

    pub fn custom_type_to_rust_type(name: &str) -> String {
        format!(
            "{}::{}",
            Self::to_rust_module_name(name),
            Self::to_rust_type_name(name)
        )
    }

    pub fn constant_type_to_rust_type(name: &str) -> String {
        format!(
            "{}::{}",
            Self::to_rust_module_name(name),
            to_rust_constant_name(name)
        )
    }
}

#[inline]
fn remove_reserved_identifier(name: &str) -> &str {
    match name.to_ascii_lowercase().as_str() {
        "type" => "z_type",
        "struct" => "z_struct",
        "self" => "z_self",
        _ => name,
    }
}

/// Translates a zserio name to a rust constant name.
pub fn to_rust_constant_name(name: &str) -> String {
    remove_reserved_identifier(name).to_ascii_uppercase()
}

pub fn convert_to_enum_field_name(name: &str) -> String {
    remove_reserved_identifier(name).to_pascal_case()
}

pub fn convert_to_union_selector_name(field_name: &str) -> String {
    remove_reserved_identifier(field_name).to_pascal_case()
}

pub fn convert_to_function_name(name: &str) -> String {
    // Converts a function name from zserio style to rust style (snake case).
    remove_reserved_identifier(name).to_snake_case_with_nums_as_word()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rust_constant_name() {
        assert_eq!(to_rust_constant_name("NAME"), "NAME");
        assert_eq!(to_rust_constant_name("MIN_VERSION"), "MIN_VERSION");
    }

    #[test]
    fn test_convert_to_enum_field_name() {
        assert_eq!(convert_to_enum_field_name("UNKNOWN"), "Unknown");
        assert_eq!(
            convert_to_enum_field_name("MORE_THAN_4_TOWARDS_CURB"),
            "MoreThan4TowardsCurb"
        );
    }

    #[test]
    fn test_convert_to_union_selector_name() {
        assert_eq!(convert_to_enum_field_name("UNKNOWN"), "Unknown");
        assert_eq!(
            convert_to_enum_field_name("MORE_THAN_4_TOWARDS_CURB"),
            "MoreThan4TowardsCurb"
        );
    }

    #[test]
    fn test_convert_to_function_name() {
        assert_eq!(convert_to_function_name("getLayerType"), "get_layer_type");
        assert_eq!(
            convert_to_function_name("getSomeRandomValue"),
            "get_some_random_value"
        );
    }

    #[test]
    fn test_convert_field_name() {
        assert_eq!(TypeGenerator::convert_field_name("simple"), "simple");
        assert_eq!(TypeGenerator::convert_field_name("numItems"), "num_items");
        assert_eq!(TypeGenerator::convert_field_name("boValue1"), "bo_value_1");
        assert_eq!(
            TypeGenerator::convert_field_name("positions2D"),
            "positions_2_d"
        );
    }
}
