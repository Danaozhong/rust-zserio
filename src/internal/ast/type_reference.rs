
use std::string::String;

#[derive(Clone)]
pub struct TypeReference {
    pub is_builtin: bool,
    pub package: String,
	pub name: String,
    pub bits: i8,
}
