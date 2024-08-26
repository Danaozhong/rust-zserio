//! # rust-zserio is obsolete
//! This package is obsolete, and should not be used. Please use the
//! [zserio-rs-build](https://crates.io/crates/zserio-rs-build) package for the
//! Rust zserio compiler, and [zserio](https://crates.io/crates/zserio) for the
//! runtime support code.

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
