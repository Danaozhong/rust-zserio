## NDS.Live

The main use case of `rust-zserio` is to allow using NDS.Live in Rust. Any
changes in `rust-zserio` should be tested with the full NDS.Live schema. This
schema is available for free after registration from the NDS Association using
the [NDS.Live package builder](https://pack.nds.live). Please note that NDS.Live
is not open source.

## Tests

`rust-zserio` comes with multiple sets of tests:

- unit tests in the runtime code. You can run this with `cargo test -p zserio`.
- unit tests for the compiler. You can run this with `cargo test -p zserio-cli`.
- round trip tests. These test that serializing data to zserio and then
  deserializing it again results in the exact same data. You can run these
  by running `cargo run -p round-trip-tests`.
- `tests-compare-ref-impl-tests` compares `rust-zserio` with the reference
  Python zserio implementation.

### Verifying against zserio reference implementation

To run the reference comparison test you must install the zserio compiler
The simplest way to do this is using [uv](https://docs.astral.sh/uv/)

```sh
uv tool install zserio==2.11.0
```

You can now generate the test data using the Python code. _Please note that
Python 3.12 is currently not supported._

```sh
cd tests/compare-ref-impl-tests/python
poetry install
poetry run python main.py
```

The test data can now be found in the `tests/artifacts` folder. Finally you can
now run the Rust code to verify the rust behavior matches the Python behavior:

```sh
cargo run -p compare-ref-impl-tests
```

## Profiling

The easiest way to profile is to use
[samply](https://github.com/mstange/samply). You can use `cargo` to install it:

```sh
cargo install --locked samply
```

Next step is to compile `rust-zserio` with profiling information:

```sh
cargo build --profile profiling
```

you can now run `rust-zserio`:

```sh
samply record ./target/profiling/rust-zserio -o /tmp/sz tests/reference_modules
```

this will automatically open your browser with the profile loaded.
