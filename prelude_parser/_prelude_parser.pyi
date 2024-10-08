from __future__ import annotations

from datetime import datetime
from pathlib import Path

from prelude_parser.types import FlatFormInfo

class Value:
    by: str
    by_unique_id: str | None
    role: str
    when: datetime

class Reason:
    by: str
    by_unique_id: str | None
    role: str
    when: datetime
    value: str

class Entry:
    entry_id: str
    value: Value | None
    reason: Reason | None

class Field:
    name: str
    data_type: str | None
    error_code: str
    when_created: datetime
    keep_history: bool
    entries: list[Entry] | None

class Category:
    name: str
    category_type: str
    highest_index: int
    fields: list[Field] | None

class State:
    value: str
    signer: str
    signer_unique_id: str

class Form:
    name: str
    last_modified: datetime | None
    who_last_modified: str | None
    who_last_modified_role: str | None
    when_created: int
    has_errors: bool
    has_warnings: bool
    locked: bool
    user: str | None
    date_time_changed: datetime | None
    form_title: str
    form_index: int
    form_group: str | None
    form_state: str
    states: list[State] | None
    categories: list[Category] | None

class Patient:
    patient_id: str
    unique_id: str
    when_created: datetime
    creator: str
    site_name: str
    site_unique_id: str
    last_language: str | None
    forms: list[Form] | None

class Site:
    name: str
    unique_id: str
    number_of_patients: int
    count_of_randomized_patients: int
    when_created: datetime
    creator: str
    number_of_forms: int
    forms: list[Form] | None

class User:
    unique_id: str
    last_language: str | None
    creator: str
    number_of_forms: int
    forms: list[Form] | None

class SiteNative:
    sites: list[Site]

class SubjectNative:
    patients: list[Patient]

class UserNative:
    users: list[User]

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
