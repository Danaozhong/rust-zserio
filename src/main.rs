#![feature(try_blocks)]

mod internal {
    mod parser;
    mod visitor;
    pub mod model;
    pub mod ast;
    pub mod generator;


}
mod ztype;

use crate::internal::model::model::from_filesystem;
use crate::internal::generator::model::generate_model;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let model = from_filesystem(Path::new("test/reference_modules"));
    generate_model(&model, Path::new("H:\\rust_test\\src"), "test123");
}
