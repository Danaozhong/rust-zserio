use rust_zserio::internal::generator::model::generate_model;
use rust_zserio::internal::model::Model;

use std::path::Path;

// custom build script, which will run zserio code generation before starting the program
fn main() {
    // Use rust-zserio to generate the new code
    let mut model = Model::from_filesystem(Path::new("../reference_modules"));
    model.evaluate();
    generate_model(&mut model, Path::new("src"), &String::new());
}
