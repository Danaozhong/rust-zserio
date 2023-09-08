# rust-zserio

[zserio](http://zserio.org/) serialization bindings for Rust.

## CI Status
![build](https://github.com/Danaozhong/rust-zserio/actions/workflows/test.yml/badge.svg)

## How to Run

You can run the code generator using the following command:

```sh
cargo run -- --root=<code_root_path> -o=<output_directory> <path_to_zserio_files>
```

This will generate the files needed to read/write `zserio`-encoded binary data.

## Development Status

Although the project is in its early stages, it is feature-complete and should work with most `zserio` applications. Successful test runs were done with sample projects, that were compared against the Python reference implementation. Nevertheless, the library does not have many users yet, and test coverage is small. If you find issues, please report them.

### Open Items
- constraints are not evaluated yet.
- proper unit test / e2e test setup.
- compare with the reference Python/C++ implementation.
- the generated code fails the lint checks.
- clean and structured error handling and logging.
- upgrade to a non-beta ANTLR parser.
- resolve all clippy warnings.
- performance evaluation and improvements.
- generate a `mod.rs` file.

## Disclaimer

This is my very first rust project written in my spare time, so the code is far from perfect. I am happy to receive any suggestions and to learn on how to write better rust code, so please raise a PR or an issue if you have suggestions!

