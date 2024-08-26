use clap::{Parser, ValueHint};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::path::Path;
use zserio_rs_build::{generate_model, Model};

#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    /// Output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Directory where to generate the zserio interface files.
    #[arg(short, long, value_name="PATH", value_hint=ValueHint::DirPath)]
    out: String,

    /// Enforces a top-level crate path. Leave empty for no custom crate prefix.
    #[arg(short, long, value_name="CRATE_PATH", default_value_t=String::from(""))]
    root: String,

    /// Input directory where the zserio files are located.
    #[arg(value_name="PATH", value_hint=ValueHint::DirPath)]
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
