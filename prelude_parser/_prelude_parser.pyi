from __future__ import annotations

from pathlib import Path
from typing import Any

def _parse_flat_file(xml_file: str | Path) -> dict[str, list[Any]]: ...

class FileNotFoundError(Exception):
    pass

class InvalidFileTypeError(Exception):
    pass

class ParsingError(Exception):
    pass
