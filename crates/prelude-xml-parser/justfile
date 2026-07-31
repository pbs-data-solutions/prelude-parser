all:
  just --justfile {{justfile()}} fmt
  just --justfile {{justfile()}} check
  just --justfile {{justfile()}} clippy
  just --justfile {{justfile()}} test

lint:
  just --justfile {{justfile()}} fmt
  just --justfile {{justfile()}} check
  just --justfile {{justfile()}} clippy

bench:
  cargo bench

bench-quick:
  cargo bench --bench parse_benchmark -- --quick

bench-smoke:
  cargo bench -- --test

clippy:
  cargo clippy --all-targets --all-features

check:
  cargo check --all-targets --all-features

fmt:
  cargo fmt --all

test:
  cargo insta test

test-review:
  cargo insta test --review
