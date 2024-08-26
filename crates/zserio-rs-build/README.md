# zserio-rs-build

A [zserio](http://zserio.org/) compiler for Rust. This provides the
`zserio-rs-build` tool to generate Rust code for a zserio schema. The generated
code will use the [zserio](https://crates.io/crates/zserio) crate for runtime
support.

## Quick Start

Install zserio-rs-build using:

```sh
cargo install zserio-rs-build
```

## Generate zserio Bindings

The code generator is executed using the following command:

```sh
zserio-rs-build =<code_root_path> -o=<output_directory> <path_to_zserio_files>
```

This will generate the files needed to read/write `zserio`-encoded binary data.
The `root` CLI flag is optional, and specifies a crate prefix. If the prefix is
set, a `mod.rs` file will be created. In case the code is generated without a
prefix, it is assumed that the crate will be built as a library, and generates
a `lib.rs` instead. You must add `zserio` as a dependency for the crate
containing the generated code:

```sh
cargo add zserio
```

The version of the `zserio` crate used must match the version of `zserio-rs-build` used.
