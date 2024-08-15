## NDS.Live

The main use case of `rust-zserio` is to allow using NDS.Live in Rust. Any
changes in `rust-zserio` should be tested with the full NDS.Live schema. This
schema is available for free after registration from the NDS Association using
the [NDS.Live package builder](https://pack.nds.live). Please note that NDS.Live
is not open source.

## Tests

`rust-zserio` comes with multiple sets of tests:

- unit tests in the main package. You can run this with `cargo test`.
- round trip tests. These test that serializing data to zserio and then
  deserializing it again results in the exact same data. You can run these
  by running `cargo test` in the `tests/round-trip-tests` folder.
- `tests-compare-ref-impl-tests` compares `rust-zserio` with the reference
  Python zserio implementation. These tests can currently not be run from
  this repository.

You can run all tests using `cargo test --all`

## Profiling

The easiest way to profile is to use
[samply](https://github.com/mstange/samply). You can use `cargo` to install it:

```shell
cargo install --locked samply
```

Next step is to compile `rust-zserio` with profiling information:

```shell
cargo build --profile profiling
```

you can now run `rust-zserio`:

```shell
samply record ./target/profiling/rust-zserio -o /tmp/sz tests/reference_modules
```

this will automatically open your browser with the profile loaded.