# rust-zserio

[zserio](http://zserio.org/) serialization bindings for Rust.

## CI Status
![build](https://github.com/Danaozhong/rust-zserio/actions/workflows/test.yml/badge.svg)

## Development Status

This project is still under construction, and not ready for use yet. A large part of the zserio syntax has been implemented, and zserio files can be successfully parsed and evaluated.
Code generation also works, but the generated code may not always compile.
The following open items still need to be addressed:

### Open Items
- passing parameters between zserio compound objects.
- index operator implementation.
- lengthof operator implementation.
- extern/bytes types are not implemented yet.
- several array traits are not implemented yet, and the array traits code has a lot of c&p.
- proper unit test / e2e test setup.
- the generated code fails the lint checks, mainly due to unnecessary includes. The include sections somehow needs to be cleaned up, and unused includes removed.
- clean and structured error handling and logging.
- the zserio union type still needs to be implemented.
- compare the serialized binary code and compare with the reference implementation.
- upgrade to a non-beta ANTLR parser.
- resolve all clippy warnings.

## Disclaimer

This is my very first rust project written in my spare time, so the code is far from perfect. I am happy to receive any suggestions and to learn on how to write better rust code, so please raise a PR or an issue if you have suggestions!

