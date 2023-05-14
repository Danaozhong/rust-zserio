use rust_zserio::internal::generator::model::generate_model;
use rust_zserio::internal::model::model;

use std::path::Path;

// custom build script, which will run zserio code generation before starting the program
fn main() {
    // Use rust-zserio to generate the new code
    let model = model::from_filesystem(Path::new("../reference_modules"));
    generate_model(&model, &Path::new("src"), "dummy");
}
