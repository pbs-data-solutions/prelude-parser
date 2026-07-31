# Prelude XML Parser

[![Tests Status](https://github.com/pbs-data-solutions/prelude-parser/actions/workflows/testing.yml/badge.svg?branch=main&event=push)](https://github.com/pbs-data-solutions/prelude-parser/actions?query=workflow%3ATesting+branch%3Amain+event%3Apush)
![crates.io](https://img.shields.io/crates/v/prelude-xml-parser.svg?color=brightgreen)

Deserialize Prelude EDC native XML files into Rust structs. Enabling the `python` feature allows
deserializing to Python classes with PyO3.

## Installation

```sh
cargo add prelude-xml-parser
```

## Supported native files

- [x] Subject native XML
- [x] Site Native XML
- [x] User Native XML

## Development

This crate lives in the [prelude-parser](https://github.com/pbs-data-solutions/prelude-parser)
repository, alongside the Python bindings that build on it. It is published to crates.io on its own
version series and can be used as a standalone Rust dependency. See the repository's
[contributing guide](https://github.com/pbs-data-solutions/prelude-parser/blob/main/CONTRIBUTING.md)
for how to build, test, and benchmark it.
