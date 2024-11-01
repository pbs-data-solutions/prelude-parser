import sys
from pathlib import Path

from prelude_parser import __version__

if sys.version_info < (3, 11):
    import tomli as tomllib
else:
    import tomllib


def test_versions_match():
    pyproject = Path().absolute() / "Cargo.toml"
    with open(pyproject, "rb") as f:
        data = tomllib.load(f)
        cargo_version = data["package"]["version"]

    assert __version__ == cargo_version
