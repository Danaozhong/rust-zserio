use clap::Parser;
use log::LevelFilter;
use rust_zserio::internal::generator::model::generate_model;
use rust_zserio::internal::model::Model;
use simple_logger::SimpleLogger;
use std::path::Path;

/// zserio generator for rust.
#[derive(Parser, Debug)]
#[command(version = None)]
struct Args {
    /// Output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

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

    SimpleLogger::new()
        .with_level(match args.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            3.. => LevelFilter::Trace,
        })
        .init()
        .expect("can not initialize logger");

    // Load the zserio files (*.zs) from the filesystem.
    let mut model = Model::from_filesystem(Path::new(args.zserio_path.as_str()));

    // Ensure that the model is correct, by evaluating templates, types, and expressions.
    model.evaluate();

    // Generate the rust files.
    generate_model(&mut model, Path::new(args.out.as_str()), &args.root);
}
