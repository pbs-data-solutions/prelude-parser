@_default:
  just --list

@develop:
  uv run maturin develop --uv -E all

@develop-release:
  uv run maturin develop --uv -E all -r

@install: && develop
  uv sync --frozen --all-extras

@install-release: && develop-release
  uv sync --frozen --all-extras

@lock:
  uv lock

@lint:
  echo cargo check
  just --justfile {{justfile()}} check
  echo cargo clippy
  just --justfile {{justfile()}} clippy
  echo cargo fmt
  just --justfile {{justfile()}} fmt
  echo ruff-check
  just --justfile {{justfile()}} ruff-check
  echo ruff-format
  just --justfile {{justfile()}} ruff-format
  echo pyrefly
  just --justfile {{justfile()}} pyrefly

@check:
  cargo check --workspace --all-targets --all-features

@clippy:
  cargo clippy --workspace --all-targets --all-features

@fmt:
  cargo fmt --all

@bench:
  cargo bench -p prelude-xml-parser

@bench-quick:
  cargo bench -p prelude-xml-parser --bench parse_benchmark -- --quick

@bench-smoke:
  cargo bench -p prelude-xml-parser -- --test

@rust-test *args="":
  cargo test -p prelude-xml-parser {{args}}

@rust-test-review:
  cargo insta test -p prelude-xml-parser --review

@pyrefly:
  uv run pyrefly check

@ruff-check:
  uv run ruff check prelude_parser tests

@ruff-format:
  uv run ruff format prelude_parser tests

@python-test *args="":
  uv run pytest {{args}}

@test:
  echo testing rust
  just --justfile {{justfile()}} rust-test
  echo testing python
  just --justfile {{justfile()}} python-test
