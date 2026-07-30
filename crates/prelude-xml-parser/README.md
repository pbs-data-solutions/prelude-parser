# Prelude XML Parser

[![Tests Status](https://github.com/pbs-data-solutions/prelude-xml-parser/actions/workflows/testing.yml/badge.svg?branch=main&event=push)](https://github.com/pbs-data-solutions/prelude-xml-parser/actions?query=workflow%3ATesting+branch%3Amain+event%3Apush)
![crates.io](https://img.shields.io/crates/v/prelude-xml-parser.svg?color=brightgreen)

Deserialize Prelude EDC native XML files into Rust structs. Enabling the `python` feature allows
deserializing to Python classes with PyO3.

## Supported native files

- [x] Subject native XML
- [x] Site Native XML
- [x] User Native XML

## Benchmarks

Benchmarks are run with [criterion](https://github.com/bheisler/criterion.rs) and cover all three
parsers, in both their `_string` and `_file` forms, across a range of input sizes. Inputs are
generated at run time by replicating the fixtures in `tests/assets/`, so no extra setup is needed:

```sh
just bench
```

An HTML report is written to `target/criterion/report/index.html`.

For a faster measurement that stops as soon as the significance level is reached (~20s instead of
several minutes):

```sh
just bench-quick
```

To only check that the benchmarks still run, without measuring anything:

```sh
just bench-smoke
```

To additionally benchmark against a real subject native export, point `BENCH_XML_FILE` at it:

```sh
BENCH_XML_FILE=/path/to/subject_native.xml cargo bench
```
