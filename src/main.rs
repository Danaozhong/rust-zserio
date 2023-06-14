use crate::internal::generator::model::generate_model;
use crate::internal::model::Model;
use clap::Parser;
use std::path::Path;
pub mod internal;
pub mod ztype;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version = None)]
struct Args {
    /// directory where to generate the files
    #[arg(short, long)]
    out: String,

    /// the root package name to generate the rust library with
    #[arg(short, long, default_value_t=String::from("test"))]
    root: String,

    /// Input directory where the zserio files are
    zserio_path: String,
}

fn main() {
    let args = Args::parse();
    let mut model = Model::from_filesystem(Path::new(args.zserio_path.as_str()));
    model.evaluate();
    generate_model(&model, Path::new(args.out.as_str()), args.root.as_str());
}
