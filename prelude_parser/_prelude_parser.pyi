from __future__ import annotations

from pathlib import Path

from prelude_parser._prelude_parser import (  # type: ignore[attr-defined]
    SiteNative,
    SubjectNative,
    UserNative,
)
from prelude_parser.types import FlatFormInfo

def _parse_flat_file_to_dict(
    xml_file: str | Path, *, short_names: bool = False
) -> dict[str, FlatFormInfo]: ...
def _parse_flat_file_to_pandas_dict(
    xml_file: str | Path, *, short_names: bool = False
) -> dict[str, FlatFormInfo]: ...
def parse_site_native_file(xml_file: str | Path) -> SiteNative: ...
def parse_site_native_string(xml_str: str) -> SiteNative: ...
def parse_subject_native_file(xml_file: str | Path) -> SubjectNative: ...
def parse_subject_native_string(xml_str: str) -> SubjectNative: ...
def parse_user_native_file(xml_file: str | Path) -> UserNative: ...
def parse_user_native_string(xml_str: str) -> UserNative: ...

class FileNotFoundError(Exception):
    pass

class InvalidFileTypeError(Exception):
    pass

class ParsingError(Exception):
    pass
