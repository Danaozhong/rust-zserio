use cargo_util::ProcessBuilder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if ProcessBuilder::new("zserio-cli")
        .args_replace(&args)
        .exec_replace()
        .is_err()
    {
        eprintln!("Can not run zserio-cli. Please make sure you have installed it.");
        eprintln!();
        eprintln!("    cargo install zserio-cli");
    }
}
