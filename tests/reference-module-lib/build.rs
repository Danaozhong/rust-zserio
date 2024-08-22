use rust_zserio::{generate_model, Model};

use std::path::Path;

// custom build script, which will run zserio code generation before building the library
fn main() {
    // Use rust-zserio to generate the new code
    let mut model = Model::from_filesystem(Path::new("../reference_modules"));
    model.evaluate();
    generate_model(&mut model, Path::new("src"), "");
}
