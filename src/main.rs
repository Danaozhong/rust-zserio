mod internal {
    mod parser;
    mod visitor;
    pub mod model;
    pub mod ast;
    pub mod generator;
}
mod ztype;
use clap::Parser;
use crate::internal::model::model::from_filesystem;
use crate::internal::generator::model::generate_model;
use std::path::Path;


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
    let model = from_filesystem(Path::new(args.zserio_path.as_str()));
    generate_model(&model, Path::new(args.out.as_str()), args.root.as_str());
}
