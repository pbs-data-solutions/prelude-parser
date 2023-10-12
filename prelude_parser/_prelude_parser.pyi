from __future__ import annotations

from pathlib import Path

from prelude_parser.types import FlatFormInfo

def _parse_flat_file_to_dict(
    xml_file: str | Path, short_names: bool
) -> dict[str, FlatFormInfo]: ...
def _parse_flat_file_to_pandas_dict(
    xml_file: str | Path, short_names: bool
) -> dict[str, FlatFormInfo]: ...

class FileNotFoundError(Exception):
    pass

class InvalidFileTypeError(Exception):
    pass

class ParsingError(Exception):
    pass
