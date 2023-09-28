# rust-zserio

[zserio](http://zserio.org/) serialization bindings for Rust.

## CI Status
![build](https://github.com/Danaozhong/rust-zserio/actions/workflows/test.yml/badge.svg)

## Quick Start

Install rust-zserio using:

```sh
cargo install rust-zserio
```

## Generate zserio Bindings

The code generator is executed using the following command:

```sh
rust-zserio --root=<code_root_path> -o=<output_directory> <path_to_zserio_files>
```

This will generate the files needed to read/write `zserio`-encoded binary data. The `root` CLI flag is optional, and specifies a crate prefix. If the prefix is set, rust-zserio will generate a `mod.rs` file. In case the code is generated without a prefix, rust-zserio assumes that the crate will be built as a library, and generates a `lib.rs` instead.

## Development Status

Although the project is in its early stages, it is feature-complete and should work with most `zserio` applications. Successful test runs were done with sample projects, that were compared against the Python reference implementation. Nevertheless, the library does not have many users yet, and test coverage is small. If you find issues, please report them.

### Open Items
- constraints are not evaluated yet.
- proper unit test / e2e test setup.
- the generated code fails the lint checks.
- clean and structured error handling and logging.
- upgrade to a non-beta ANTLR parser.
- resolve all clippy warnings.
- performance evaluation and improvements.

## Disclaimer

This is my very first Rust project written in my spare time, and the code is far from perfect. I am happy to receive any suggestions and to learn on how to write better Rust code, so please feel welcome to raise a PR!

