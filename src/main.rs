use crate::internal::generator::model::generate_model;
use crate::internal::model::Model;
use clap::Parser;
use std::path::Path;
pub mod internal;
pub mod ztype;

/// zserio generator for rust.
#[derive(Parser, Debug)]
#[command(version = None)]
struct Args {
    /// Directory where to generate the zserio interface files.
    #[arg(short, long)]
    out: String,

    /// Enforces a top-level crate path. Leave empty for no custom crate prefix.
    #[arg(short, long, default_value_t=String::from(""))]
    root: String,

    /// Input directory where the zserio files are located.
    zserio_path: String,
}

fn main() {
    let args = Args::parse();
    let mut model = Model::from_filesystem(Path::new(args.zserio_path.as_str()));
    model.evaluate();
    generate_model(&mut model, Path::new(args.out.as_str()), &args.root);
}
